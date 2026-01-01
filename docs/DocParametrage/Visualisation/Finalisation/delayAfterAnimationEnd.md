# ⏳ Paramètre : Délai après fin d'animation (Visualisation)

Ce document détaille le paramètre `delayAfterAnimationEnd`, qui définit le délai en secondes avant de lancer la séquence de finalisation (par exemple, le retour à la vue globale de la trace) après la fin de l'animation principale.

---

## 🎯 Rôle du Paramètre

Le paramètre `delayAfterAnimationEnd` introduit une courte pause après que l'animation ait atteint la fin de la trace. Ce délai permet à l'utilisateur de visualiser le point d'arrivée final avant que la caméra ne commence à se déplacer pour la séquence de finalisation.

-   **Libellé**: Durée de la pause à l'arrivée
-   **Type**: Réel
-   **Valeur par défaut**: 3.0 sec
-   **Minimum**: 0.0 sec
-   **Maximum**: 10.0 sec

## ⚖️ Justification : Pourquoi un délai après la fin de l'animation ?

Un délai après la fin de l'animation est important pour marquer la conclusion du parcours et offrir un moment de contemplation du point d'arrivée.

### 1. 👀 Contemplation du Point d'Arrivée

-   Permet à l'utilisateur de s'attarder sur la fin de la trace et d'apprécier le point d'arrivée avant que la vue ne change.

### 2. 🎥 Fluidité de la Transition

-   Assure une transition douce vers la séquence de finalisation, évitant une coupure abrupte de l'animation.

---

## ⚠️ Recommandations

-   **Valeur par défaut (3 secondes)** : Trois secondes offrent une pause suffisante pour apprécier la fin de la trace sans prolonger excessivement l'attente.
-   **Adapter aux préférences** : Vous pouvez ajuster cette durée si vous souhaitez une pause plus longue ou plus courte.
