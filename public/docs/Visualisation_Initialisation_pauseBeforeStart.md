# ⏸️ Paramètre : Pause avant le début de l'animation (Visualisation)

Ce document détaille le paramètre `pauseBeforeStart`, qui définit la durée en millisecondes d'une pause entre la vue globale de la trace et le début effectif de l'animation en mode visualisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `pauseBeforeStart` introduit un court délai après que la trace complète soit affichée à l'écran, mais avant que l'animation ne commence à se déplacer le long de la trace. Cette pause permet à l'utilisateur de s'orienter et de visualiser l'ensemble du parcours.

-   **Libellé**: Pause avant début (ms)
-   **Type**: Entier
-   **Valeur par défaut**: 1000 ms
-   **Minimum**: 500 ms
-   **Maximum**: 5000 ms
-   **Unité**: ms

## ⚖️ Justification : Pourquoi une pause avant le début de l'animation ?

Une pause avant le début de l'animation est cruciale pour l'expérience utilisateur, car elle offre un moment de préparation et de compréhension du parcours.

### 1. 👀 Orientation et Compréhension

-   Permet à l'utilisateur de prendre connaissance de la forme générale de la trace, de son point de départ et d'arrivée, et de son environnement avant que la caméra ne commence à se déplacer.
-   Évite que l'animation ne démarre trop rapidement, ce qui pourrait être déroutant.

### 2. 🎥 Effet Cinématique

-   Renforce l'effet d'introduction en marquant une transition claire entre la présentation de la trace et le début de son exploration.

---

## ⚠️ Recommandations

-   **Valeur par défaut (1000 ms)** : Une seconde est une durée suffisante pour que l'utilisateur puisse s'orienter sans que l'attente ne soit trop longue.
-   **Adapter aux préférences** : Vous pouvez ajuster cette durée si vous souhaitez une pause plus courte ou plus longue.
