# 🖱️ Paramètre : Incrément de cap (molette de la souris)

Ce document détaille le paramètre `incrementBearing`, qui définit l'incrément de rotation du cap de la caméra lorsque la molette de la souris est utilisée en mode édition, sous certaines conditions.

---

## 🎯 Rôle du Paramètre

Le paramètre `incrementBearing` contrôle la granularité des ajustements manuels du cap de la caméra. Cette fonctionnalité est active uniquement lorsque les deux conditions suivantes sont remplies :
1. Le curseur de la souris se trouve sur le widget des onglets de contrôle (Caméra, Pause/Zooms, Message).
2. L'onglet "Caméra" est sélectionné.

-   **Libellé**: Incrément Cap
-   **Type**: Réel (Float)
-   **Valeur par défaut**: 1.0
-   **Minimum**: 0.1
-   **Maximum**: 10.0
-   **Décimales**: 1

## ⚖️ Justification : Pourquoi ajuster l'incrément de cap ?

L'ajustement de cet incrément permet de moduler la sensibilité de la rotation de la caméra pour un contrôle plus fin ou plus rapide.

### 1. 🧭 Précision de l'Ajustement

-   **Valeur faible** (`< 1.0`) : Permet des rotations très lentes et précises, idéales pour un cadrage minutieux.
-   **Valeur élevée** (`> 1.0`) : Permet des rotations plus rapides, utiles pour changer rapidement l'orientation de la caméra.

### 2. 🚀 Rapidité de Réglage

-   Une valeur plus élevée accélère le réglage manuel du cap, ce qui peut être pratique pour des changements d'angle importants.

---

## ⚠️ Recommandations

-   **Valeur par défaut (1.0)** : Offre un bon compromis entre précision et vitesse pour la plupart des usages.
-   **Adapter aux besoins** :
    -   Pour des ajustements de cap très précis, réduisez la valeur.
    -   Pour des rotations plus rapides et réactives, utilisez conjointement le touche **Shift**.
