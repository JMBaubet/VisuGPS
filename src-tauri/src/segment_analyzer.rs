use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Représente une zone où deux segments se superposent (aller et retour)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SegmentOverlap {
    pub zone_id: usize,
    pub aller_start_index: usize,
    pub aller_end_index: usize,
    pub retour_start_index: usize,
    pub retour_end_index: usize,
    pub aller_start_km: f64,
    pub aller_end_km: f64,
    pub retour_start_km: f64,
    pub retour_end_km: f64,
}

/// Métadonnées de segmentation pour un circuit
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SegmentMetadata {
    pub circuit_id: String,
    pub overlapping_zones: Vec<SegmentOverlap>,
    pub detection_threshold_meters: f64,
    pub total_points: usize,
    pub analysis_date: String,
}

/// Point de tracking simplifié pour l'analyse
#[derive(Debug, Clone)]
struct TrackingPoint {
    lat: f64,
    lon: f64,
    distance_km: f64,
}

/// Calcule la distance entre deux points GPS en utilisant la formule de Haversine
///
/// # Arguments
/// * `lat1` - Latitude du premier point (en degrés)
/// * `lon1` - Longitude du premier point (en degrés)
/// * `lat2` - Latitude du deuxième point (en degrés)
/// * `lon2` - Longitude du deuxième point (en degrés)
///
/// # Returns
/// Distance en mètres
pub fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const EARTH_RADIUS_M: f64 = 6_371_000.0; // Rayon de la Terre en mètres

    let phi1 = lat1 * PI / 180.0;
    let phi2 = lat2 * PI / 180.0;
    let delta_phi = (lat2 - lat1) * PI / 180.0;
    let delta_lambda = (lon2 - lon1) * PI / 180.0;

    let a = (delta_phi / 2.0).sin().powi(2)
        + phi1.cos() * phi2.cos() * (delta_lambda / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS_M * c
}

/// Détecte les segments qui se superposent dans une trace GPX
///
/// # Arguments
/// * `tracking_points` - Liste des points de tracking avec coordonnées et distances
/// * `distance_threshold` - Distance maximale (en mètres) pour considérer deux points comme superposés
///
/// # Returns
/// Vecteur de zones de superposition détectées
pub fn detect_overlapping_segments(
    tracking_points: &[serde_json::Value],
    distance_threshold: f64,
) -> Result<Vec<SegmentOverlap>, String> {
    // Convertir les points JSON en structure simplifiée
    let points: Vec<TrackingPoint> = tracking_points
        .iter()
        .enumerate()
        .filter_map(|(_, point)| {
            let coordonnee = point.get("coordonnee")?.as_array()?;
            let lon = coordonnee.get(0)?.as_f64()?;
            let lat = coordonnee.get(1)?.as_f64()?;
            let distance_km = point.get("distance")?.as_f64()?;

            Some(TrackingPoint {
                lat,
                lon,
                distance_km,
            })
        })
        .collect();

    if points.len() < 100 {
        return Ok(Vec::new()); // Pas assez de points pour détecter des superpositions
    }

    // Détecter les paires de points qui se superposent
    let mut overlaps: Vec<(usize, usize, f64)> = Vec::new();

    // Pour chaque point, chercher les points ultérieurs qui sont proches géographiquement
    // On utilise un écart en DISTANCE (mètres) plutôt qu'en nombre de points
    // Cela permet de gérer correctement les demi-tours (U-turns) quelle que soit la densité des points
    const MIN_TRACK_DISTANCE_DIFF_M: f64 = 200.0; // 200m d'écart sur la trace avant de chercher une superposition

    for i in 0..points.len() {
        let p1 = &points[i];

        // On cherche j plus loin dans la trace
        // On avance j tant que la distance sur trace n'est pas suffisante
        // Optimisation: on peut commencer j 'un peu plus loin'
        // Mais comme on iter sur tous, on cherche le premier j valide
        
        // Trouver l'index de départ pour j tel que distance_trace(j) - distance_trace(i) > MIN_DIFF
        let mut start_j = i + 1;
        while start_j < points.len() && (points[start_j].distance_km - p1.distance_km) * 1000.0 < MIN_TRACK_DISTANCE_DIFF_M {
            start_j += 1;
        }

        for j in start_j..points.len() {
            let p2 = &points[j];

            let distance = haversine_distance(p1.lat, p1.lon, p2.lat, p2.lon);

            if distance < distance_threshold {
                overlaps.push((i, j, distance));
            }
        }
    }

    if overlaps.is_empty() {
        return Ok(Vec::new());
    }

    // Regrouper les overlaps en zones continues
    // On revient à un seuil raisonnable (100 points) car le problème des U-turns est géré par MIN_TRACK_DISTANCE_DIFF_M
    let zones = group_overlaps_into_zones(&overlaps, &points, 100);

    Ok(zones)
}

