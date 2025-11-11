# ⏱️ Paramètre : Post-affichage du message (incréments)

Ce document détaille le paramètre `postAffichage`, qui définit le nombre d'incréments de temps (ou de points de tracking) après le point de contrôle où le message cesse de s'afficher.

---

## 🎯 Rôle du Paramètre

Le paramètre `postAffichage` permet de contrôler le moment où un message disparaît après que la caméra ait dépassé le point de la trace où l'événement de message est défini.

-   **Libellé**: Post-affichage (incréments)
-   **Type**: Entier
-   **Valeur par défaut**: 20
-   **Minimum**: 0 (disparition au point de contrôle)
-   **Maximum**: 100
-   **Pas (Step)**: 1

## ⚖️ Justification : Pourquoi un post-affichage du message ?

Le post-affichage améliore la fluidité de la visualisation et permet de laisser le message visible pendant une durée pertinente après le point d'intérêt.

### 1. 🎥 Fluidité de la Visualisation

-   Permet au message de disparaître progressivement après avoir dépassé le point clé, rendant l'expérience moins abrupte.

### 2. 🧠 Assimilation de l'Information

-   Donne à l'utilisateur un temps supplémentaire pour relire et assimiler le message après que la caméra ait dépassé le point d'intérêt.

---

## ⚠️ Recommandations

-   **Valeur par défaut (20)** : C'est un bon compromis qui offre un temps de post-affichage suffisant sans que le message ne reste visible trop longtemps.
-   **Adapter au rythme** :
    -   Pour des messages courts ou si l'animation est rapide, une valeur plus faible peut être suffisante.
    -   Pour des messages plus longs ou si l'animation est lente, une valeur plus élevée peut être préférable.
-   **Valeur de 0** : Le message disparaît exactement au moment où la caméra atteint le point de contrôle.
