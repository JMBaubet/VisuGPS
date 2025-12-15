# ⏳ Paramètre : Durée du survol global (Finalisation Visualisation)

Ce document détaille le paramètre `flyToGlobalDuration`, qui définit la durée en millisecondes de l'animation "flyTo" qui ramène la caméra à une vue globale de la trace après la fin de l'animation principale.

---

## 🎯 Rôle du Paramètre

Le paramètre `flyToGlobalDuration` contrôle la vitesse de la transition visuelle qui, après la fin de l'animation, ramène la caméra à une vue d'ensemble de la trace complète. Une durée plus longue rend la transition plus lente et douce.

-   **Libellé**: Durée du survol global (ms)
-   **Type**: Entier
-   **Valeur par défaut**: 2000 ms
-   **Minimum**: 500 ms
-   **Maximum**: 5000 ms
-   **Unité**: ms

## ⚖️ Justification : Pourquoi ajuster la durée du survol global ?

L'ajustement de cette durée est essentiel pour une finalisation fluide et agréable de l'animation, offrant une vue récapitulative du parcours.

### 1. 🎥 Effet Cinématique

-   Une durée appropriée permet de créer un effet de "zoom-out" doux, offrant une perspective finale sur l'ensemble de la trace.
-   Une durée trop courte peut rendre la transition abrupte et moins agréable visuellement.

### 2. 👀 Récapitulatif Visuel

-   Permet à l'utilisateur de revoir l'intégralité de la trace après l'avoir parcourue virtuellement.

---

## ⚠️ Recommandations

-   **Valeur par défaut (2000 ms)** : Deux secondes offrent une transition douce et suffisamment rapide pour ne pas prolonger inutilement la finalisation.
-   **Adapter aux préférences** : Vous pouvez augmenter ou diminuer cette durée selon l'effet cinématique souhaité.
