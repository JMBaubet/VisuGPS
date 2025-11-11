# 📊 Paramètre : Afficher Somme Delta Bearing (Édité)

Ce document détaille le paramètre `afficherBearingTotalDeltaEdite`, qui contrôle l'affichage de la courbe de la somme cumulée des deltas de cap édités sur le graphe d'édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `afficherBearingTotalDeltaEdite` est un interrupteur (booléen) qui, lorsqu'activé, affiche une courbe représentant la somme cumulée des changements de cap (bearing) après l'application des modifications manuelles ou des lissages. Cette courbe donne une idée de la rotation totale de la caméra le long de la trace après édition.

-   **Libellé**: Afficher Somme Delta Bearing (Édité)
-   **Type**: Booléen
-   **Valeur par défaut**: `true`

## ⚖️ Justification : Pourquoi afficher la somme du delta de cap édité ?

L'affichage de cette courbe est crucial pour visualiser l'impact des modifications apportées au cap de la caméra et pour s'assurer de la fluidité et de la cohérence du mouvement global.

### 1. 📈 Visualisation de la Rotation Globale

-   Permet de voir comment les ajustements manuels ou les lissages ont affecté la rotation totale de la caméra.
-   Utile pour s'assurer que la caméra suit la trace de manière cohérente et fluide sur l'ensemble du parcours.

### 2. 🔍 Comparaison

-   Peut être comparé à la courbe de la "Somme Delta Bearing (Calculé)" pour évaluer l'efficacité des modifications.

---

## ⚠️ Recommandations

-   **Activé par défaut** : Ce paramètre est activé par défaut car il fournit une vue essentielle de la rotation globale de la caméra après édition, ce qui est souvent le résultat souhaité.
-   **Désactiver si non pertinent** : Si vous n'êtes pas intéressé par la rotation cumulée ou si vous préférez une vue plus épurée du graphe, vous pouvez désactiver ce paramètre.
