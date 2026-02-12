// src/composables/useSlopeColors.js
// Composable partagé pour la gestion des couleurs de pente (slope colors)

/**
 * Retourne la couleur hex correspondant à une pente donnée.
 * @param {number} slope - La pente en pourcentage
 * @param {Object} colorMap - Le HashMap des couleurs (clé → hex)
 * @returns {string} La couleur hex
 */
export function getSlopeColor(slope, colorMap) {
    if (slope < -12) return colorMap.TrancheNeg6 || '#01579B';
    if (slope < -9) return colorMap.TrancheNeg5 || '#0277BD';
    if (slope < -6) return colorMap.TrancheNeg4 || '#0288D1';
    if (slope < -3) return colorMap.TrancheNeg3 || '#039BE5';
    if (slope < -1) return colorMap.TrancheNeg2 || '#29B6F6';
    if (slope < 1) return colorMap.TrancheNeg1 || '#4FC3F7';
    if (slope < 3) return colorMap.Tranche1 || '#4CAF50';
    if (slope < 6) return colorMap.Tranche2 || '#FFEB3B';
    if (slope < 9) return colorMap.Tranche3 || '#FF9800';
    if (slope < 12) return colorMap.Tranche4 || '#F44336';
    return colorMap.Tranche5 || '#9C27B0';
}

/**
 * Construit le HashMap complet des couleurs de pente à partir des settings.
 * Les 6 tranches négatives sont dérivées automatiquement de la couleur de base TrancheNegative.
 * @param {Function} getSettingValue - Fonction pour lire un paramètre
 * @param {Function} toHex - Fonction de conversion nom Vuetify → hex
 * @returns {Promise<Object>} Le HashMap des 11 couleurs
 */
export async function buildSlopeColorsMap(getSettingValue, toHex) {
    const baseName = await getSettingValue('Visualisation/Profil Altitude/Couleurs/TrancheNegative') || 'light-blue';

    return {
        // Tranches négatives (dérivées de la couleur de base)
        TrancheNeg1: toHex(`${baseName}-lighten-2`),
        TrancheNeg2: toHex(`${baseName}-lighten-1`),
        TrancheNeg3: toHex(`${baseName}-darken-1`),
        TrancheNeg4: toHex(`${baseName}-darken-2`),
        TrancheNeg5: toHex(`${baseName}-darken-3`),
        TrancheNeg6: toHex(`${baseName}-darken-4`),
        // Tranches positives (configurables individuellement)
        Tranche1: toHex(await getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche1') || 'green'),
        Tranche2: toHex(await getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche2') || 'yellow'),
        Tranche3: toHex(await getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche3') || 'orange'),
        Tranche4: toHex(await getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche4') || 'red'),
        Tranche5: toHex(await getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche5') || 'purple'),
    };
}
