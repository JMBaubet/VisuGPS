import { invoke } from '@tauri-apps/api/core';
import * as turf from '@turf/turf';

/**
 * Service responsible for mapping positions and distances between
 * the Main Trace and a Variant.
 */
class TraceMappingService {
    constructor() {
        this.cache = new Map(); // variantId -> mapping
    }

    /**
     * Loads and processes the mapping for a specific variant.
     * @param {string} circuitId 
     * @param {string} variantId 
     * @param {Array} mainTrackingPoints - The full tracking points of the main trace (for exact Km/Index reference)
     */
    async loadVariantMapping(circuitId, variantId, mainTrackingPoints) {
        if (this.cache.has(variantId)) {
            return this.cache.get(variantId);
        }

        console.log(`[TraceMappingService] Loading mapping for ${variantId} with ${mainTrackingPoints?.length} ref points`);

        try {
            // 1. Get the comparison GeoJSON from Backend
            // This returns features with properties.status = 'COMMON' | 'NEW' | 'ABANDONED'
            const geojsonStr = await invoke('get_variant_comparison_geojson', { circuitId, variantId });
            const geojson = JSON.parse(geojsonStr);
            console.log(`[TraceMappingService] GeoJSON features:`, geojson.features.length);

            // 2. Process features to build the continuous variant timeline
            const mapping = this._buildMapping(geojson, mainTrackingPoints);
            console.log(`[TraceMappingService] Mapping built:`, mapping);

            this.cache.set(variantId, mapping);
            return mapping;
        } catch (e) {
            console.error("Failed to load variant mapping:", e);
            return null;
        }
    }

    /**
     * Internal method to build the segment list.
     */
    _buildMapping(geojson, mainTrackingPoints) {
        let variantCumulDist = 0;
        const segments = [];

        // The GeoJSON features are already ordered by the backend (normally).
        // If not, we might need to rely on connectivity, but let's assume valid order for now or rely on backend sort.
        // Backend 'get_variant_comparison_geojson' sorts by anchor index, so it should be mostly linear.

        for (const feature of geojson.features) {
            const status = feature.properties.status;
            const coords = feature.geometry.coordinates;

            if (status === 'ABANDONED') {
                continue;
            }

            // Calculate length of this segment
            const segmentLengthKm = turf.length(feature, { units: 'kilometers' });

            const segment = {
                type: status,
                variantStartKm: variantCumulDist,
                variantEndKm: variantCumulDist + segmentLengthKm,
                variantLength: segmentLengthKm,
                // geometry: feature.geometry 
            };

            if (status === 'COMMON') {
                // Find where this common segment fits on the Main Trace
                // We match the Start and End coordinates to the Main Tracking Points
                const startPt = coords[0];
                const endPt = coords[coords.length - 1];

                const mainStart = this._findClosestPoint(startPt, mainTrackingPoints);
                const mainEnd = this._findClosestPoint(endPt, mainTrackingPoints);

                if (mainStart && mainEnd) {
                    segment.mainStartKm = mainStart.distance; // Already in Km
                    segment.mainEndKm = mainEnd.distance;     // Already in Km
                }
            }

            segments.push(segment);
            variantCumulDist += segmentLengthKm;
        }

        console.log(`[TraceMappingService] Built ${segments.length} segments. Common count: ${segments.filter(s => s.type === 'COMMON').length}`);
        // console.log("Segments:", JSON.stringify(segments, null, 2));

        return {
            totalDistance: variantCumulDist,
            segments: segments
        };
    }

    _findClosestPoint(coord, trackingPoints) {
        if (!trackingPoints || trackingPoints.length === 0) return null;

        // Simple closest point search. 
        // Optimized: Could restrict search window if we had context, but full scan is okay for <10k points usually.
        let minDist = Infinity;
        let bestPt = null;

        const [lon, lat] = coord;

        // Naive scan
        for (const pt of trackingPoints) {
            // coords in tracking might be [lon, lat]
            const ptLon = pt.coordonnee[0];
            const ptLat = pt.coordonnee[1];

            // Euclidean distance squared (sufficient for finding closest point locally)
            const d = (ptLon - lon) ** 2 + (ptLat - lat) ** 2;
            if (d < minDist) {
                minDist = d;
                bestPt = pt;
            }
        }

        // Threshold: 0.0001 degrees is roughly 11 meters. 
        // If the point is further than that, we might have a drift or mismatch.
        // Let's relax it slightly to 0.0005 (~55m) to be safe with simplifications.
        if (minDist < 0.000001) {
            return bestPt;
        } else {
            // RETURNING NULL HERE MIGHT BE THE ISSUE IF DISTANCE IS > 1m (0.00001^2 = 0.00000001)
            // 0.0001^2 = 0.00000001. My check was 0.000001 (1e-6). 
            // 1e-6 is sqrt(1e-6)=0.001 deg (~111m). That should be plenty.
        }
        return null;
    }

    /**
     * Converts a distance on the Main Trace to the real distance on the Variant.
     * @param {number} currentMainKm 
     * @param {Object} mapping 
     * @returns {number|null} Real variant Km, or null if off-track (on a deviation).
     */
    getRealDistanceFromMain(currentMainKm, mapping) {
        if (!mapping || !mapping.segments) return currentMainKm; // Fallback

        // Find which 'COMMON' segment contains currentMainKm
        const commonSeg = mapping.segments.find(s =>
            s.type === 'COMMON' &&
            currentMainKm >= (s.mainStartKm - 0.05) && // Increased tolerance to 50m
            currentMainKm <= (s.mainEndKm + 0.05)
        );

        if (commonSeg) {
            // Calculate offset within the segment
            const offset = currentMainKm - commonSeg.mainStartKm;
            return commonSeg.variantStartKm + offset;
        }

        // If not found in a COMMON segment, it means the Main Trace point being viewed
        // corresponds to a section that does NOT exist in the variant (it was 'ABANDONED').
        // So the group is 'OFF_TRACK' relative to this visual position.
        return null;
    }

    /**
     * Returns true if the group is currently on a specific variant segment (NEW) 
     * that is NOT on the main trace.
     */
    isOffTrack(currentMainKm, mapping) {
        return this.getRealDistanceFromMain(currentMainKm, mapping) === null;
    }

    /**
     * Converts a distance on the Variant to the corresponding distance on the Main Trace.
     * @param {number} currentVariantKm 
     * @param {Object} mapping 
     * @returns {number|null} corresponding Main Trace Km, or null if on a NEW segment (not on Main Trace).
     */
    getMainDistanceFromVariant(currentVariantKm, mapping) {
        if (!mapping || !mapping.segments) return currentVariantKm;

        // Find which 'COMMON' segment contains currentVariantKm
        const commonSeg = mapping.segments.find(s =>
            s.type === 'COMMON' &&
            currentVariantKm >= (s.variantStartKm - 0.005) &&
            currentVariantKm <= (s.variantEndKm + 0.005)
        );

        if (commonSeg) {
            const offset = currentVariantKm - commonSeg.variantStartKm;
            return commonSeg.mainStartKm + offset;
        }

        return null;
    }
}

export default new TraceMappingService();
