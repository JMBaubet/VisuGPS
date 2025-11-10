# 📏 Paramètre : Épaisseur de l'avancement (Édition)

Ce document détaille le paramètre `epaisseurAvancement`, qui définit l'épaisseur en pixels de la ligne d'avancement de la caméra sur la carte en mode édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `epaisseurAvancement` contrôle la largeur visuelle de la ligne qui indique la portion de la trace déjà parcourue par la caméra, influençant sa visibilité et son impact visuel.

-   **Libellé**: Épaisseur de l'avancement
-   **Type**: Entier
-   **Valeur par défaut**: 5 px
-   **Minimum**: 1 px
-   **Maximum**: 20 px
-   **Unité**: px (pixels)

## ⚖️ Justification : Pourquoi ajuster l'épaisseur de l'avancement ?

L'ajustement de l'épaisseur de la ligne d'avancement permet de trouver un équilibre entre sa visibilité et sa discrétion sur la carte.

### 1. 👀 Visibilité et Distinction

-   Une épaisseur plus grande rend la ligne d'avancement plus visible, ce qui peut être utile pour la faire ressortir sur des fonds de carte complexes ou par rapport à la trace principale.
-   Une épaisseur plus petite la rend plus discrète, évitant qu'elle ne masque les détails du fond de carte ou de la trace.

### 2. 🎨 Esthétique

-   Permet de personnaliser l'apparence de la ligne d'avancement selon les préférences de l'utilisateur.

---

## ⚠️ Recommandations

-   **Valeur par défaut (5 px)** : C'est un bon compromis qui offre une ligne d'avancement visible sans être trop imposante.
-   **Adapter aux préférences** : Vous pouvez augmenter l'épaisseur si vous souhaitez une ligne plus prononcée, ou la réduire si vous préférez un style plus minimaliste.
