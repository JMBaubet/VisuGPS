# Configuration des Vignettes

VisuGPS génère automatiquement des images miniatures (vignettes) pour chaque circuit importé. Ce document détaille les paramètres personnalisables pour adapter le rendu de ces images.

[< Retour aux Paramètres](./parametres.md)

## Accès aux Paramètres
Ces réglages sont accessibles dans le menu **Paramètres > Importation > Vignette**.

> **Note** : Les modifications de ces paramètres s'appliqueront uniquement aux **nouvelles importations** de circuits. Pour modifier la vignette d'un circuit existant, vous devez le supprimer et le réimporter.

## Vignette

Ce groupe principal contient les réglages de base de la carte.

*   **<span style="color: #2196F3">Style de la vignette</span>** : Choix du fond de plan via une liste prédéfinie (Rues, Satellite, Outdoor, Sombre, etc.). Les différents styles peuvent être visualisés directement sur le site de Mapbox. (voir [Documentation Mapbox Styles](https://docs.mapbox.com/api/maps/styles/#classic-mapbox-styles))
    *   *Défaut : mapbox://styles/mapbox/streets-v12*

### Dimensions

*   **<span style="color: #2196F3">Largeur de la vignette</span>** : Largeur en pixels de l'image générée.
    *   *Minimum : 400px / Maximum : 1280px*
*   **<span style="color: #2196F3">Format de la vignette</span>** : Ratio d'aspect de l'image.
    *   *Options : 1/1 (Carré), 4/3, 16/9 (Panoramique)*

### Trace

Ces options modifient le dessin du parcours sur la carte :

*   **<span style="color: #2196F3">Couleur de la trace sur la vignette</span>** : Couleur de la ligne du parcours.
    *   *Défaut : Orange foncé*
*   **<span style="color: #2196F3">Largeur de la trace</span>** : Épaisseur du trait en pixels.
    *   *Défaut : 3px*

### MarqueurDistance

Ces indicateurs permettent de visualiser le sens de parcours, ce qui est particulièrement utile pour les circuits en boucle (fermés).

*   **<span style="color: #2196F3">Afficher la distance</span>** : Active ou désactive l'affichage des bornes kilométriques.
*   **<span style="color: #2196F3">Intervalle distance</span>** : Distance en km entre deux marqueurs.
    *   *Exemple : Tous les 10 km.*
*   **<span style="color: #2196F3">Couleur des marqueurs de distance</span>** : Couleur de base des pastilles.

> **Note - Limitations techniques** :
> * Sur la vignette, chaque marqueur est limité à **un seul caractère** (ex: 1, 2, ... 9, 0). Pour les parcours de plus de 100 km (par défaut), la saturation de la couleur change tous les 10 marqueurs (du plus clair au plus foncé) pour permettre de différencier les dizaines.

### DepartArrivee

Ces marqueurs permettent de visualiser le sens du parcours pour les circuits "ouverts" (point-à-point) et d'identifier clairement les points de départ et d'arrivée pour les circuits en boucle.

*   **<span style="color: #2196F3">Afficher les marqueurs</span>** : Active ou désactive les icônes de Départ et d'Arrivée.
*   **<span style="color: #2196F3">Distance max départ/arrivée (m)</span>** : Seuil en mètres pour considérer le parcours comme une boucle. Si la distance entre le départ et l'arrivée est inférieure à ce seuil, un marqueur unique est affiché.
*   **<span style="color: #2196F3">Couleur du marqueur de départ</span>** : Couleur du marqueur de début.
    1. *Défaut : Vert*
*   **<span style="color: #2196F3">Couleur du marqueur d'arrivée</span>** : Couleur du marqueur de fin.
    *   *Défaut : Rouge*
*   **<span style="color: #2196F3">Couleur du marqueur départ/arrivée (proches)</span>** : Couleur du marqueur unique pour les boucles.
    *   *Défaut : Bleu*

---
[< Retour aux Paramètres](./parametres.md) | [< Retour à l'Importation](./upload.md)
