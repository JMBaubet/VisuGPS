# 📏 Paramètre : Largeur de la trace sur la vignette (px)

Ce document détaille le paramètre `largeurTrace`, qui contrôle l'épaisseur en pixels de la ligne représentant la trace GPX sur la vignette 2D générée lors de l'importation.

---

## 🎯 Rôle du Paramètre

Le paramètre `largeurTrace` définit l'épaisseur de la ligne de la trace GPX affichée sur la miniature.

-   **Libellé**: Largeur de la trace
-   **Type**: Entier
-   **Valeur par défaut**: 3 px
-   **Minimum**: 1 px
-   **Maximum**: 20 px
-   **Unité**: px (pixels)

## ⚖️ Justification : Pourquoi ajuster la largeur de la trace ?

L'ajustement de la largeur de la trace est une question de visibilité et d'esthétique.

### 1. 👀 Visibilité

-   Une trace plus épaisse (`> 3 px`) peut être plus facile à voir sur des fonds de carte complexes ou si la vignette est de petite taille.
-   Une trace très fine (`1 px`) peut être plus discrète et élégante, mais potentiellement moins visible.

### 2. 🖼️ Esthétique

Le choix de l'épaisseur peut contribuer à l'esthétique générale de la vignette. Une trace trop épaisse peut paraître grossière, tandis qu'une trace trop fine peut se perdre.

---

## ⚠️ Recommandations

-   **Valeur par défaut (3 px)** : C'est un bon compromis qui offre une bonne visibilité sans surcharger la vignette.
-   **Adapter au fond de carte** : Si le fond de carte est très détaillé, une trace légèrement plus épaisse peut aider à la distinguer.
-   **Adapter à la taille de la vignette** : Pour des vignettes de très petite taille, une trace plus fine peut être préférable.
