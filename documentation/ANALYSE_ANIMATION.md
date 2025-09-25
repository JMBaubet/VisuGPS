### Analyse de la Synchronisation entre Tracé, Caméra, Temps et Distance

Ce document explique comment les différents éléments de l'animation "flyby" sont synchronisés. Comprendre cette relation est essentiel pour reproduire ou modifier l'effet.

#### 1. Les Trois Piliers de l'Animation

L'animation repose sur la synchronisation de trois concepts fondamentaux :

1. **Le Tracé au Sol (`LineString.json`)** :
   
   * **Ce que c'est** : Une simple liste de coordonnées `[lon, lat, elevation]` qui représente le parcours physique. )`elevation` ne semble pas utile)
   * **Son Unité de Mesure** : La **distance** (en kilomètres ou mètres). C'est la "vérité terrain". Chaque point du tracé est situé à une distance `d` du point de départ.

2. **Le Chemin de la Caméra (`tracking.json`)** :
   
   * **Ce que c'est** : Une liste de "points de contrôle" ou "keyframes" quand `pointDeControl = true`pour la caméra. Ce n'est PAS un simple tracé. Chaque point de contrôle définit un état complet de la caméra (`position`, `point regardé`, `altitude`) (`coordonneeCamera`, `coordonnee`, `altitudeCamera`) à un moment précis.
   * **Son Lien au Tracé** : Chaque point de contrôle de la caméra est **mappé à une distance spécifique sur le tracé au sol**. Par exemple, "quand le coureur est à 2.5km, la caméra doit être à telle position et regarder tel point".

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

## 4. Schéma Récapitulatif

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
