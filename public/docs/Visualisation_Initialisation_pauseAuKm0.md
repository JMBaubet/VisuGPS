# ⏸️ Paramètre : Pause au Km 0 (Initialisation Visualisation)

Ce document détaille le paramètre `pauseAuKm0`, qui définit la durée en secondes d'une pause au point de départ (Km 0) de la trace avant le démarrage automatique de l'animation. Cette pause s'applique à l'initialisation et à la réinitialisation de l'animation.

---

## 🎯 Rôle du Paramètre

Le paramètre `pauseAuKm0` introduit un court délai une fois que la caméra est positionnée au point de départ de la trace. Cette pause permet à l'utilisateur de se préparer au début du mouvement et d'apprécier le point de départ.

-   **Libellé**: Pause au Km 0 (sec)
-   **Type**: Réel (Float)
-   **Valeur par défaut**: 0.5 sec
-   **Minimum**: 0 sec
-   **Maximum**: 60 sec
-   **Pas (step)**: 0.5 sec
-   **Unité**: sec

## ⚖️ Justification : Pourquoi une pause au Km 0 ?

Une pause au point de départ est importante pour marquer le début de l'exploration de la trace et offrir un moment de préparation à l'utilisateur.

### 1. 👀 Préparation au Mouvement

-   Permet à l'utilisateur de s'habituer à la vue du point de départ avant que l'animation ne commence à se dérouler.
-   Évite un démarrage trop abrupt de l'animation.

### 2. 🎥 Effet Cinématique

-   Crée un moment de "suspense" ou de préparation avant le début de l'action.

---

## ⚠️ Recommandations

-   **Valeur par défaut (0.5 sec)** : Une demi-seconde est une courte pause qui marque le début sans ralentir excessivement l'introduction.
-   **Adapter aux préférences** : Vous pouvez ajuster cette durée si vous souhaitez une pause plus longue pour une meilleure immersion ou une pause plus courte pour un démarrage plus rapide.
