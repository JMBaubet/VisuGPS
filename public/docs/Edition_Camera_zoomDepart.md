# 🔎 Paramètre : Activer le zoom au départ

Ce document détaille le paramètre `zoomDepart`, qui contrôle l'activation d'un dézoom automatique au début du parcours lors de l'édition d'une trace.

---

## 🎯 Rôle du Paramètre

Le paramètre `zoomDepart` est un interrupteur (booléen) qui, lorsqu'activé, déclenche une animation de dézoom au point de départ (Km 0) de la trace. Cette animation permet d'avoir une vue plus large du début du parcours.

-   **Libellé**: Activer le zoom au départ
-   **Type**: Booléen
-   **Valeur par défaut**: `true`

## ⚖️ Justification : Pourquoi un zoom au départ ?

Le zoom au départ améliore l'expérience utilisateur en fournissant un contexte visuel plus large dès le début de l'édition.

### 1. 🧭 Contexte Visuel

-   Permet de mieux situer le début de la trace dans son environnement immédiat, ce qui est utile pour comprendre le contexte du parcours.
-   Évite de commencer l'édition avec une vue trop rapprochée qui pourrait masquer des informations importantes.

### 2. 🎥 Fluidité de l'Interface

-   L'animation de dézoom offre une transition douce vers la vue d'édition, rendant l'expérience plus agréable.

---

## ⚠️ Recommandations

-   **Activé par défaut** : Il est recommandé de laisser ce paramètre activé pour bénéficier d'une meilleure vue d'ensemble au début de l'édition.
-   **Désactiver si non souhaité** : Si vous préférez commencer l'édition avec un zoom très rapproché ou si l'animation vous dérange, vous pouvez désactiver ce paramètre.
