#!/usr/bin/env node

/**
 * Script d'analyse des traces GPX pour détecter les allers-retours
 * Détecte les segments qui se superposent géographiquement
 */

import fs from 'fs';
import path from 'path';

// Fonction pour calculer la distance entre deux points (formule de Haversine)
function haversineDistance(lat1, lon1, lat2, lon2) {
    const R = 6371000; // Rayon de la Terre en mètres
    const φ1 = lat1 * Math.PI / 180;
    const φ2 = lat2 * Math.PI / 180;
    const Δφ = (lat2 - lat1) * Math.PI / 180;
    const Δλ = (lon2 - lon1) * Math.PI / 180;

    const a = Math.sin(Δφ / 2) * Math.sin(Δφ / 2) +
        Math.cos(φ1) * Math.cos(φ2) *
        Math.sin(Δλ / 2) * Math.sin(Δλ / 2);
    const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));

    return R * c;
}

// Fonction pour parser un fichier GPX
function parseGPX(filePath) {
    const content = fs.readFileSync(filePath, 'utf-8');
    const points = [];

    // Regex pour extraire les trackpoints
    const trkptRegex = /<trkpt lat="([^"]+)" lon="([^"]+)">\s*<ele>([^<]+)<\/ele>\s*<time>([^<]+)<\/time>/g;

    let match;
    while ((match = trkptRegex.exec(content)) !== null) {
        points.push({
            lat: parseFloat(match[1]),
            lon: parseFloat(match[2]),
            ele: parseFloat(match[3]),
            time: new Date(match[4])
        });
    }

    return points;
}

// Fonction pour détecter les segments qui se superposent
function detectOverlaps(points, distanceThreshold = 20) {
    const overlaps = [];

    console.log(`\n📊 Analyse de ${points.length} points avec seuil de ${distanceThreshold}m`);

    // Pour chaque point, chercher les points ultérieurs qui sont proches géographiquement
    for (let i = 0; i < points.length - 50; i++) {
        const p1 = points[i];

        // Chercher dans les points suivants (au moins 50 points plus loin pour éviter les faux positifs)
        for (let j = i + 50; j < points.length; j++) {
            const p2 = points[j];

            const distance = haversineDistance(p1.lat, p1.lon, p2.lat, p2.lon);

            if (distance < distanceThreshold) {
                const elevationDiff = Math.abs(p2.ele - p1.ele);
                const timeDiff = (p2.time - p1.time) / 1000; // en secondes

                overlaps.push({
                    point1Index: i,
                    point2Index: j,
                    distance: distance.toFixed(2),
                    elevationDiff: elevationDiff.toFixed(1),
                    timeDiff: timeDiff.toFixed(0),
                    point1: { lat: p1.lat, lon: p1.lon, ele: p1.ele },
                    point2: { lat: p2.lat, lon: p2.lon, ele: p2.ele }
                });
            }
        }
    }

    return overlaps;
}

// Fonction pour regrouper les overlaps en zones
function groupOverlapsIntoZones(overlaps, proximityThreshold = 100) {
    if (overlaps.length === 0) return [];

    const zones = [];
    const used = new Set();

    for (let i = 0; i < overlaps.length; i++) {
        if (used.has(i)) continue;

        const zone = {
            overlaps: [overlaps[i]],
            startIndex: overlaps[i].point1Index,
            endIndex: overlaps[i].point2Index,
            count: 1
        };

        used.add(i);

        // Chercher les overlaps proches
        for (let j = i + 1; j < overlaps.length; j++) {
            if (used.has(j)) continue;

            const indexDiff = Math.abs(overlaps[j].point1Index - zone.startIndex);

            if (indexDiff < proximityThreshold) {
                zone.overlaps.push(overlaps[j]);
                zone.endIndex = Math.max(zone.endIndex, overlaps[j].point2Index);
                zone.count++;
                used.add(j);
            }
        }

        zones.push(zone);
    }

    return zones;
}

// Fonction principale d'analyse
function analyzeGPXFile(filePath) {
    const fileName = path.basename(filePath);
    console.log(`\n${'='.repeat(80)}`);
    console.log(`🗺️  Analyse de: ${fileName}`);
    console.log(`${'='.repeat(80)}`);

    const points = parseGPX(filePath);
    console.log(`✅ ${points.length} points chargés`);

    if (points.length === 0) {
        console.log('❌ Aucun point trouvé dans le fichier');
        return;
    }

    // Statistiques de base
    const elevations = points.map(p => p.ele);
    const minEle = Math.min(...elevations);
    const maxEle = Math.max(...elevations);
    const totalTime = (points[points.length - 1].time - points[0].time) / 1000 / 60; // en minutes

    console.log(`📈 Altitude min: ${minEle.toFixed(0)}m, max: ${maxEle.toFixed(0)}m, dénivelé: ${(maxEle - minEle).toFixed(0)}m`);
    console.log(`⏱️  Durée totale: ${totalTime.toFixed(0)} minutes`);

    // Détection des overlaps
    const overlaps = detectOverlaps(points, 20);
    console.log(`\n🔍 ${overlaps.length} superpositions détectées`);

    let zones = [];
    if (overlaps.length > 0) {
        // Regrouper en zones
        zones = groupOverlapsIntoZones(overlaps, 100);
        console.log(`📍 ${zones.length} zones d'allers-retours identifiées\n`);

        // Afficher les zones principales (plus de 5 overlaps)
        const majorZones = zones.filter(z => z.count > 5).slice(0, 5);

        if (majorZones.length > 0) {
            console.log(`🎯 Zones principales d'allers-retours:\n`);
            majorZones.forEach((zone, idx) => {
                const avgElevDiff = zone.overlaps.reduce((sum, o) => sum + parseFloat(o.elevationDiff), 0) / zone.overlaps.length;
                console.log(`   Zone ${idx + 1}:`);
                console.log(`   - Points ${zone.startIndex} à ${zone.endIndex}`);
                console.log(`   - ${zone.count} superpositions`);
                console.log(`   - Différence d'altitude moyenne: ${avgElevDiff.toFixed(1)}m`);
                console.log('');
            });
        }

        // Exemples de superpositions
        console.log(`📋 Exemples de superpositions:\n`);
        overlaps.slice(0, 10).forEach((overlap, idx) => {
            console.log(`   ${idx + 1}. Points ${overlap.point1Index} ↔ ${overlap.point2Index}`);
            console.log(`      Distance: ${overlap.distance}m, Δ altitude: ${overlap.elevationDiff}m, Δ temps: ${overlap.timeDiff}s`);
        });
    } else {
        console.log('✅ Aucun aller-retour détecté dans cette trace');
    }

    return {
        fileName,
        totalPoints: points.length,
        overlapsCount: overlaps.length,
        zonesCount: zones.length,
        points,
        overlaps,
        zones
    };
}

// Analyse de tous les fichiers
const tracesDir = '/Volumes/Externe/Dev/VisuGPS/tmp/test-traces';
const files = fs.readdirSync(tracesDir).filter(f => f.endsWith('.gpx'));

console.log(`\n🚀 Analyse de ${files.length} fichiers GPX\n`);

const results = files.map(file => {
    const filePath = path.join(tracesDir, file);
    return analyzeGPXFile(filePath);
});

// Résumé global
console.log(`\n${'='.repeat(80)}`);
console.log(`📊 RÉSUMÉ GLOBAL`);
console.log(`${'='.repeat(80)}\n`);

results.forEach(result => {
    if (result) {
        console.log(`${result.fileName}:`);
        console.log(`   - ${result.totalPoints} points`);
        console.log(`   - ${result.overlapsCount} superpositions`);
        console.log(`   - ${result.zonesCount} zones d'allers-retours`);
        console.log('');
    }
});

console.log(`\n✅ Analyse terminée\n`);
