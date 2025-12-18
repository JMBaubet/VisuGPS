# 🔍 Paramètre : Zoom Minimum en pause (Visualisation)

Ce document détaille le paramètre `zoomMinimum`, qui définit le niveau de zoom minimum autorisé pour la vue Mapbox lorsque l'animation est en pause en mode visualisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `zoomMinimum` restreint la capacité de l'utilisateur à dézoomer excessivement la carte pendant une pause de l'animation. La valeur réelle du zoom est calculée en divisant la valeur de ce paramètre par 10. (ex: 100 signifie un zoom de 10.0).

-   **Libellé**: Zoom Minimum
-   **Type**: Entier
-   **Valeur par défaut**: 100 (correspond à un zoom réel de 10.0)
-   **Minimum**: 80 (correspond à un zoom réel de 8.0)
-   **Maximum**: 125 (correspond à un zoom réel de 12.5)
-   **Pas (step)**: 1

## ⚖️ Justification : Pourquoi définir un zoom minimum en pause ?

La définition d'un zoom minimum en pause permet de maintenir une vue suffisamment détaillée de la trace, même en cas de dézoom manuel, et d'éviter que l'utilisateur ne perde de vue la trace.

### 1. 👀 Maintien du Focale

-   Garantit que la trace reste visible et exploitable pour l'analyse ou le positionnement manuel de la caméra pendant la pause.
-   Empêche l'utilisateur de dézoomer à un point tel que la trace deviendrait difficilement identifiable.

### 2. 🖐️ Expérience Utilisateur

-   Évite une perte de contexte si l'utilisateur dézoome par inadvertance loin de la trace.

---

## ⚠️ Recommandations

-   **Valeur par défaut (100)** : Un zoom de 10.0 offre généralement une bonne vue d'ensemble de la trace sans perdre trop de détails du terrain environnant.
-   **Adapter au type de trace** :
    -   Pour des traces très courtes ou complexes, une valeur légèrement plus élevée peut être souhaitable pour maintenir une vue plus détaillée.
    -   Pour des traces très longues couvrant de grandes distances, un zoom minimum légèrement plus faible (tout en restant au-dessus du minimum de 80) pourrait être envisagé.
-   **Comprendre la conversion** : Rappelez-vous que la valeur affichée dans l'interface correspond à la valeur réelle du zoom divisée par 10.
