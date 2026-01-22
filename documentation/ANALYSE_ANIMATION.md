### Analyse de la Synchronisation entre Tracé, Caméra, Temps et Distance

Ce document explique comment les différents éléments de l'animation "flyby" sont synchronisés. Comprendre cette relation est essentiel pour reproduire ou modifier l'effet.

#### 1. Les Trois Piliers de l'Animation

L'animation repose sur la synchronisation de trois concepts fondamentaux :

1. **Le Tracé au Sol (`LineString.json`)** :
   
   * **Ce que c'est** : Une simple liste de coordonnées `[lon, lat, elevation]` qui représente le parcours physique. )`elevation` ne semble pas utile)
   * **Son Unité de Mesure** : La **distance** (en kilomètres ou mètres). C'est la "vérité terrain". Chaque point du tracé est situé à une distance `d` du point de départ.

2. **Le Chemin de la Caméra (`tracking.json` / `tracking_var`)** :
   
   * **Ce que c'est** : Une liste de "points de contrôle" ou "keyframes" (quand `pointDeControl = true`). Chaque point de contrôle définit un état complet de la caméra (`positionCamera`, `lookAt`, `zoom`, `pitch`, `cap`) à un moment précis.
   * **Son Lien au Tracé** : Chaque point de contrôle est **mappé à une distance spécifique sur le tracé au sol**.
   * **Spécificité Variantes** : Pour les variantes, le fichier de tracking est reconstruit ou récupéré spécifiquement (`tracking_var`) pour correspondre au nouveau parcours hybride.

3. **Le Temps (`time`)** :
   
   * **Ce que c'est** : Le temps réel qui s'écoule, fourni par `requestAnimationFrame`. C'est le moteur, l'horloge qui fait avancer l'animation.
   * **Son Rôle** : Il sert de base pour calculer la progression globale de l'animation.

#### 2. Le Chef d'Orchestre : La Variable `phase`

Le lien entre le **Temps** et la **Distance** est une variable normalisée appelée `phase`. C'est le concept le plus important.

* **Définition** : `phase` est un nombre qui évolue de **0** (début de l'animation) à **1** (fin de l'animation).
* **Calcul** : `phase = (temps_ecoule) / duree_totale_animation`

La `duree_totale_animation` est directement proportionnelle à la `distance_totale_trace`. C'est le point de connexion crucial :
`duree_totale_animation = distance_totale_trace * coefficient_de_vitesse`

> **Conclusion** : Grâce à la `phase`, on a transformé le **temps qui passe** en un **pourcentage de progression** de l'animation. Un temps `t` correspond désormais à `x%` de l'animation totale.

#### 3. La Synchronisation en Pratique

Voici comment la `phase` est utilisée pour synchroniser le tracé et la caméra à chaque image :

##### A. Positionner le Marqueur sur le Tracé

1. On calcule la distance actuelle à parcourir sur le tracé :
   `distance_actuelle = distance_totale_trace * phase`

2. On utilise `turf.along()` pour trouver les coordonnées exactes du point situé à cette `distance_actuelle` sur le `LineString` du tracé.
   `comete_sur_trace = turf.along(trace, distance_actuelle)`

3. Ce `comete_sur_trace` est utilisé pour mettre à jour la position du marqueur au sol.

> **Flux** : `Temps -> phase -> Pourcentage de Distance -> Coordonnées sur le Tracé`

##### B. Positionner la Caméra

1. On utilise la même `distance_actuelle` calculée précédemment pour trouver où se situer dans le fichier de données de la caméra (`tracking`). Le tableau `tracking` agit comme une table de correspondance ("lookup table") indexée par la distance.

2. On identifie les deux "keyframes" de caméra (`tracking[i]` et `tracking[i+1]`) entre lesquels le coureur se trouve actuellement.

3. On calcule une **phase de segment** (`nbrSegment`), qui est la progression (de 0 à 1) *entre ces deux keyframes de caméra*.

4. On interpole pour trouver la position et l'orientation exactes de la caméra :
   
   * La position de la caméra est interpolée entre la `positionCamera` de `tracking[i]` et `tracking[i+1]`.
   * Le point regardé par la caméra est interpolé entre le `lookAt` de `tracking[i]` et `tracking[i+1]`.

5. Ces valeurs interpolées sont appliquées à la caméra via `map.setFreeCameraOptions()`.

> **Flux** : `Temps -> phase -> Pourcentage de Distance -> Index dans 'tracking' -> Interpolation entre les keyframes -> Position & Orientation de la Caméra`

#### 4. Le Traitement Spécifique des Variantes

Le mode `VisualizeVariantView.vue` introduit des étapes cruciales de synchronisation pour garantir la fluidité malgré la nature "dynamique" du tracé :

1.  **Reconstruction du Tracé (Frontend)** : La trace `fullLineString` est reconstruite en "snappant" les points de départ et d'arrivée de la variante sur la trace maîtresse haute résolution.
2.  **Lissage des Caps (`applySmoothingToTracking`)** : Les points générés pour les variantes peuvent avoir des caps brusques. Un lissage par moyenne vectorielle (fenêtre glissante) est appliqué sur le frontend pour éviter les saccades de rotation de la caméra.
3.  **Continuité des Splines (`nbrSegment`)** : Les valeurs de `nbrSegment` (utilisées pour l'interpolation spline) sont recalculées entre les points de contrôle lors de la suture des segments, assurant une transition douce entre la trace originale et la variante.
4.  **Correction de Distance** : Les distances du tracking backend sont recalibrées pour correspondre parfaitement à la géométrie JS calculée par Turf.js.

## 5. Comparaison des Écarts de Synchronisation

| Élément | VisualizeView (Standard) | VisualizeVariantView (Variante) |
| :--- | :--- | :--- |
| **Origine Trace** | Fichier `LineString.json` statique | Reconstruction dynamique (JS) |
| **Tracking** | `tracking.json` brut | Tracking lissé et recalculé (Frontend) |
| **Événements** | Chargés de `evt.json` | Désactivés (liste vide par défaut) |
| **Vitesse** | Basée sur les paramètres globaux | Ajustée selon la longueur réelle calculée |

## 6. Schéma Récapitulatif

```
  Temps (fourni par le navigateur)
    |
    v
+-------------------------------------------------------------+
|   phase (0 à 1)                                             |  <-- Le chef d'orchestre
| (Basée sur la durée, qui dépend de la DISTANCE totale)      |
+-------------------------------------------------------------+
    |
    +--------------------------------+---------------------------------------+
    |                                |                                       |
    v                                v                                       v
+---------------------------+      +-----------------------------------+     +-------------------------+
| Position sur le Tracé     |      | Position de la Caméra             |     | Style du Tracé          |
|---------------------------|      |-----------------------------------|     |-------------------------|
| distance = total * phase  |      | 1. Trouve l'index dans 'tracking' |     | Le gradient de la ligne |
| point = turf.along(tracé, |      |    via la `distance`.             |     | progresse avec la `phase`|
|          distance)        |      | 2. Interpole entre les points     |     +-------------------------+
+---------------------------+      |    clés de la caméra.             |
    |                              +-----------------------------------+
    v                                    |
+---------------------------+            v
| Mise à jour du marqueur   |      +--------------------------------+
| (source de données)       |      | Mise à jour de la caméra       |
+---------------------------+      | (setFreeCameraOptions)         |
                                   +--------------------------------+
```
