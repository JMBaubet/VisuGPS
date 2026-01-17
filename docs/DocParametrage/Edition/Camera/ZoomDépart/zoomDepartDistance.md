# 📏 Paramètre : Distance du zoom départ

Ce document détaille le paramètre `Distance du zoom départ`, qui contrôle la durée de la transition de zoom initiale.

---

## 🎯 Rôle du Paramètre

Il définit la distance sur laquelle la caméra va passer du zoom initial (*Valeur du zoom départ*) au zoom de croisière. Cette distance est exprimée en nombre de segments de 100 mètres.

-   **Type**: Entier
-   **Valeur par défaut**: 20 (soit 2 km)
-   **Plage autorisée**: 1 - 100
-   **Unité** : Segments de 100m (Ex : 20 = 2000m)

## 🎬 Impact Visuel

-   **Valeur faible (ex: 5)** : Transition rapide, effet de "zoom arrière" brutal.
-   **Valeur élevée (ex: 50)** : Transition très douce et lente, on garde une vue détaillée plus longtemps.

---

## ⚠️ Recommandations

-   La valeur par défaut de **20 (2 km)** offre un bon équilibre pour la plupart des randonnées ou sorties vélo.
-   Pour des circuits très courts, réduisez cette valeur pour que la transition ne prenne pas toute la durée de la vidéo.
