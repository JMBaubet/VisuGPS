# ⏳ Paramètre : Durée de la reprise après pause (Visualisation)

Ce document détaille le paramètre `timerReprisePause`, qui définit la durée en millisecondes de l'animation de survol utilisée pour revenir à la position de la caméra après une pause dans l'animation principale.

---

## 🎯 Rôle du Paramètre

Le paramètre `timerReprisePause` contrôle la fluidité de la transition de la caméra lorsque l'animation reprend après avoir été mise en pause. Une valeur plus élevée rend la transition plus lente et douce, tandis qu'une valeur plus faible la rend plus rapide et abrupte.

-   **Libellé**: Durée de la reprise après pause (sec)
-   **Type**: Réel
-   **Valeur par défaut**: 1.0 sec
-   **Minimum**: 0.0 sec
-   **Maximum**: 10.0 sec
-   **Unité**: sec

## ⚖️ Justification : Pourquoi ajuster la durée de la reprise après pause ?

L'ajustement de cette durée est important pour l'expérience utilisateur, car il influence la perception de la fluidité et du contrôle de l'animation.

### 1. 🎥 Fluidité de la Transition

-   Une transition douce (valeur plus élevée) est plus agréable visuellement et moins déroutante pour l'utilisateur, surtout si la caméra a été déplacée manuellement pendant la pause.
-   Une transition instantanée (valeur de 0 ms) peut être préférée pour une reprise immédiate sans mouvement de caméra.

### 2. 🖐️ Expérience Utilisateur

-   Permet de personnaliser la réactivité de l'application après une interaction de pause/reprise.

---

## ⚠️ Recommandations

-   **Valeur par défaut (1000 ms)** : Une durée d'une seconde offre une transition douce et perceptible, sans être trop longue.
-   **Adapter aux préférences** :
    -   Si vous préférez une reprise instantanée, définissez la valeur à 0.
    -   Si vous souhaitez une transition très lente et cinématographique, augmentez la valeur.
