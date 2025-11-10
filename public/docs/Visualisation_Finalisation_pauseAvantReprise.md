# ⏸️ Paramètre : Pause avant reprise automatique (Visualisation)

Ce document détaille le paramètre `pauseAvantReprise`, qui définit la durée en secondes d'une pause sur la vue globale de la trace avant de relancer l'animation, lorsque la reprise automatique est activée.

---

## 🎯 Rôle du Paramètre

Le paramètre `pauseAvantReprise` introduit un court délai après que la caméra soit revenue à la vue globale de la trace et avant que l'animation ne redémarre. Cette pause permet à l'utilisateur de revoir l'ensemble du parcours avant qu'un nouveau cycle ne commence.

-   **Libellé**: Pause avant reprise auto. (sec)
-   **Type**: Réel (Float)
-   **Valeur par défaut**: 1.0 sec
-   **Minimum**: 0.5 sec
-   **Maximum**: 10.0 sec
-   **Pas (step)**: 0.5 sec
-   **Unité**: sec

## ⚖️ Justification : Pourquoi une pause avant la reprise automatique ?

Une pause avant la reprise automatique est importante pour la fluidité de la boucle d'animation et pour offrir un moment de récapitulation visuelle.

### 1. 👀 Récapitulation Visuelle

-   Permet à l'utilisateur de revoir l'intégralité de la trace après un cycle complet, avant que l'animation ne redémarre.

### 2. 🔄 Fluidité de la Boucle

-   Assure une transition douce entre la fin d'un cycle et le début du suivant, évitant une reprise trop abrupte.

---

## ⚠️ Recommandations

-   **Valeur par défaut (1.0 sec)** : Une seconde offre une pause suffisante pour une brève récapitulation sans ralentir excessivement la boucle.
-   **Adapter aux préférences** : Vous pouvez ajuster cette durée si vous souhaitez une pause plus longue pour une meilleure observation ou une pause plus courte pour une boucle plus rapide.
