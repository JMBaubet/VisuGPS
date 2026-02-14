# ⏱️ Paramètre : Vitesse par défaut de l'animation

Ce document détaille le paramètre `default_value`, qui définit le multiplicateur de vitesse par défaut de l'animation de la trace en mode visualisation, au démarrage ou à la réinitialisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `default_value` spécifie le multiplicateur de vitesse qui sera appliqué à l'animation lorsque celle-ci démarre ou est réinitialisée. Il doit se situer entre `min_value` et `max_value`.

-   **Libellé**: Vitesse par défaut (x)
-   **Type**: Réel (Float)
-   **Valeur par défaut**: 1.0
-   **Minimum**: 0.2
-   **Maximum**: 20.0
-   **Pas (step)**: 0.2
-   **Décimales**: 1

## ⚖️ Justification : Pourquoi définir une vitesse par défaut ?

La définition d'une vitesse par défaut assure une expérience utilisateur cohérente et prévisible au démarrage de chaque animation.

### 1. 🚀 Expérience Utilisateur

-   Permet de commencer l'animation à une vitesse confortable et standard, évitant ainsi une vitesse trop lente ou trop rapide par défaut.
-   Facilite la prise en main de l'application en offrant un point de départ connu.

### 2. 🔄 Cohérence

-   Garantit que toutes les animations commencent avec le même réglage de vitesse, sauf si l'utilisateur le modifie manuellement.

---

## ⚠️ Recommandations

-   **Valeur par défaut (1.0)** : Un multiplicateur de 1.0 signifie que l'animation se déroule à la vitesse de base définie par le paramètre `vitesse` (ms/km), ce qui est un bon équilibre pour une première visualisation.
-   **Entre min et max** : Assurez-vous que cette valeur est toujours comprise entre `min_value` et `max_value` pour éviter des comportements inattendus.
