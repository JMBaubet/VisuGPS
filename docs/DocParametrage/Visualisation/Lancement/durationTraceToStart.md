# ⏳ Paramètre : Durée Trace vers Départ (Initialisation Visualisation)

Ce document détaille le paramètre `durationTraceToStart`, qui définit la durée en millisecondes de l'animation de survol qui va de la vue globale de la trace vers le point de départ (Km 0) de l'animation en mode visualisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `durationTraceToStart` contrôle la vitesse de la transition visuelle du zoom depuis la vue complète de la trace vers son point de départ. Une durée plus longue rend la transition plus lente et douce, préparant l'utilisateur au début de l'animation.

-   **Libellé**: Durée Trace vers Départ (ms)
-   **Type**: Entier
-   **Valeur par défaut**: 2000 ms
-   **Minimum**: 1000 ms
-   **Maximum**: 5000 ms
-   **Unité**: ms

## ⚖️ Justification : Pourquoi ajuster la durée de la transition Trace vers Départ ?

L'ajustement de cette durée est essentiel pour une introduction fluide et compréhensible de l'animation.

### 1. 🎥 Effet Cinématique

-   Une durée appropriée permet de guider l'œil de l'utilisateur vers le point de départ de l'animation de manière douce et progressive.
-   Une durée trop courte peut rendre la transition abrupte et potentiellement déroutante.

### 2. 👀 Préparation à l'Animation

-   Donne à l'utilisateur le temps de se préparer au début du mouvement le long de la trace.

---

## ⚠️ Recommandations

-   **Valeur par défaut (2000 ms)** : Deux secondes offrent une transition douce et suffisamment rapide pour ne pas prolonger inutilement l'introduction.
-   **Adapter aux préférences** : Vous pouvez augmenter ou diminuer cette durée selon l'effet cinématique souhaité.
