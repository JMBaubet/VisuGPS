# ⏳ Paramètre : Durée Europe vers Trace (Initialisation Visualisation)

Ce document détaille le paramètre `durationEuropeToTrace`, qui définit la durée en millisecondes de l'animation "flyTo" qui va de la vue générale de l'Europe vers la vue globale de la trace, au début de la visualisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `durationEuropeToTrace` contrôle la vitesse de la transition visuelle du zoom depuis l'Europe vers la trace complète. Une durée plus longue rend la transition plus lente et douce, renforçant l'effet cinématographique.

-   **Libellé**: Durée Europe vers Trace (ms)
-   **Type**: Entier
-   **Valeur par défaut**: 5000 ms
-   **Minimum**: 1000 ms
-   **Maximum**: 10000 ms
-   **Unité**: ms

## ⚖️ Justification : Pourquoi ajuster la durée de la transition Europe vers Trace ?

L'ajustement de cette durée est essentiel pour créer une introduction à l'animation qui est à la fois informative et esthétiquement agréable.

### 1. 🎥 Effet Cinématique

-   Une durée appropriée permet de mettre en scène l'introduction de la trace en la faisant apparaître progressivement depuis une vue large, créant un effet "zoom-in" doux.
-   Une durée trop courte peut rendre la transition abrupte et moins agréable visuellement.

### 2. 👀 Immersion Utilisateur

-   Une transition fluide contribue à l'immersion de l'utilisateur en lui donnant le temps d'assimiler le contexte géographique.

---

## ⚠️ Recommandations

-   **Valeur par défaut (5000 ms)** : Cinq secondes offrent une transition douce et suffisamment longue pour apprécier l'effet de zoom progressif.
-   **Adapter aux préférences** : Vous pouvez augmenter ou diminuer cette durée selon l'effet cinématique souhaité.
