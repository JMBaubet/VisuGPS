# 🔄 Paramètre : Reprise automatique de l'animation (Visualisation)

Ce document détaille le paramètre `repriseAutomatique`, qui contrôle le redémarrage automatique de l'animation après la séquence de finalisation (retour à la vue globale de la trace) en mode visualisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `repriseAutomatique` est un interrupteur (booléen) qui, lorsqu'activé, fait boucler l'animation indéfiniment. Après avoir terminé le parcours et la séquence de finalisation, l'animation redémarre automatiquement depuis le début.

-   **Libellé**: Reprise automatique de l'animation
-   **Type**: Booléen
-   **Valeur par défaut**: `false`
-   **Surcharge**: `null` (indique que ce paramètre peut être surchargé par les paramètres spécifiques à une trace)

## ⚖️ Justification : Pourquoi une reprise automatique ?

La reprise automatique est utile pour des présentations en boucle, des affichages en continu ou pour permettre une observation répétée de la trace sans intervention manuelle.

### 1. 🔁 Usage en Continu

-   Idéal pour des démonstrations, des écrans d'affichage ou des événements où l'animation doit être diffusée en continu.

### 2. 👀 Analyse Répétée

-   Permet d'analyser la trace à plusieurs reprises sous différents angles ou avec des paramètres variés sans avoir à relancer manuellement l'animation.

### 3. 🖐️ Simplicité d'Utilisation

-   Réduit le besoin d'interaction de l'utilisateur une fois que l'animation est configurée.

---

## ⚠️ Recommandations

-   **Valeur par défaut (`false`)** : Par défaut, l'animation se termine après un cycle.
-   **Activer pour des démonstrations** : Si vous utilisez l'application pour des présentations ou des affichages publics, l'activation de ce paramètre est recommandée.
-   **Combiner avec d'autres paramètres de finalisation** : Si activé, assurez-vous que les paramètres de finalisation (`delayAfterAnimationEnd`, `pauseAvantReprise`) sont configurés pour une boucle fluide et agréable.
