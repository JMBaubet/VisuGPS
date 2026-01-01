# ⏱️ Paramètre : Durée du survol

Ce document détaille le paramètre `duree`, qui définit la durée par défaut de l'animation de survol en secondes, utilisée pour les événements de type "Survol" dans l'édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `duree` contrôle la vitesse de l'animation de la caméra lorsqu'elle se déplace vers un point d'intérêt. Une durée plus courte rend l'animation plus rapide, et vice-versa.

-   **Libellé**: Durée du survol (sec)
-   **Type**: Réel
-   **Valeur par défaut**: 2.0 sec
-   **Minimum**: 0.2 sec
-   **Maximum**: 10.0 sec
-   **Pas (Step)**: 0.1 sec
-   **Unité**: sec (secondes)

## ⚖️ Justification : Pourquoi ajuster la durée du survol ?

L'ajustement de la durée du survol permet de contrôler le rythme et la fluidité des transitions de la caméra entre les points d'intérêt.

### 1. 🎥 Fluidité de l'Animation

-   Une durée plus longue (`> 2.0 sec`) rend l'animation de survol plus douce et progressive, ce qui peut être agréable pour des transitions entre des vues éloignées.
-   Une durée plus courte (`< 2.0 sec`) rend l'animation plus rapide et dynamique, utile pour des transitions rapides entre des points proches.

### 2. ⏱️ Rythme de la Visualisation

-   Le choix de la durée peut influencer le rythme général de la visualisation, en particulier si de nombreux événements de survol sont utilisés.

---

## ⚠️ Recommandations

-   **Valeur par défaut (2.0 sec)** : C'est un bon compromis pour une animation de survol équilibrée.
-   **Adapter aux besoins** :
    -   Pour des transitions rapides et dynamiques, réduisez la durée.
    -   Pour des transitions plus lentes et contemplatives, augmentez la durée.
