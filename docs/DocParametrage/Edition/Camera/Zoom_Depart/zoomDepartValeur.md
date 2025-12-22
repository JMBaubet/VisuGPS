# 🔎 Paramètre : Valeur du zoom au départ

Ce document détaille le paramètre `zoomDepartValeur`, qui définit le niveau de zoom appliqué au point de départ (Km 0) de la trace lorsque l'option `Activer le zoom au départ` est activée.

---

## 🎯 Rôle du Paramètre

Le paramètre `zoomDepartValeur` contrôle le niveau de zoom final atteint par la caméra après l'animation de dézoom au début du parcours.

-   **Libellé**: Valeur du zoom au départ
-   **Type**: Entier
-   **Valeur par défaut**: 18
-   **Minimum**: 10
-   **Maximum**: 22

## ⚖️ Justification : Pourquoi ajuster la valeur du zoom au départ ?

L'ajustement de cette valeur permet de contrôler la proximité de la vue au début de la trace.

### 1. 👀 Détail vs. Vue d'Ensemble

-   **Valeur élevée** (`> 18`) : La caméra reste relativement proche du point de départ, offrant une vue détaillée de l'environnement immédiat.
-   **Valeur faible** (`< 18`) : La caméra s'éloigne davantage, offrant une vue plus large du contexte du début de la trace.

### 2. 🏞️ Contexte Environnemental

Un zoom plus faible peut aider à visualiser le début de la trace par rapport aux éléments géographiques majeurs.

---

## ⚠️ Recommandations

-   **Valeur par défaut (18)** : C'est un bon compromis qui offre un équilibre entre le détail du point de départ et une vue d'ensemble suffisante.
-   **Adapter à la trace** :
    -   Pour des départs en zone urbaine dense, un zoom plus élevé peut être utile.
    -   Pour des départs en pleine nature ou si vous souhaitez une vue plus large, un zoom plus faible peut être préférable.
