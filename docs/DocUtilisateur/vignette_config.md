# Configuration des Vignettes

VisuGPS génère automatiquement des images miniatures (vignettes) pour chaque circuit importé. Ce document détaille les paramètres personnalisables pour adapter le rendu de ces images.

[< Retour aux Paramètres](./parametres.md)

## Accès aux Paramètres
Ces réglages sont accessibles dans le menu **Paramètres > Importation > Vignette**.

> **Note** : Les modifications de ces paramètres s'appliqueront uniquement aux **nouvelles importations** de circuits. Pour modifier la vignette d'un circuit existant, vous devez le supprimer et le réimporter.

## 1. Style et Dimensions

### Fond de Carte
*   **Style de la vignette** : Choix du fond de plan Mapbox (Rues, Satellite, Outdoor, Sombre, etc.).
    *   *Défaut : mapbox://styles/mapbox/streets-v12*

### Dimensions
*   **Largeur de la vignette** : Largeur en pixels de l'image générée.
    *   *Minimum : 400px / Maximum : 1280px*
*   **Format de la vignette** : Ratio d'aspect de l'image.
    *   *Options : 1/1 (Carré), 4/3, 16/9 (Panoramique)*

## 2. Apparence de la Trace

Ces options modifient le dessin du parcours sur la carte :

*   **Couleur de la trace** : Couleur de la ligne du parcours.
    *   *Défaut : Orange foncé*
*   **Largeur de la trace** : Épaisseur du trait en pixels.
    *   *Défaut : 3px*

## 3. Marqueurs de Distance

Vous pouvez afficher des indicateurs kilométriques sur la vignette :

*   **Afficher la distance** : Active ou désactive l'affichage des bornes kilométriques.
*   **Intervalle distance** : Distance en km entre deux marqueurs.
    *   *Exemple : Tous les 10 km.*
*   **Couleur des marqueurs** : Couleur des pastilles indiquant les kilomètres.

## 4. Marqueurs Départ / Arrivée

Pour visualiser le sens du parcours :

*   **Afficher les marqueurs** : Active ou désactive les icônes de Départ (D) et d'Arrivée (A).
*   **Couleur du Départ** : Couleur du marqueur de début.
    *   *Défaut : Vert*
*   **Couleur de l'Arrivée** : Couleur du marqueur de fin.
    *   *Défaut : Rouge*
*   **Distance max départ/arrivée** : Seuil en mètres pour considérer le parcours comme une boucle. Si la distance entre le départ et l'arrivée est inférieure à ce seuil, un marqueur unique (D/A) est affiché.
*   **Couleur Départ/Arrivée (Boucle)** : Couleur du marqueur unique pour les boucles.
    *   *Défaut : Bleu*

---
[< Retour aux Paramètres](./parametres.md) | [< Retour à l'Importation](./upload.md)
