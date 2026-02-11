# 🔍 Paramètre : Marge autour de la trace (Début)

Ce document détaille le paramètre `margeLancement`, qui définit l'espace (padding) en pixels conservé autour de la trace lors du cadrage initial (vue globale de la trace) au lancement de l'animation.

---

## 🎯 Rôle du Paramètre

Le paramètre `margeLancement` contrôle le niveau de recul de la caméra lors de la vue d'ensemble du parcours au début de l'animation. Une marge plus élevée (valeur numérique plus grande) éloigne la caméra, créant plus d'espace vide autour de la trace, tandis qu'une marge plus faible (valeur numérique plus petite) rapproche la caméra, remplissant l'écran avec la trace.

-   **Libellé**: Marge autour de la trace (Début)
-   **Type**: Entier
-   **Unité**: px
-   **Valeur par défaut**: 80
-   **Minimum**: 20
-   **Maximum**: 140

## ⚖️ Justification : Pourquoi ajuster la marge initiale ?

L'ajustement de la marge permet d'optimiser la présentation visuelle du parcours avant le début du suivi détaillé.

### 1. 🖼️ Cadrage Esthétique

-   Une marge suffisante évite que la trace ne touche les bords de l'écran, offrant une composition plus équilibrée et professionnelle.
-   Elle permet de visualiser le contexte géographique immédiat autour du parcours.

### 2. 🛡️ Visibilité et Interface

-   Les widgets (commandes, météo, etc.) peuvent masquer une partie de la carte. Une marge plus importante garantit que la trace reste visible même en présence de ces éléments d'interface.

---

## ⚠️ Recommandations

-   **Valeur par défaut (80 px)** : Cette valeur offre un bon compromis, laissant respirer la trace tout en maximisant sa visibilité à l'écran.
-   **Écrans encombrés** : Si vous utilisez de nombreux widgets ou une interface dense, augmentez la marge (vers 100-120 px) pour éviter tout chevauchement.
-   **Immersion maximale** : Pour un effet plus immersif où la trace occupe tout l'espace disponible, réduisez la marge (vers 20-40 px).