/// Regroupe les superpositions individuelles en zones continues
fn group_overlaps_into_zones(
    overlaps: &[(usize, usize, f64)],
    points: &[TrackingPoint],
    proximity_threshold: usize,
) -> Vec<SegmentOverlap> {
    if overlaps.is_empty() {
        return Vec::new();
    }

    let mut zones: Vec<SegmentOverlap> = Vec::new();
    let mut used = vec![false; overlaps.len()];

    for (idx, &(start_idx, end_idx, _)) in overlaps.iter().enumerate() {
        if used[idx] {
            continue;
        }

        let mut zone_aller_start = start_idx;
        let mut zone_aller_end = start_idx;
        let mut zone_retour_start = end_idx;
        let mut zone_retour_end = end_idx;

        used[idx] = true;

        // Chercher les overlaps proches pour étendre la zone
        for (other_idx, &(other_start, other_end, _)) in overlaps.iter().enumerate() {
            if used[other_idx] {
                continue;
            }

            // Vérifier si cet overlap est proche de la zone actuelle
            // On exige une continuité sur les DEUX index (aller ET retour) pour éviter
            // de fusionner des zones géographiquement proches mais chronologiquement très éloignées
            // (ex: le début de la trace et la fin d'une boucle)
            let start_diff = (other_start as i32 - zone_aller_end as i32).abs() as usize;
            let end_diff = (other_end as i32 - zone_retour_end as i32).abs() as usize;

            if start_diff < proximity_threshold && end_diff < proximity_threshold {
                zone_aller_end = other_start;
                zone_retour_end = other_end;
                used[other_idx] = true;
            }
        }

        // Créer la zone en normalisant les index (start doit être < end)
        // C'est important pour que les tests de présence dans une plage fonctionnent
        let (s_a, e_a) = (zone_aller_start.min(zone_aller_end), zone_aller_start.max(zone_aller_end));
        let (s_r, e_r) = (zone_retour_start.min(zone_retour_end), zone_retour_start.max(zone_retour_end));

        zones.push(SegmentOverlap {
            zone_id: zones.len() + 1,
            aller_start_index: s_a,
            aller_end_index: e_a,
            retour_start_index: s_r,
            retour_end_index: e_r,
            aller_start_km: points[s_a].distance_km,
            aller_end_km: points[e_a].distance_km,
            retour_start_km: points[s_r].distance_km,
            retour_end_km: points[e_r].distance_km,
        });
    }

    zones
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haversine_distance() {
        // Paris à Lyon (environ 392 km)
        let paris_lat = 48.8566;
        let paris_lon = 2.3522;
        let lyon_lat = 45.7640;
        let lyon_lon = 4.8357;

        let distance = haversine_distance(paris_lat, paris_lon, lyon_lat, lyon_lon);
        
        // La distance réelle est d'environ 392 km
        assert!((distance - 392_000.0).abs() < 10_000.0, "Distance should be around 392 km");
    }

    #[test]
    fn test_haversine_distance_same_point() {
        let lat = 45.0;
        let lon = 5.0;

        let distance = haversine_distance(lat, lon, lat, lon);
        assert!(distance < 0.1, "Distance between same point should be ~0");
    }

    #[test]
    fn test_haversine_distance_close_points() {
        // Deux points à environ 20m l'un de l'autre
        let lat1 = 45.0;
        let lon1 = 5.0;
        let lat2 = 45.0002;
        let lon2 = 5.0;

        let distance = haversine_distance(lat1, lon1, lat2, lon2);
        assert!((distance - 22.0).abs() < 5.0, "Distance should be around 22 meters");
    }

    #[test]
    fn test_group_overlaps_separation() {
        let points = vec![
            TrackingPoint { lat: 45.0, lon: 5.0, distance_km: 0.0 },
            TrackingPoint { lat: 45.0, lon: 5.0, distance_km: 0.1 },
            TrackingPoint { lat: 45.0, lon: 5.0, distance_km: 0.2 },
            TrackingPoint { lat: 46.0, lon: 6.0, distance_km: 1.0 }, // Loin
            TrackingPoint { lat: 45.0, lon: 5.0, distance_km: 2.0 }, // Retour 1
            TrackingPoint { lat: 45.0, lon: 5.0, distance_km: 3.0 }, // Retour 2 (Loin du Retour 1)
        ];

        // On simule deux superpositions : 
        // 1. (0, 4) - Le tout début avec le retour 1
        // 2. (1, 5) - Un peu après le début avec le retour 2
        let overlaps = vec![
            (0, 4, 1.0),
            (1, 5, 1.0),
        ];

        // Seuil de proximité = 5 points
        let zones = group_overlaps_into_zones(&overlaps, &points, 3);
        
        // start_diff = |1 - 0| = 1 (< 3)
        // end_diff = |5 - 4| = 1 (< 3)
        // ==> Celles-ci doivent fusionner (c'est un AR continu)
        assert_eq!(zones.len(), 1, "Un AR continu doit fusionner");

        // Simulons maintenant le cas problématique du début vs fin de boucle
        let overlaps_mixed = vec![
            (0, 4, 1.0), // Début avec retour proche
            (0, 50, 1.0), // Début avec fin de boucle (beaucoup plus loin)
        ];
        
        // On rajoute un point fictif à l'index 50
        let mut many_points = points.clone();
        for _ in 0..100 {
            many_points.push(TrackingPoint { lat: 47.0, lon: 7.0, distance_km: 10.0 });
        }

        let zones_mixed = group_overlaps_into_zones(&overlaps_mixed, &many_points, 10);
        assert_eq!(zones_mixed.len(), 2, "Les zones chronologiquement distantes ne doivent pas fusionner");
    }
}
