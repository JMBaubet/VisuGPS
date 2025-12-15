# 🗺️ Paramètre : Style de carte pour la création des vignettes

Ce document détaille le paramètre `styleVignette`, qui définit le style de carte Mapbox utilisé pour générer les vignettes 2D des traces GPX.

---

## 🎯 Rôle du Paramètre

Le paramètre `styleVignette` permet de choisir un style de carte Mapbox parmi une liste prédéfinie pour l'apparence du fond de carte sur lequel la trace GPX sera dessinée pour créer la vignette.

-   **Libellé**: Style de la vignette
-   **Type**: Liste de sélection
-   **Valeur par défaut**: "mapbox://styles/mapbox/streets-v12"

## ⚖️ Justification : Pourquoi choisir un style de carte ?

Le choix du style de carte est crucial pour la lisibilité et l'esthétique des vignettes.

### 1. 👀 Lisibilité de la Trace

-   Certains styles de carte (comme `streets-v12`) sont clairs et mettent en évidence les routes et les points d'intérêt, ce qui peut aider à situer la trace.
-   D'autres styles (comme `satellite-v9`) offrent une vue satellite, utile pour visualiser le terrain réel.

### 2. 🎨 Esthétique

Le style de carte contribue à l'aspect général de la vignette. Vous pouvez choisir un style qui correspond à vos préférences visuelles.

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

-   **Valeur par défaut ("mapbox://styles/mapbox/streets-v12")** : C'est un style polyvalent qui offre une bonne lisibilité de la trace sur un fond de carte clair.
-   **Expérimenter** : N'hésitez pas à essayer d'autres styles Mapbox de la liste pour voir celui qui convient le mieux à vos besoins.
