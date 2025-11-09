# 📏 Paramètre : Distance du zoom au départ (x100m)

Ce document détaille le paramètre `zoomDepartDistance`, qui définit la distance sur laquelle l'animation de dézoom s'applique au début du parcours, exprimée en segments de 100 mètres.

---

## 🎯 Rôle du Paramètre

Le paramètre `zoomDepartDistance` contrôle la durée spatiale de l'animation de dézoom au point de départ. Plus la valeur est élevée, plus l'animation s'étendra sur une longue portion de la trace.

-   **Libellé**: Distance du zoom au départ (x100m)
-   **Type**: Entier
-   **Valeur par défaut**: 20 (soit 2000 mètres ou 2 km)
-   **Minimum**: 1 (soit 100 mètres)
-   **Maximum**: 100 (soit 10000 mètres ou 10 km)

## ⚖️ Justification : Pourquoi ajuster la distance du zoom au départ ?

L'ajustement de cette distance permet de contrôler la durée et l'étendue de l'animation de dézoom, influençant la fluidité et la pertinence du contexte visuel.

### 1. 🎥 Fluidité de l'Animation

-   Une distance plus longue (`> 20`) rend l'animation de dézoom plus lente et progressive, offrant une transition plus douce.
-   Une distance plus courte (`< 20`) rend l'animation plus rapide et plus abrupte.

### 2. 🧭 Contexte Visuel

-   Une distance plus longue permet de révéler une plus grande portion du début de la trace, offrant un contexte géographique plus étendu.
-   Une distance plus courte se concentre sur le contexte immédiat du point de départ.

---

## ⚠️ Recommandations

-   **Valeur par défaut (20)** : C'est un bon compromis pour une animation fluide et un contexte visuel suffisant.
-   **Adapter à la trace** :
    -   Pour des traces très courtes, une distance plus faible peut être préférable pour ne pas "survoler" une trop grande partie du parcours.
    -   Pour des traces très longues, une distance plus élevée peut donner une meilleure introduction au parcours.
