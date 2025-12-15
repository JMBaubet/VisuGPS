# ⌨️ Paramètre : Incrément du pitch (Shift + flèches haut/bas)

Ce document détaille le paramètre `incrementPitchShift`, qui définit l'incrément en degrés pour l'angle d'inclinaison (pitch) de la caméra lorsque les flèches haut/bas du clavier sont utilisées en combinaison avec la touche `Shift`.

---

## 🎯 Rôle du Paramètre

Le paramètre `incrementPitchShift` offre un moyen d'accélérer les ajustements de l'angle de la caméra (pitch). Il permet de modifier le pitch par pas plus importants que l'incrément standard.

-   **Libellé**: Incrément Pitch (Shift)
-   **Type**: Entier
-   **Valeur par défaut**: 5°
-   **Minimum**: 1°
-   **Maximum**: 20°
-   **Unité**: ° (degrés)

## ⚖️ Justification : Pourquoi un incrément de pitch avec Shift ?

L'utilisation de la touche `Shift` pour modifier l'incrément de pitch est une convention ergonomique courante pour les actions "rapides".

### 1. 🚀 Rapidité de Réglage

-   Permet de modifier rapidement l'angle de la caméra sans avoir à changer les paramètres, idéal pour des ajustements grossiers ou pour trouver rapidement une perspective générale.

### 2. ⚡ Gain de Temps

-   Réduit le nombre de pressions sur les touches nécessaires pour atteindre un angle de pitch désiré.

### 3. 🤔 Flexibilité

-   Offre une flexibilité à l'utilisateur, lui permettant de choisir entre un ajustement précis et un ajustement rapide selon le contexte.

---

## ⚠️ Recommandations

-   **Valeur par défaut (5°)** : C'est un bon compromis qui offre un ajustement significativement plus rapide que l'incrément standard.
-   **Adapter aux besoins** :
    -   Si vous avez besoin de faire des ajustements très rapides et importants du pitch, vous pouvez augmenter cette valeur.
    -   Si un ajustement légèrement plus rapide que l'incrément standard est suffisant, vous pouvez la réduire.
