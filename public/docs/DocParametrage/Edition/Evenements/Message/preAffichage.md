# ⏱️ Paramètre : Pré-affichage du message (incréments)

Ce document détaille le paramètre `preAffichage`, qui définit le nombre d'incréments de temps (ou de points de tracking) avant le point de contrôle où le message commence à s'afficher.

---

## 🎯 Rôle du Paramètre

Le paramètre `preAffichage` permet de contrôler le moment où un message commence à être visible avant que la caméra n'atteigne le point de la trace où l'événement de message est défini.

-   **Libellé**: Pré-affichage (incréments)
-   **Type**: Entier
-   **Valeur par défaut**: 10
-   **Minimum**: 0 (affichage au point de contrôle)
-   **Maximum**: 50
-   **Pas (Step)**: 1

## ⚖️ Justification : Pourquoi un pré-affichage du message ?

Le pré-affichage améliore la fluidité de la visualisation et permet une meilleure anticipation des informations importantes.

### 1. 🎥 Fluidité de la Visualisation

-   Permet au message d'apparaître progressivement avant d'atteindre le point clé, rendant l'expérience moins abrupte.

### 2. 🧠 Anticipation de l'Information

-   Donne à l'utilisateur un temps pour lire et assimiler le message avant que la caméra n'atteigne précisément le point d'intérêt.

---

## ⚠️ Recommandations

-   **Valeur par défaut (10)** : C'est un bon compromis qui offre un temps de pré-affichage suffisant sans que le message n'apparaisse trop tôt.
-   **Adapter au rythme** :
    -   Pour des messages courts ou si l'animation est rapide, une valeur plus faible peut être suffisante.
    -   Pour des messages plus longs ou si l'animation est lente, une valeur plus élevée peut être préférable.
-   **Valeur de 0** : Le message apparaît exactement au moment où la caméra atteint le point de contrôle.
