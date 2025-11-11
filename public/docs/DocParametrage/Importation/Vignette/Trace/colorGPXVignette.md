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

---

## 🌈 Annexe : Palette de Couleurs Material Design

Pour le paramètre `colorGPXVignette`, vous pouvez utiliser les noms de couleurs définis par la spécification Material Design. Chaque couleur est disponible en plusieurs nuances (lighten, darken) et en variantes d'accentuation (accent).

Voici une liste non exhaustive des couleurs principales et de leurs suffixes de nuance :

| Couleur Principale | Nuances (Exemples)                               |
| :----------------- | :----------------------------------------------- |
| `red`              | `red-lighten-5` à `red-darken-4`                 |
| `pink`             | `pink-lighten-5` à `pink-darken-4`               |
| `purple`           | `purple-lighten-5` à `purple-darken-4`           |
| `deep-purple`      | `deep-purple-lighten-5` à `deep-purple-darken-4` |
| `indigo`           | `indigo-lighten-5` à `indigo-darken-4`           |
| `blue`             | `blue-lighten-5` à `blue-darken-4`               |
| `light-blue`       | `light-blue-lighten-5` à `light-blue-darken-4`   |
| `cyan`             | `cyan-lighten-5` à `cyan-darken-4`               |
| `teal`             | `teal-lighten-5` à `teal-darken-4`               |
| `green`            | `green-lighten-5` à `green-darken-4`             |
| `light-green`      | `light-green-lighten-5` à `light-green-darken-4` |
| `lime`             | `lime-lighten-5` à `lime-darken-4`               |
| `yellow`           | `yellow-lighten-5` à `yellow-darken-4`           |
| `amber`            | `amber-lighten-5` à `amber-darken-4`             |
| `orange`           | `orange-lighten-5` à `orange-darken-4`           |
| `deep-orange`      | `deep-orange-lighten-5` à `deep-orange-darken-4` |
| `brown`            | `brown-lighten-5` à `brown-darken-4`             |
| `grey`             | `grey-lighten-5` à `grey-darken-4`               |
| `blue-grey`        | `blue-grey-lighten-5` à `blue-grey-darken-4`     |
| `black`            |                                                  |
| `white`            |                                                  |

**Exemples d'utilisation :**
- `red`
- `blue-lighten-2`
- `green-darken-4`
- `purple-accent-3` (pour les couleurs d'accentuation)

Pour une liste complète et des exemples visuels, veuillez consulter la [documentation officielle des couleurs de Vuetify](https://vuetifyjs.com/en/styles/colors/) ou la documentation de Material Design.