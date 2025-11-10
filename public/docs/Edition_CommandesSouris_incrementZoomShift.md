# 🖱️ Paramètre : Incrément de zoom (Shift + molette de la souris)

Ce document détaille le paramètre `incrementZoomShift`, qui définit l'incrément de zoom de la caméra lorsque la molette de la souris est utilisée en combinaison avec la touche `Shift` en mode édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `incrementZoomShift` offre un moyen d'accélérer les ajustements du niveau de zoom de la caméra. Il permet de modifier le zoom par pas plus importants que l'incrément standard.

-   **Libellé**: Incrément Zoom (Shift)
-   **Type**: Réel (Float)
-   **Valeur par défaut**: 1.0
-   **Minimum**: 0.1
-   **Maximum**: 5.0
-   **Décimales**: 2

## ⚖️ Justification : Pourquoi un incrément de zoom avec Shift ?

L'utilisation de la touche `Shift` pour modifier l'incrément de zoom est une convention ergonomique courante pour les actions "rapides".

### 1. 🚀 Rapidité de Réglage

-   Permet de modifier rapidement le niveau de zoom de la caméra sans avoir à changer les paramètres, idéal pour des ajustements grossiers ou pour trouver rapidement une perspective générale.

### 2. ⚡ Gain de Temps

-   Réduit le nombre de mouvements de molette nécessaires pour atteindre un niveau de zoom désiré.

### 3. 🤔 Flexibilité

-   Offre une flexibilité à l'utilisateur, lui permettant de choisir entre un ajustement précis et un ajustement rapide selon le contexte.

---

## ⚠️ Recommandations

-   **Valeur par défaut (1.0)** : C'est un bon compromis qui offre un ajustement significativement plus rapide que l'incrément standard.
-   **Adapter aux besoins** :
    -   Si vous avez besoin de faire des ajustements très rapides et importants du zoom, vous pouvez augmenter cette valeur.
    -   Si un ajustement légèrement plus rapide que l'incrément standard est suffisant, vous pouvez la réduire.
