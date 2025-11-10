# ⏱️ Paramètre : Vitesse de base de l'animation (ms/km)

Ce document détaille le paramètre `vitesse`, qui définit la vitesse de base de l'animation en mode visualisation, exprimée en millisecondes pour parcourir 1 kilomètre.

---

## 🎯 Rôle du Paramètre

Le paramètre `vitesse` est la valeur fondamentale qui détermine la rapidité de l'animation. Tous les multiplicateurs de vitesse (`min_value`, `max_value`, `default_value`) sont appliqués à cette valeur de base.

-   **Libellé**: Vitesse de l'animation (ms/km)
-   **Type**: Entier
-   **Valeur par défaut**: 3730 ms/km
-   **Minimum**: 100 ms/km
-   **Maximum**: 60000 ms/km

## ⚖️ Justification : Pourquoi ajuster la vitesse de base de l'animation ?

L'ajustement de la vitesse de base permet de calibrer l'animation pour qu'elle corresponde à la durée souhaitée de la visualisation ou au rythme d'observation préféré.

### 1. ⏳ Durée de l'Animation

-   Une valeur plus faible (moins de ms/km) rend l'animation plus rapide, ce qui est utile pour des présentations courtes ou des aperçus rapides.
-   Une valeur plus élevée (plus de ms/km) ralentit l'animation, permettant une observation plus détaillée et une meilleure immersion.

### 2. 🚶‍♀️ Réalisme

-   Peut être ajustée pour simuler une vitesse de déplacement plus réaliste (par exemple, vitesse de marche, de course, de vélo, de voiture).

---

## ⚠️ Recommandations

-   **Valeur par défaut (3730 ms/km)** : Cette valeur correspond approximativement à une vitesse de 1 km en 3.73 secondes. C'est un bon point de départ pour une visualisation dynamique.
-   **Expérimentation** : Testez différentes valeurs pour trouver celle qui convient le mieux à la durée et au rythme souhaités pour votre animation.
-   **Impact sur les multiplicateurs** : Gardez à l'esprit que cette valeur est la base sur laquelle les multiplicateurs de vitesse sont appliqués.
