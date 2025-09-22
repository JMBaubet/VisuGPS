# Génération des Points de Suivi tous les 100m

## Contexte et Objectif

L'objectif de cet algorithme est de créer une série de points le long d'une trace GPX, en s'assurant que chaque point est espacé précisément de 100 mètres (ou de la `segment_length` configurée) du précédent, mesuré le long du chemin réel de la trace. Ceci est essentiel pour une analyse et une visualisation cohérentes de la trace.

---

## Algorithme : Échantillonnage le long de la LineString

L'algorithme parcourt la `LineString` originale segment par segment, en "découpant" des portions de 100m (ou `segment_length`) et en interpolant les coordonnées des points à ces distances précises.

### Étape 1 : Initialisation

-   On commence avec le tout premier point de la `LineString` originale, qui est ajouté à la liste des points calculés.
-   Une variable `distance_needed` est initialisée à la `segment_length` (par exemple, 100m), représentant la prochaine distance à laquelle un point doit être trouvé.
-   Une variable `distance_traversed` est initialisée à 0, pour suivre la distance parcourue le long de la trace.

### Étape 2 : Parcours des Segments de la Trace

L'algorithme itère sur chaque segment de la `LineString` originale (de `p1` à `p2`).

### Étape 3 : Interpolation des Points dans le Segment Actuel

Pour chaque segment, une boucle interne vérifie si la `distance_needed` (la prochaine marque des 100m, 200m, etc.) tombe à l'intérieur de ce segment :

-   Si `distance_traversed + longueur_du_segment >= distance_needed` :
    -   Cela signifie que le point à `distance_needed` se trouve dans le segment actuel.
    -   On calcule la fraction de la longueur du segment où se situe ce point.
    -   On utilise cette fraction pour interpoler précisément les coordonnées du nouveau point (`new_point`) le long du segment.
    -   Ce `new_point` est ajouté à la liste `calculated_points`.
    -   `distance_needed` est incrémenté de `segment_length` (on cherche le prochain point).
    -   Cette boucle interne continue tant que d'autres points à `segment_length` peuvent être trouvés dans le même segment.

### Étape 4 : Avancement de la Distance Totale

Une fois que tous les points à `segment_length` ont été extraits du segment actuel, la longueur de ce segment est ajoutée à `distance_traversed` pour passer au segment suivant.

### Étape 5 : Inclusion du Dernier Point

Après avoir parcouru tous les segments, une vérification finale est effectuée pour s'assurer que le tout dernier point de la `LineString` originale est bien inclus dans la liste `calculated_points`, même s'il est à moins de `segment_length` du dernier point calculé.

---

## Extrait du Code Rust

Voici la partie du code implémentée dans `src-tauri/src/tracking_processor.rs` qui gère cette logique :

```rust
    let mut calculated_points = Vec::new();
    let mut distance_needed = 0.0;
    let mut distance_traversed = 0.0;

    // Ajoute le tout premier point de la trace
    if let Some(first_point) = line.points().next() {
        calculated_points.push(first_point);
        distance_needed += segment_length;
    }

    // Parcourt chaque segment de la LineString originale
    for segment in line.lines() {
        let p1 = segment.start;
        let p2 = segment.end;
        let segment_len = Point::from(p1).haversine_distance(&Point::from(p2));

        // Tant que la prochaine distance requise tombe dans le segment actuel
        while distance_traversed + segment_len >= distance_needed {
            let dist_into_segment = distance_needed - distance_traversed;
            let fraction = if segment_len > 0.0 { dist_into_segment / segment_len } else { 0.0 };
            
            // Interpolation du point à la distance exacte
            let new_point = geo::Line::new(p1, p2).line_interpolate_point(fraction).unwrap();
            calculated_points.push(new_point);
            distance_needed += segment_length;
        }
        distance_traversed += segment_len;
    }

    // Ajoute le tout dernier point de la trace originale si nécessaire
    if let Some(last_original_point) = line.points().last() {
        if let Some(last_calculated_point) = calculated_points.last() {
            // Si le dernier point calculé est trop éloigné du dernier point original
            if last_calculated_point.haversine_distance(&last_original_point) > 1.0 { // Seuil de 1m
                 calculated_points.push(last_original_point);
            }
        }
    }
```
