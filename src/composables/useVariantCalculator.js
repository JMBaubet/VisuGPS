export function useVariantCalculator() {
    /**
     * Calcule les statistiques projetées/réelles d'une variante.
     * 
     * @param {number} masterLength - Longueur totale de la trace maître (km).
     * @param {Array} modifications - Liste des modifications de la variante.
     * @param {number} segmentLength - Longueur d'un segment de tracking (m, ex: 100).
     * @param {number} currentSegmentIndex - Index de la modification en cours d'édition (Optionnel).
     * @param {number} currentProgressDistance - Distance parcourue dans le segment actuel (Optionnel, km).
     * @returns {Object} { total: number, current: number } - Distances en km.
     */
    const calculateVariantStats = (masterLength, modifications, segmentLength, currentSegmentIndex = null, currentProgressDistance = 0) => {
        if (!masterLength) return { total: 0, current: 0 };

        const segLenKm = segmentLength / 1000;
        let totalDistance = masterLength;
        let currentDistance = 0;

        // 1. Calculer la distance totale de la variante
        // On retire les morceaux du maître et on ajoute les nouveaux morceaux
        const sortedMods = [...modifications].sort((a, b) => {
            const getStartIdx = (m) => {
                if (m.type === 'DEPART_DEPORTE') return 0;
                if (m.type === 'ARRIVEE_REPORTEE') return m.anchorIndexOnMaster || 0;
                if (m.type === 'SEGMENT_DEVIATION') return m.anchorStart?.index || 0;
                return 0;
            };
            return getStartIdx(a) - getStartIdx(b);
        });

        for (const mod of sortedMods) {
            let removedLength = 0;
            if (mod.type === 'DEPART_DEPORTE') {
                removedLength = (mod.anchorIndexOnMaster || 0) * segLenKm;
            } else if (mod.type === 'ARRIVEE_REPORTEE') {
                const startIdx = mod.anchorIndexOnMaster || 0;
                // On estime l'index de fin par rapport à la longueur totale
                const endIdx = Math.round(masterLength / segLenKm);
                removedLength = Math.max(0, (endIdx - startIdx) * segLenKm);
            } else if (mod.type === 'SEGMENT_DEVIATION') {
                const startIdx = mod.anchorStart?.index || 0;
                const endIdx = mod.anchorEnd?.index || 0;
                removedLength = Math.max(0, (endIdx - startIdx) * segLenKm);
            }

            totalDistance = totalDistance - removedLength + (mod.longueur || 0);
        }

        // 2. Calculer la position actuelle si spécifiée
        if (currentSegmentIndex !== null) {
            // Identifier le type et la position du segment actuel dans la liste triée
            // On calcule la distance du début de la variante jusqu'au début du segment actuel, 
            // puis on ajoute currentProgressDistance.

            let accumulated = 0;
            let currentMasterPos = 0; // Position sur le maître (index de tracking)

            for (let i = 0; i < sortedMods.length; i++) {
                const mod = sortedMods[i];
                const modOrigIndex = modifications.indexOf(mod);

                // Section commune AVANT ce segment
                let nextMasterStart = 0;
                if (mod.type === 'DEPART_DEPORTE') nextMasterStart = 0;
                else if (mod.type === 'ARRIVEE_REPORTEE') nextMasterStart = mod.anchorIndexOnMaster;
                else if (mod.type === 'SEGMENT_DEVIATION') nextMasterStart = mod.anchorStart?.index;

                const commonSectionLength = Math.max(0, (nextMasterStart - currentMasterPos) * segLenKm);

                if (modOrigIndex === currentSegmentIndex) {
                    accumulated += commonSectionLength;
                    currentDistance = accumulated + currentProgressDistance;
                    break;
                }

                accumulated += commonSectionLength + (mod.longueur || 0);

                // Mettre à jour la position sur le maître après le segment
                if (mod.type === 'DEPART_DEPORTE') currentMasterPos = mod.anchorIndexOnMaster;
                else if (mod.type === 'ARRIVEE_REPORTEE') currentMasterPos = Math.round(masterLength / segLenKm);
                else if (mod.type === 'SEGMENT_DEVIATION') currentMasterPos = mod.anchorEnd?.index;
            }
        }

        return {
            total: Math.max(0, totalDistance),
            current: Math.max(0, currentDistance)
        };
    };

    return {
        calculateVariantStats
    };
}
