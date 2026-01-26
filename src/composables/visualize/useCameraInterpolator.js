import { ref } from 'vue';
import * as turf from '@turf/turf';

export function useCameraInterpolator(map) {

    // Helpers Math (Dupliqués ici ou importés d'un utils partagé si existant)
    const lerp = (start, end, amt) => (1 - amt) * start + amt * end;
    const lerpAngle = (start, end, amt) => {
        const da = (end - start) % 360;
        const shortestAngle = (2 * da % 360) - da;
        return start + shortestAngle * amt;
    };

    /**
     * Calcule et applique la position de la caméra pour une distance donnée.
     * @param {number} distanceTraveled - Distance parcourue en km
     * @param {Array} trackingPoints - Liste des points avec distance
     * @param {Array} controlPointIndices - Indices des points de contrôle
     * @param {Object} options - Options { dynamicZoomCoefficient, lineStringRef, isMultisegment, activeVariantSegments, apply }
     * @returns {{ bearing: number|null, target: Object|null }} Le bearing calculé de la trace et l'objet cible de la caméra.
     */
    const updateCameraPosition = (distanceTraveled, trackingPoints, controlPointIndices, options = {}) => {
        if (!map.value || !trackingPoints || trackingPoints.length < 2) return { bearing: null, target: null };

        const {
            dynamicZoomIntensity = 10,
            currentSpeed = 1.0,
            lineStringRef,
            isMultisegment,
            activeVariantSegments,
            apply = true // New option, default true
        } = options;

        let prevCamKeyframe, nextCamKeyframe;
        let constructedBearing = null;
        let target = null; // Will hold { center, zoom, pitch, bearing }

        // 1. Recherche du dernier point de contrôle passé
        let lastPassedControlPointIndex = -1;
        // Optimisation possible : ne pas parcourir tout le tableau à chaque frame si on garde l'index précédent
        // Mais pour l'instant on garde la logique robuste de la vue
        for (let i = controlPointIndices.length - 1; i >= 0; i--) {
            const cpIndex = controlPointIndices[i];
            const point = trackingPoints[cpIndex];
            if (point && point.distance <= distanceTraveled) {
                lastPassedControlPointIndex = cpIndex;
                break;
            }
        }

        if (lastPassedControlPointIndex !== -1) {
            const controlPoint = trackingPoints[lastPassedControlPointIndex];
            if (controlPoint.nbrSegment > 0) {
                const nextCpIndex = lastPassedControlPointIndex + controlPoint.nbrSegment;
                if (nextCpIndex < trackingPoints.length) {
                    prevCamKeyframe = controlPoint;
                    nextCamKeyframe = trackingPoints[nextCpIndex];
                    // console.log(`[DEBUG] Camera Spline: ${lastPassedControlPointIndex} -> ${nextCpIndex} (Dist: ${controlPoint.distance.toFixed(1)} -> ${nextCamKeyframe.distance.toFixed(1)})`);
                }
            }
        }

        // 2. Interpolation entre points de contrôle (Keyframes)
        if (prevCamKeyframe && nextCamKeyframe && distanceTraveled < nextCamKeyframe.distance) {
            const prevKeyframeDist = prevCamKeyframe.distance;
            const nextKeyframeDist = nextCamKeyframe.distance;
            const segmentDist = nextKeyframeDist - prevKeyframeDist;
            const progressInSegment = segmentDist > 0 ? (distanceTraveled - prevKeyframeDist) / segmentDist : 0;

            const lookAtPointLng = lerp(prevCamKeyframe.coordonnee[0], nextCamKeyframe.coordonnee[0], progressInSegment);
            const lookAtPointLat = lerp(prevCamKeyframe.coordonnee[1], nextCamKeyframe.coordonnee[1], progressInSegment);

            const prevZoom = prevCamKeyframe.editedZoom ?? prevCamKeyframe.zoom;
            const nextZoom = nextCamKeyframe.editedZoom ?? nextCamKeyframe.zoom;
            const prevPitch = prevCamKeyframe.editedPitch ?? prevCamKeyframe.pitch;
            const nextPitch = nextCamKeyframe.editedPitch ?? nextCamKeyframe.pitch;
            const prevCap = prevCamKeyframe.editedCap ?? prevCamKeyframe.cap;
            const nextCap = nextCamKeyframe.editedCap ?? nextCamKeyframe.cap;

            let zoom = lerp(prevZoom, nextZoom, progressInSegment);
            // Dynamic Zoom: Adjust based on speed (Slower = Zoom In, Faster = Zoom Out)
            zoom += (1 - currentSpeed) * (dynamicZoomIntensity / 50);

            const pitch = lerp(prevPitch, nextPitch, progressInSegment);
            const bearing = lerpAngle(prevCap, nextCap, progressInSegment);

            target = {
                center: [lookAtPointLng, lookAtPointLat],
                zoom,
                pitch,
                bearing
            };

            // Calcul du bearing instantané de la trace (pour la boussole/vent)
            if (lineStringRef && lineStringRef.value) {
                try {
                    const p1 = turf.along(lineStringRef.value, distanceTraveled, { units: 'kilometers' });
                    const p2 = turf.along(lineStringRef.value, distanceTraveled + 0.01, { units: 'kilometers' });
                    constructedBearing = turf.bearing(p1, p2);
                } catch (e) {
                    constructedBearing = bearing;
                }
            }
        }
        // 3. Interpolation standard point par point (Pas de Keyframe ou hors segment Keyframe)
        else {
            // Trouver l'index courant
            let currentPointIndex = 0;
            // Optimisation : Recherche binaire ou smart search si trackingPoints est grand
            // Pour l'instant on garde la boucle simple inversée ou directe
            // La boucle inversée est plus rapide si on est à la fin, mais au début c'est lent.
            // On fait une recherche simple car on suppose que trackingPoints est dense.
            for (let i = 0; i < trackingPoints.length - 1; i++) {
                if (trackingPoints[i + 1].distance > distanceTraveled) {
                    currentPointIndex = i;
                    break;
                }
                currentPointIndex = i; // Fallback fin
            }


            const currentPoint = trackingPoints[currentPointIndex];
            if (currentPoint) {
                const nextPointIndex = currentPointIndex + 1;

                if (nextPointIndex < trackingPoints.length) {
                    const nextPoint = trackingPoints[nextPointIndex];
                    const prevKeyframeDist = currentPoint.distance;
                    const nextKeyframeDist = nextPoint.distance;
                    const segmentDist = nextKeyframeDist - prevKeyframeDist;
                    const progressInSegment = segmentDist > 0 ? (distanceTraveled - prevKeyframeDist) / segmentDist : 0;

                    const lookAtPointLng = lerp(currentPoint.coordonnee[0], nextPoint.coordonnee[0], progressInSegment);
                    const lookAtPointLat = lerp(currentPoint.coordonnee[1], nextPoint.coordonnee[1], progressInSegment);

                    const prevPitch = currentPoint.editedPitch ?? currentPoint.pitch;
                    const nextPitch = nextPoint.editedPitch ?? nextPoint.pitch;
                    const prevCap = currentPoint.editedCap ?? currentPoint.cap;
                    const nextCap = nextPoint.editedCap ?? nextPoint.cap;

                    let zoom = lerp(currentPoint.editedZoom ?? currentPoint.zoom, nextPoint.editedZoom ?? nextPoint.zoom, progressInSegment);
                    zoom += (1 - currentSpeed) * (dynamicZoomIntensity / 50);

                    const pitch = lerp(prevPitch, nextPitch, progressInSegment);
                    const bearing = lerpAngle(prevCap, nextCap, progressInSegment);

                    target = {
                        center: [lookAtPointLng, lookAtPointLat],
                        zoom,
                        pitch,
                        bearing
                    };

                    // Calcul du bearing de la trace

                    // Logique Multi-segment (Variant)
                    if (isMultisegment && activeVariantSegments && activeVariantSegments.length > 0) {
                        const currentSeg = activeVariantSegments.find(s => distanceTraveled >= s.startDistKm && distanceTraveled < s.endDistKm);
                        if (currentSeg && currentSeg.coordinates && currentSeg.coordinates.length > 1) {
                            const localDist = distanceTraveled - currentSeg.startDistKm;
                            const segmentLine = turf.lineString(currentSeg.coordinates);
                            const len = currentSeg.lengthKm;
                            const dist1 = Math.min(localDist, len);
                            const dist2 = Math.min(localDist + 0.01, len);
                            const p1 = turf.along(segmentLine, dist1, { units: 'kilometers' });
                            const p2 = turf.along(segmentLine, dist2, { units: 'kilometers' });
                            constructedBearing = turf.bearing(p1, p2);
                        }
                    }
                    // Logique Standard
                    else if (lineStringRef && lineStringRef.value && lineStringRef.value.geometry && lineStringRef.value.geometry.type === 'LineString') { // Safety check added
                        try {
                            const p1 = turf.along(lineStringRef.value, distanceTraveled, { units: 'kilometers' });
                            const p2 = turf.along(lineStringRef.value, distanceTraveled + 0.01, { units: 'kilometers' });
                            constructedBearing = turf.bearing(p1, p2);
                        } catch (e) { }
                    }

                    if (constructedBearing === null) constructedBearing = bearing;

                } else {
                    // Dernier point exact
                    let zoom = (currentPoint.editedZoom ?? currentPoint.zoom);
                    zoom += (1 - currentSpeed) * (dynamicZoomIntensity / 50);

                    const pitch = currentPoint.editedPitch ?? currentPoint.pitch;
                    const bearing = currentPoint.editedCap ?? currentPoint.cap;

                    target = {
                        center: currentPoint.coordonnee,
                        zoom,
                        pitch,
                        bearing
                    };
                    constructedBearing = bearing;
                }
            }
        }

        // Apply if requested
        if (apply && target && map.value) {
            map.value.setZoom(target.zoom);
            map.value.setPitch(target.pitch);
            map.value.setBearing(target.bearing);
            map.value.setCenter(target.center);
        }

        return {
            bearing: constructedBearing,
            target
        };
    };

    return {
        updateCameraPosition,
        lerp,
        lerpAngle
    };
}
