# 🔎 Paramètre : Niveau de zoom par défaut pour le tracking

Ce document détaille le paramètre `Zoom`, qui définit le niveau de zoom par défaut de la caméra lors de la visualisation d'une trace GPX en mode tracking.

---

## 🎯 Rôle du Paramètre

Le paramètre `Zoom` contrôle la distance à laquelle la caméra est positionnée par rapport à la trace. Un niveau de zoom plus élevé signifie que la caméra est plus proche de la trace, offrant une vue plus détaillée.

-   **Libellé**: Zoom
-   **Type**: Entier
-   **Valeur par défaut**: 16
-   **Minimum**: 0
-   **Maximum**: 22

## ⚖️ Justification : Pourquoi ajuster le niveau de zoom ?

Le niveau de zoom affecte directement la perspective et le détail de la visualisation de la trace.

### 1. 👀 Détail de la Trace

-   **Zoom élevé** (`> 16`) : Permet de voir les détails fins de la trace, les virages serrés, les intersections, et le terrain environnant de très près. Utile pour l'analyse détaillée.
-   **Zoom faible** (`< 16`) : Offre une vue plus large de la trace et de son environnement, utile pour comprendre le contexte général du parcours.

### 2. 🏞️ Contexte Environnemental

Un zoom plus faible peut aider à visualiser la trace par rapport aux éléments géographiques majeurs (villes, montagnes, rivières).

### 3. 🎥 Expérience de Visualisation

Le choix du zoom peut influencer la sensation de vitesse et d'immersion lors du tracking.

---

## ⚠️ Recommandations

-   **Valeur par défaut (16)** : C'est un bon compromis qui offre un équilibre entre le détail de la trace et la vue d'ensemble de l'environnement immédiat.
-   **Adapter à la trace** :
    -   Pour des traces très détaillées ou en milieu urbain, un zoom plus élevé peut être préférable.
    -   Pour des traces en pleine nature ou très longues, un zoom légèrement plus faible peut donner une meilleure perspective.
