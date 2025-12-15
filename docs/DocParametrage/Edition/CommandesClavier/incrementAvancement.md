# ⌨️ Paramètre : Incrément d'avancement (flèches gauche/droite)

Ce document détaille le paramètre `incrementAvancement`, qui définit le nombre de points de tracking par lesquels la caméra avance ou recule lorsque les flèches gauche/droite du clavier sont utilisées.

---

## 🎯 Rôle du Paramètre

Le paramètre `incrementAvancement` contrôle la granularité du déplacement de la caméra le long de la trace lors de l'édition, en utilisant les touches fléchées gauche et droite.

-   **Libellé**: Incrément Avancement
-   **Type**: Entier
-   **Valeur par défaut**: 1
-   **Minimum**: 1
-   **Maximum**: 10

## ⚖️ Justification : Pourquoi ajuster l'incrément d'avancement ?

L'ajustement de cet incrément permet de contrôler la précision et la rapidité des déplacements manuels de la caméra.

### 1. 🚶 Précision du Déplacement

-   **Valeur faible** (`1`) : Permet un déplacement très précis, point par point, idéal pour des ajustements fins ou pour naviguer lentement sur la trace.
-   **Valeur élevée** (`> 1`) : Permet un déplacement plus rapide, sautant plusieurs points à la fois, utile pour parcourir rapidement de longues sections.

### 2. 🚀 Rapidité de Navigation

-   Une valeur plus élevée accélère la navigation manuelle sur la trace.

---

## ⚠️ Recommandations

-   **Valeur par défaut (1)** : C'est un bon compromis pour un contrôle précis par défaut.
-   **Adapter aux besoins** :
    -   Pour des ajustements très fins, maintenez la valeur à 1.
    -   Pour une navigation plus rapide, augmentez cette valeur.
