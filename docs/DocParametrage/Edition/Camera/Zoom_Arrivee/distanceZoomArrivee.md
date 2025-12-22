# 📏 Paramètre : Distance du zoom à l'arrivée (x100m)

Ce document détaille le paramètre `distanceZoomArrivee`, qui définit la distance sur laquelle l'animation de zoom s'applique à l'approche de l'arrivée du parcours, exprimée en segments de 100 mètres.

---

## 🎯 Rôle du Paramètre

Le paramètre `distanceZoomArrivee` contrôle la durée spatiale de l'animation de zoom à l'approche du point d'arrivée. Plus la valeur est élevée, plus l'animation s'étendra sur une longue portion de la fin de la trace.

-   **Libellé**: Distance du zoom à l'arrivée (x100m)
-   **Type**: Entier
-   **Valeur par défaut**: 20 (soit 2000 mètres ou 2 km)
-   **Minimum**: 1 (soit 100 mètres)
-   **Maximum**: 100 (soit 10000 mètres ou 10 km)

## ⚖️ Justification : Pourquoi ajuster la distance du zoom à l'arrivée ?

L'ajustement de cette distance permet de contrôler la durée et l'étendue de l'animation de zoom, influençant la fluidité et la pertinence du contexte visuel.

### 1. 🎥 Fluidité de l'Animation

-   Une distance plus longue (`> 20`) rend l'animation de zoom plus lente et progressive, offrant une transition plus douce.
-   Une distance plus courte (`< 20`) rend l'animation plus rapide et plus abrupte.

### 2. 🏁 Contexte Visuel de l'Arrivée

-   Une distance plus longue permet de révéler une plus grande portion de la fin de la trace, offrant un contexte géographique plus étendu avant d'atteindre le point d'arrivée.
-   Une distance plus courte se concentre sur le contexte immédiat du point d'arrivée.

---

## ⚠️ Recommandations

-   **Valeur par défaut (20)** : C'est un bon compromis pour une animation fluide et un contexte visuel suffisant.
-   **Adapter à la trace** :
    -   Pour des traces très courtes, une distance plus faible peut être préférable pour ne pas "survoler" une trop grande partie du parcours.
    -   Pour des traces très longues, une distance plus élevée peut donner une meilleure introduction à la fin du parcours.
