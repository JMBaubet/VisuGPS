# 🔎 Paramètre : Valeur du zoom à l'arrivée

Ce document détaille le paramètre `zoomArriveeValeur`, qui définit le niveau de zoom appliqué au point d'arrivée de la trace lorsque l'option `Activer le zoom à l'arrivée` est activée.

---

## 🎯 Rôle du Paramètre

Le paramètre `zoomArriveeValeur` contrôle le niveau de zoom final atteint par la caméra après l'animation de zoom à l'approche de la fin du parcours.

-   **Libellé**: Valeur du zoom à l'arrivée
-   **Type**: Entier
-   **Valeur par défaut**: 18
-   **Minimum**: 10
-   **Maximum**: 22

## ⚖️ Justification : Pourquoi ajuster la valeur du zoom à l'arrivée ?

L'ajustement de cette valeur permet de contrôler la proximité de la vue à la fin de la trace.

### 1. 👀 Détail vs. Vue d'Ensemble

-   **Valeur élevée** (`> 18`) : La caméra reste relativement proche du point d'arrivée, offrant une vue détaillée de l'environnement immédiat.
-   **Valeur faible** (`< 18`) : La caméra s'éloigne davantage, offrant une vue plus large du contexte de la fin de la trace.

### 2. 🏞️ Contexte Environnemental

Un zoom plus faible peut aider à visualiser la fin de la trace par rapport aux éléments géographiques majeurs.

---

## ⚠️ Recommandations

-   **Valeur par défaut (18)** : C'est un bon compromis qui offre un équilibre entre le détail du point d'arrivée et une vue d'ensemble suffisante.
-   **Adapter à la trace** :
    -   Pour des arrivées en zone urbaine dense, un zoom plus élevé peut être utile.
    -   Pour des arrivées en pleine nature ou si vous souhaitez une vue plus large, un zoom plus faible peut être préférable.
