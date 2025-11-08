# 🎨 Paramètre : Couleur de la trace GPX sur la vignette

Ce document détaille le paramètre `colorGPXVignette`, qui contrôle la couleur de la trace GPX affichée sur la vignette 2D générée lors de l'importation.

---

## 🎯 Rôle du Paramètre

Le paramètre `colorGPXVignette` définit la couleur de la ligne représentant la trace GPX sur la miniature.

-   **Libellé**: Couleur de la trace sur la vignette
-   **Type**: Couleur (Material Design)
-   **Valeur par défaut**: "orange-darken-4"

## ⚖️ Justification : Pourquoi choisir une couleur pour la trace ?

Le choix de la couleur de la trace est principalement esthétique et vise à améliorer la visibilité de la trace sur le fond de carte de la vignette.

### 1. 👀 Visibilité et Contraste

-   Une couleur bien contrastée par rapport au fond de carte (qui est généralement un style de carte Mapbox comme "streets-v12") rend la trace plus facile à distinguer.
-   La valeur par défaut "orange-darken-4" offre un bon contraste sur la plupart des fonds de carte clairs.

### 2. 🎨 Cohérence Visuelle

Vous pouvez choisir une couleur qui correspond à vos préférences personnelles ou qui s'harmonise avec d'autres éléments visuels de l'application.

---

## ⚠️ Recommandations

-   **Choisir une couleur contrastée** : Assurez-vous que la couleur choisie offre un bon contraste avec le fond de carte pour une meilleure lisibilité de la trace.
-   **Utiliser les couleurs Material Design** : Le paramètre accepte les noms de couleurs définis par Material Design (par exemple, "red", "blue-lighten-2", "green-darken-4").
