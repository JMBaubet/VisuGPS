# 📏 Paramètre : Distance du zoom arrivée

Ce document détaille le paramètre `Distance du zoom arrivée`, qui détermine quand commence l'effet de zoom final.

---

## 🎯 Rôle du Paramètre

Il définit la distance avant la fin du parcours à partir de laquelle la caméra commence sa transition vers le zoom d'arrivée. Cette distance est exprimée en segments de 100m.

-   **Type**: Entier
-   **Valeur par défaut**: 20 (soit 2 km)
-   **Plage autorisée**: 1 - 100
-   **Unité** : Segments de 100m

## 🎬 Impact Visuel

-   Une distance suffisante permet une transition fluide et évite un effet de "plongeon" trop brutal sur l'arrivée.
-   Le zoom progressif accompagne les derniers kilomètres de l'effort.

---

## ⚠️ Recommandations

-   Ajustez cette valeur en fonction de la longueur totale de votre circuit. Sur un circuit de 5km, 2km de zoom peut sembler long (40% du trajet).
