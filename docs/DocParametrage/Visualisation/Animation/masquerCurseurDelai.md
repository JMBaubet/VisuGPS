# 🖱️ Paramètre : Délai avant de masquer le curseur (Visualisation)

Ce document détaille le paramètre `masquerCurseurDelai`, qui définit le délai d'inactivité en millisecondes avant que le curseur de la souris ne soit automatiquement masqué pendant l'animation en mode visualisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `masquerCurseurDelai` permet de contrôler l'immersion visuelle en faisant disparaître le curseur de la souris après une période d'inactivité, évitant ainsi qu'il ne gêne la visualisation de l'animation.

-   **Libellé**: Délai avant de masquer le curseur (ms)
-   **Type**: Entier
-   **Valeur par défaut**: 1000 ms
-   **Minimum**: 500 ms
-   **Maximum**: 10000 ms
-   **Pas (step)**: 500 ms
-   **Unité**: ms

## ⚖️ Justification : Pourquoi ajuster le délai de masquage du curseur ?

L'ajustement de ce délai est important pour l'expérience utilisateur, car il équilibre la disponibilité du curseur pour les interactions et l'immersion visuelle.

### 1. 👀 Immersion Visuelle

-   Masquer le curseur après une courte période d'inactivité permet une visualisation plus immersive de l'animation, sans distraction.

### 2. 🖐️ Confort d'Utilisation

-   Un délai trop court pourrait rendre le curseur difficile à utiliser pour des interactions rapides.
-   Un délai trop long pourrait laisser le curseur visible inutilement, nuisant à l'immersion.

---

## ⚠️ Recommandations

-   **Valeur par défaut (1000 ms)** : Une seconde est un bon compromis qui permet au curseur de disparaître rapidement sans être intrusif.
-   **Adapter aux préférences** :
    -   Si vous préférez que le curseur reste visible plus longtemps, augmentez la valeur.
    -   Si vous souhaitez une immersion maximale et que le curseur disparaisse presque instantanément, réduisez la valeur (mais pas en dessous de 500 ms pour éviter des problèmes d'interaction).
