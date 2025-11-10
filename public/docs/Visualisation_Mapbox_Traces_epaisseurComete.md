# 📏 Paramètre : Épaisseur de la comète (Visualisation)

Ce document détaille le paramètre `epaisseurComete`, qui définit l'épaisseur en pixels de la "comète" (la trace lumineuse) qui suit l'avancement de la caméra le long de la trace GPX en mode visualisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `epaisseurComete` contrôle la largeur visuelle de la comète. Une épaisseur appropriée assure la visibilité de cet indicateur sans qu'il ne masque une trop grande partie de la trace ou des détails du fond de carte.

-   **Libellé**: Épaisseur de la comète
-   **Type**: Entier
-   **Valeur par défaut**: 8 px
-   **Minimum**: 1 px
-   **Maximum**: 20 px

## ⚖️ Justification : Pourquoi ajuster l'épaisseur de la comète ?

L'ajustement de l'épaisseur de la comète permet d'optimiser sa visibilité et son rôle en tant qu'indicateur clair de la progression.

### 1. 👀 Visibilité et Impact Visuel

-   Une épaisseur suffisante permet de bien suivre la progression de l'animation.
-   Une épaisseur trop fine pourrait la rendre difficile à percevoir, tandis qu'une épaisseur excessive pourrait devenir intrusive.

### 2. 🎨 Esthétique

-   Permet de personnaliser l'apparence de l'animation pour qu'elle corresponde mieux aux préférences de l'utilisateur ou à l'esthétique générale de la visualisation.

---

## ⚠️ Recommandations

-   **Valeur par défaut (8 px)** : C'est un bon compromis qui rend la comète bien visible sans être trop envahissante.
-   **Synchroniser avec l'épaisseur de la trace** : Souvent, une épaisseur similaire ou légèrement supérieure à celle de la trace principale offre une bonne cohérence et clarté.
-   **Tester sur différents fonds de carte** : Assurez-vous que la comète reste bien visible quel que soit le style de carte choisi.
