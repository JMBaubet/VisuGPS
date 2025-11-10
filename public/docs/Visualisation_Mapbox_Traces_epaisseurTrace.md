# 📏 Paramètre : Épaisseur de la trace (Visualisation)

Ce document détaille le paramètre `epaisseurTrace`, qui définit l'épaisseur en pixels de la trace GPX affichée sur la carte en mode visualisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `epaisseurTrace` contrôle la largeur visuelle de la ligne représentant la trace GPX, influençant sa visibilité et son impact visuel pendant l'animation.

-   **Libellé**: Épaisseur de la trace
-   **Type**: Entier
-   **Valeur par défaut**: 8 px
-   **Minimum**: 1 px
-   **Maximum**: 20 px

## ⚖️ Justification : Pourquoi ajuster l'épaisseur de la trace en visualisation ?

L'ajustement de l'épaisseur de la trace permet de trouver un équilibre entre sa visibilité et sa discrétion sur la carte, en particulier pendant l'animation.

### 1. 👀 Visibilité et Distinction

-   Une épaisseur plus grande rend la trace plus visible sur le fond de carte, ce qui est crucial pendant une animation où l'attention est portée sur le mouvement.
-   Une épaisseur plus petite peut la rendre plus discrète, mais aussi plus difficile à suivre visuellement.

### 2. 🎨 Esthétique

-   Permet de personnaliser l'apparence de la trace selon les préférences de l'utilisateur.

---

## ⚠️ Recommandations

-   **Valeur par défaut (8 px)** : C'est un bon compromis qui offre une trace bien visible pour le suivi de l'animation sans être trop dominante.
-   **Adapter aux préférences** : Vous pouvez augmenter l'épaisseur si vous souhaitez une trace plus prononcée, ou la réduire si vous préférez une apparence plus subtile (mais attention à ne pas la rendre invisible lors de l'animation).
