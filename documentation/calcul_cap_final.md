# Calcul du Cap Final — Algorithme Actuel (Champ de Vision Pondéré)

## Contexte et Objectif

Après plusieurs itérations, l'objectif du calcul de cap a été affiné. Le but n'est pas de connaître la direction instantanée de la trace, mais de simuler une caméra positionnée sur le point actuel (`P`) et de déterminer la direction centrale dans laquelle elle devrait regarder pour cadrer la scène des `n` prochains points.

De plus, pour une navigation plus naturelle, les points les plus éloignés dans cette "scène" doivent avoir plus d'influence sur l'orientation de la caméra. Le cap doit donc être le résultat d'une **moyenne pondérée par la distance**.

---

## Algorithme : Champ de Vision Pondéré par la Distance

Voici les étapes de l'algorithme final implémenté :

### Étape 1 : Collecte des Données

Pour le point actuel `P_i`, on regarde les `n` points suivants dans la trace (où `n` est le paramètre `LissageCap`). Pour chacun de ces points futurs `P_k` :

1.  On calcule le **cap** (l'angle en degrés) de `P_i` à `P_k`.
2.  On calcule la **distance** (en mètres) de `P_i` à `P_k`.

On obtient ainsi une liste de `n` paires `(cap, distance)`.

### Étape 2 : Identification des Bords du Champ de Vision

Dans cette liste, on recherche les deux paires qui définissent les limites de notre "champ de vision" :

-   La paire qui possède le **plus petit cap** (numériquement). On la nomme `(cap_min, dist_min)`.
-   La paire qui possède le **plus grand cap** (numériquement). On la nomme `(cap_max, dist_max)`.

### Étape 3 : Calcul de la Moyenne Pondérée par Vecteurs

Pour moyenner correctement des angles tout en les pondérant par une valeur (la distance), on utilise une approche vectorielle.

1.  **Conversion en Vecteurs Pondérés** : On transforme nos deux caps "extrêmes" en vecteurs, et on "allonge" chaque vecteur en le multipliant par sa distance. Un point lointain aura un vecteur plus long, donc plus de "force".

    ```
    // Conversion en radians pour les fonctions cos/sin
    rad_min = cap_min * PI / 180
    rad_max = cap_max * PI / 180

    // Calcul des composantes X et Y des vecteurs pondérés
    x_pondere_min = dist_min * cos(rad_min)
    y_pondere_min = dist_min * sin(rad_min)

    x_pondere_max = dist_max * cos(rad_max)
    y_pondere_max = dist_max * sin(rad_max)
    ```

2.  **Somme des Vecteurs** : On additionne les composantes des deux vecteurs pour obtenir un unique vecteur final qui représente la direction d'équilibre.

    ```
    X_total = x_pondere_min + x_pondere_max
    Y_total = y_pondere_min + y_pondere_max
    ```

3.  **Calcul du Cap Final** : L'angle de ce `V_final` nous donne notre cap moyen pondéré. On le calcule avec la fonction `atan2(Y_total, X_total)` et on le reconvertit en degrés (en le normalisant entre 0° et 360°).

---

## Extrait du Code Rust Final

Voici la fonction implémentée dans `src-tauri/src/tracking_processor.rs` :

```rust
fn calculate_smoothed_bearing(current_index: usize, points: &Vec<Point<f64>>, window_size: usize) -> f64 {
    if points.len() < 2 || current_index >= points.len() - 1 {
        // Cas de repli pour la toute fin de la trace
        if current_index > 0 && current_index < points.len() {
            return points[current_index - 1].haversine_bearing(points[current_index]);
        }
        return 0.0;
    }

    let current_point = points[current_index];
    let end_index = (current_index + 1 + window_size).min(points.len());
    
    let mut bearing_distance_pairs = Vec::new();

    // Étape 1: Calcul du cap et de la distance pour chaque point dans la fenêtre à venir
    for i in (current_index + 1)..end_index {
        let target_point = points[i];
        if current_point != target_point {
            let bearing = current_point.haversine_bearing(target_point);
            let distance = current_point.haversine_distance(&target_point);
            bearing_distance_pairs.push((bearing, distance));
        }
    }

    if bearing_distance_pairs.is_empty() {
        // Cas de repli si aucun point valide n'est trouvé
        if current_index + 1 < points.len() {
             return points[current_index].haversine_bearing(points[current_index + 1]);
        }
       return 0.0;
    }

    // Étape 2: Trouver les paires correspondant au cap min et max
    let (min_bearing, dist_for_min) = *bearing_distance_pairs.iter().min_by(|a, b| a.0.partial_cmp(&b.0).unwrap()).unwrap();
    let (max_bearing, dist_for_max) = *bearing_distance_pairs.iter().max_by(|a, b| a.0.partial_cmp(&b.0).unwrap()).unwrap();

    // Étape 3, 4, 5: Moyenne circulaire pondérée par vecteurs
    let rad_min = min_bearing.to_radians();
    let rad_max = max_bearing.to_radians();

    let sum_x = dist_for_min * rad_min.cos() + dist_for_max * rad_max.cos();
    let sum_y = dist_for_min * rad_min.sin() + dist_for_max * rad_max.sin();

    // Étape 6: Calcul de l'angle final
    let avg_rad = sum_y.atan2(sum_x);
    let mut avg_bearing = avg_rad.to_degrees();

    // Normalisation entre 0 et 360
    if avg_bearing < 0.0 {
        avg_bearing += 360.0;
    }
    
    avg_bearing
}
```