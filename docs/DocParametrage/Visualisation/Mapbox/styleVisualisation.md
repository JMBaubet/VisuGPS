# 🗺️ Paramètre : Style de carte pour la visualisation (Mapbox)

Ce document détaille le paramètre `styleVisualisation`, qui définit le style de carte utilisé par Mapbox lors de la visualisation d'une trace.

---

## 🎯 Rôle du Paramètre

Le paramètre `styleVisualisation` permet de choisir un style de carte Mapbox parmi une liste prédéfinie. Cela permet de personnaliser l'apparence visuelle du fond de carte affiché pendant l'animation de la trace.

-   **Libellé**: Style de la carte
-   **Type**: Liste de sélection
-   **Valeur par défaut**: "mapbox://styles/mapbox/satellite-v9"

## ⚖️ Justification : Pourquoi personnaliser le style de la carte ?

La personnalisation du style de la carte permet d'adapter l'environnement visuel aux préférences de l'utilisateur ou aux spécificités de la trace visualisée.

### 1. 👀 Esthétique et Immersion

-   Certains styles sont plus adaptés pour une immersion visuelle (satellite), d'autres pour une meilleure lisibilité des noms de lieux (streets), ou pour des styles plus artistiques.
-   Le choix du style peut améliorer l'expérience de visualisation de l'animation.

### 2. 🗺️ Contexte Géographique

-   Permet de choisir un style de carte qui met en valeur les éléments géographiques pertinents pour la visualisation (par exemple, un style satellite pour le relief, un style routier pour les itinéraires).

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

-   **Valeur par défaut ("mapbox://styles/mapbox/satellite-v9")** : C'est un style satellite qui offre une bonne base pour la visualisation des traces, car il met en évidence le relief et les détails du terrain.
-   **Choisir un style pertinent** : Sélectionnez un style de la liste qui correspond à l'ambiance ou au type de visualisation que vous souhaitez créer.
