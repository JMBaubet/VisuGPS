# 📊 Paramètre : Afficher Delta Bearing (Édité)

Ce document détaille le paramètre `afficherBearingDeltaEdite`, qui contrôle l'affichage de la courbe du delta de cap édité sur le graphe d'édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `afficherBearingDeltaEdite` est un interrupteur (booléen) qui, lorsqu'activé, affiche une courbe représentant les changements de cap (bearing) après l'application des modifications manuelles ou des lissages.

-   **Libellé**: Afficher Delta Bearing (Édité)
-   **Type**: Booléen
-   **Valeur par défaut**: `false`

## ⚖️ Justification : Pourquoi afficher le delta de cap édité ?

L'affichage de cette courbe est utile pour visualiser l'impact des modifications apportées au cap de la caméra et pour s'assurer de la fluidité du mouvement.

### 1. 📈 Visualisation des Modifications

-   Permet de voir comment les ajustements manuels ou les lissages ont affecté les changements de direction de la caméra.
-   Utile pour s'assurer que les virages sont pris de manière fluide et naturelle.

### 2. 🔍 Comparaison

-   Peut être comparé à la courbe du "Delta Bearing (Calculé)" pour évaluer l'efficacité des modifications.

---

## ⚠️ Recommandations

-   **Désactivé par défaut** : Ce paramètre est désactivé par défaut.
-   **Activer pour l'édition** : Activez ce paramètre lorsque vous éditez activement le cap de la caméra pour visualiser l'impact de vos modifications.
