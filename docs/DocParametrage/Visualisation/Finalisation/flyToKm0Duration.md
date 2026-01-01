# ⏳ Paramètre : Durée du survol vers Km 0 (Visualisation)

Ce document détaille le paramètre `flyToKm0Duration`, qui définit la durée en secondes de l'animation de survol qui ramène la caméra vers le point de départ (Km 0) de la trace, dans le cadre de la séquence de finalisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `flyToKm0Duration` contrôle la vitesse de la transition visuelle qui, après la fin de l'animation, peut ramener la caméra au point de départ de la trace. Cette étape est souvent utilisée avant une reprise automatique de l'animation.

-   **Libellé**: Durée de l'animation vers le départ
-   **Type**: Réel
-   **Valeur par défaut**: 1.0 sec
-   **Minimum**: 0.2 sec
-   **Maximum**: 5.0 sec
-   **Unité**: sec

## ⚖️ Justification : Pourquoi ajuster la durée du survol vers Km 0 ?

L'ajustement de cette durée est important pour la fluidité de la transition vers le point de départ, en particulier si une reprise automatique de l'animation est configurée.

### 1. 🎥 Fluidité de la Transition

-   Une durée appropriée assure une transition douce vers le point de départ, évitant une navigation abrupte.

### 2. 🔄 Préparation à la Reprise Automatique

-   Si l'animation est configurée pour redémarrer automatiquement, cette transition prépare l'utilisateur à la nouvelle boucle.

---

## ⚠️ Recommandations

-   **Valeur par défaut (1.0 sec)** : Une seconde offre une transition douce et suffisamment rapide.
-   **Adapter aux préférences** : Vous pouvez augmenter ou diminuer cette durée selon l'effet cinématique souhaité.
