# Reconstruction du fichier LineString_FULL pour les Variantes

Ce document détaille le processus technique de reconstruction du fichier `lineString_{id}_FULL.json` et la gestion des altitudes, particulièrement lors des défaillances des services d'altitude.

## 1. Origine des données et Routage

Lorsqu'une variante est éditée dans `VariantTraceView.vue`, de nouveaux segments sont créés.
- Si le routage est utilisé, l'application fait appel à l'API **GraphHopper** ou **OpenRouteService**.
- Ces services retournent une géométrie (LineString). Parfois, ces services ne fournissent pas d'altitude ou retournent `0.0`.

## 2. Mise à jour de l'Altitude (`prepare_points_3d`)

Une fois la géométrie récupérée du service de routage, le backend Rust exécute la fonction `prepare_points_3d` dans `variant_processor.rs` :

1. **Identification des manques** : La fonction parcourt chaque point. Si l'altitude est absente ou égale à `0.0`, le point est marqué pour une mise à jour.
2. **Appel à l'API d'altitude** : Une requête groupée est envoyée au service d'altitude configuré (via `elevation_provider.rs`).
3. **Mécanisme de Fallback** : 
   - Si le service d'altitude répond correctement, les `0.0` sont remplacés par les valeurs réelles.
   - **En cas d'erreur ou d'absence de réponse** : Le système conserve la valeur `0.0` pour permettre la sauvegarde de la variante malgré tout, au lieu de bloquer l'utilisateur. Une alerte est affichée dans les logs.

## 3. Assemblage de la trace complète (Stitching)

Le fichier `_FULL` est une reconstruction chronologique de la trace totale (Tracé maître + Modifications). Le processus suit ces étapes :

- **Tri des modifications** : Les segments modifiés sont triés par leur position (index) sur la trace maître.
- **Topological Matching** : Le système utilise une fonction `find_corresponding_idx` pour retrouver précisément où raccorder le nouveau segment sur la trace haute résolution originale, évitant ainsi les erreurs sur les circuits en boucle.
- **Concaténation** : Le système alterne entre les portions "communes" (issues du fichier `lineString.json` original) et les segments "variantes" (nouveaux tracés).

## 4. Lissage et Traitement final

Une fois la trace assemblée, deux traitements majeurs interviennent :

### A. Lissage géométrique (`clean_altitude_data`)
Le système applique des filtres (médiane et moyenne glissante) sur l'ensemble de la nouvelle trace pour éliminer les pics d'altitude aberrants (bruit des GPS ou des modèles numériques de terrain).

### B. Réparation des "trous" (Interpolation Linéaire)
Le système effectue maintenant une réparation systématique à plusieurs niveaux :
1. **Dès le routage** : La commande `calculate_route` appelle automatiquement le service d'altitude pour combler les manques et interpole les points restants à 0.0. Cela garantit une prévisualisation correcte.
2. **Sur les fichiers individuels** : Chaque segment `lineString_{id}_{suffix}.json` est réparé individuellement.
3. **Sur la trace complète** : Le fichier `lineString_{id}_FULL.json` bénéficie d'une réparation globale sur l'ensemble de la géométrie haute résolution.
4. **Sur le fichier de tracking** : Le fichier `tracking_{id}_FULL.json` subit une dernière passe de vérification.

**Logique d'interpolation :**
1. Le système détecte les séquences de points où l'altitude est inférieure à 1.0 mètre.
2. Il cherche le point valide (altitude >= 1.0) immédiatement **avant** le trou et le point valide immédiatement **après**.
3. **Interpolation** : Il calcule une rampe linéaire entre ces deux altitudes pour combler le trou.
   - *Exemple* : Si le point A est à 100m et le point B (5 points plus loin) est à 110m, les points intermédiaires recevront des valeurs progressives (102m, 104m, etc.).
   - *Cas particuliers* : Si le trou est au début ou à la fin de la trace, il effectue un plateau à partir du premier ou dernier point valide trouvé.

## 5. Génération des fichiers finaux

Le résultat est sauvegardé dans deux fichiers clés au sein du dossier `data/{circuit_id}/` :
- `lineString_{variant_id}_FULL.json` : La géométrie 3D haute résolution.
- `tracking_{variant_id}_FULL.json` : La version échantillonnée (tous les 100m) utilisée pour l'animation et l'affichage de l'altimétrie.

---
*Note : Ce processus garantit une continuité visuelle et altimétrique de la trace, même si les services tiers de données géographiques subissent des micro-coupures.*
