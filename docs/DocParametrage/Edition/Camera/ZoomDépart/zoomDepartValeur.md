# 🔭 Paramètre : Valeur du zoom départ

Ce document détaille le paramètre `Valeur du zoom départ`, qui définit l'intensité du zoom initial.

---

## 🎯 Rôle du Paramètre

Il détermine le niveau de zoom exact appliqué à la caméra lorsque la trace est à son point de départ (km 0), avant que le dézoom progressif ne commence.

-   **Type**: Entier
-   **Valeur par défaut**: 18
-   **Plage autorisée**: 10 - 22

## 📏 Échelle de Zoom

Les valeurs correspondent aux niveaux de zoom standards des cartes web :
-   **10-12** : Vue régionale / Ville.
-   **13-15** : Vue quartier / Village.
-   **16-18** : Vue rue / Détails topographiques précis (recommandé pour le départ).
-   **19-22** : Vue très rapprochée (bâtiments, arbres).

---

## ⚠️ Recommandations

-   Une valeur de **18** est idéale pour montrer le point de départ précis (parking, place de village).
-   Évitez des valeurs trop faibles (< 14) si vous voulez un effet de "décollage" marqué.
