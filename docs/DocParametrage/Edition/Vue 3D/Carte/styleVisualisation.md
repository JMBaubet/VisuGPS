# 🗺️ Paramètre : Style de carte pour l'édition (Mapbox)

Ce document détaille le paramètre `styleVisualisation`, qui définit le style de carte utilisé par Mapbox lors de la visualisation de la trace en mode édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `styleVisualisation` permet de choisir un style de carte Mapbox parmi une liste prédéfinie. Cela permet de personnaliser l'apparence visuelle du fond de carte affiché dans l'interface d'édition.

-   **Libellé**: Style de la carte
-   **Type**: Liste de sélection
-   **Valeur par défaut**: "mapbox://styles/mapbox/satellite-v9"

## ⚖️ Justification : Pourquoi personnaliser le style de la carte ?

La personnalisation du style de la carte permet d'adapter l'environnement visuel aux préférences de l'utilisateur ou aux spécificités de la trace éditée.

### 1. 👀 Esthétique et Lisibilité

-   Certains styles sont plus adaptés pour la visualisation du terrain (satellite), d'autres pour les routes (streets), ou pour des styles plus abstraits.
-   Le choix du style peut améliorer la lisibilité de la trace et des points de contrôle.

### 2. 🗺️ Contexte Géographique

-   Permet de choisir un style de carte qui met en valeur les éléments géographiques pertinents pour l'édition (par exemple, un style satellite pour le relief, un style routier pour les itinéraires).

### 3. 🌐 Styles Disponibles

Les styles suivants sont disponibles :
- `mapbox://styles/mapbox/standard`
- `mapbox://styles/mapbox/streets-v12`
- `mapbox://styles/mapbox/outdoors-v12`
- `mapbox://styles/mapbox/light-v11`
- `mapbox://styles/mapbox/dark-v11`
- `mapbox://styles/mapbox/satellite-v9`
- `mapbox://styles/mapbox/satellite-streets-v12`
- `mapbox://styles/mapbox/navigation-day-v1`
- `mapbox://styles/mapbox/navigation-night-v1`

---

## ⚠️ Recommandations

-   **Valeur par défaut ("mapbox://styles/mapbox/satellite-v9")** : C'est un style satellite qui offre une bonne base pour l'édition de traces, car il met en évidence le relief et les détails du terrain.
-   **Choisir un style pertinent** : Sélectionnez un style de la liste qui facilite votre travail d'édition.
