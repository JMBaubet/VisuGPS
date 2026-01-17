import { computed } from 'vue';
import * as turf from '@turf/turf';

/**
 * Composable for calculating total distance and current progression
 * across a master trace with multiple variant segments applied.
 */
export function useVariantCalculator() {

    /**
     * Calculates the total distance and current progression for a variant composite.
     * 
     * @param {number} mainTraceTotalLength - Total length of the master trace in km.
     * @param {Array} modifications - Array of modifications (saved in variant details).
     * @param {number} segmentLength - Length of one tracking segment in km (usually 0.1).
     * @param {number|null} currentSegmentIndex - Index of the currently active variant segment (original index).
     * @param {number} currentProgressInSegment - Current distance in km within the active segment.
     * @returns {Object} { total: number, current: number }
     */
    const calculateVariantStats = (
        mainTraceTotalLength,
        modifications,
        segmentLength,
        currentSegmentIndex = null,
        currentProgressInSegment = 0
    ) => {
        if (!modifications || modifications.length === 0) {
            return { total: mainTraceTotalLength, current: currentProgressInSegment };
        }

        // 1. Sort modifications by anchor index to ensure chronological processing
        const sortedMods = [...modifications].map((m, i) => ({
            ...m,
            // If originalIndex is missing, we use i (useful for RouteBuilder/VariantTraceView)
            originalIndex: m.originalIndex !== undefined ? m.originalIndex : i
        })).sort((a, b) => {
            let idxA = 0;
            if (a.type === 'DEPART_DEPORTE' || a.type === 'DEPART') idxA = -1;
            else if (a.type === 'ARRIVEE_REPORTEE' || a.type === 'ARRIVEE') idxA = Number.MAX_SAFE_INTEGER;
            else idxA = a.anchorStart ? a.anchorStart.index : (a.anchorIndexOnMaster || 0);

            let idxB = 0;
            if (b.type === 'DEPART_DEPORTE' || b.type === 'DEPART') idxB = -1;
            else if (b.type === 'ARRIVEE_REPORTEE' || b.type === 'ARRIVEE') idxB = Number.MAX_SAFE_INTEGER;
            else idxB = b.anchorStart ? b.anchorStart.index : (b.anchorIndexOnMaster || 0);

            return idxA - idxB;
        });

        let total = mainTraceTotalLength;
        let current = currentProgressInSegment;
        let accumulatedOffset = 0;
        let activeSegmentFound = false;

        sortedMods.forEach((mod) => {
            // Calculate Cut Length (on Main Trace)
            let startIndex = 0;
            let endIndex = 0;
            let cutLength = 0;

            if (mod.type === 'DEPART_DEPORTE' || mod.type === 'DEPART') {
                startIndex = 0;
                endIndex = mod.anchorIndexOnMaster || 0;
                cutLength = (endIndex - startIndex) * segmentLength;
            } else if (mod.type === 'ARRIVEE_REPORTEE' || mod.type === 'ARRIVEE') {
                startIndex = mod.anchorIndexOnMaster || 0;
                // For ARRIVEE_REPORTEE, cutLength is from anchor to the end of main trace
                cutLength = Math.max(0, mainTraceTotalLength - (startIndex * segmentLength));
            } else {
                // SEGMENT_DEVIATION or SEGMENT
                startIndex = mod.anchorStart ? mod.anchorStart.index : 0;
                endIndex = mod.anchorEnd ? mod.anchorEnd.index : 0;
                cutLength = (endIndex - startIndex) * segmentLength;
            }

            const variantLength = mod.longueur || 0;

            // Update Total: Total = Total - MainTracePortion + VariantPortion
            total = total - cutLength + variantLength;

            // Update Current (Progress) if we are in a variant
            if (currentSegmentIndex !== null) {
                if (mod.originalIndex === currentSegmentIndex) {
                    activeSegmentFound = true;

                    let startOfMainBeforeCut = startIndex * segmentLength;

                    // Position globally = (Start on Main + Shifts from previous variants) + current segment progress
                    current = (startOfMainBeforeCut + accumulatedOffset) + currentProgressInSegment;
                } else if (!activeSegmentFound) {
                    // This segment is BEFORE the active one, its length change shifts global position
                    accumulatedOffset += (variantLength - cutLength);
                }
            }
        });

        return {
            total: Math.max(0, total),
            current: Math.max(0, current)
        };
    };

    return {
        calculateVariantStats
    };
}
