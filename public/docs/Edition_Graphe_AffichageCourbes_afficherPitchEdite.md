# 📊 Paramètre : Afficher Pitch (Édité)

Ce document détaille le paramètre `afficherPitchEdite`, qui contrôle l'affichage de la courbe de l'angle d'inclinaison (pitch) édité sur le graphe d'édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `afficherPitchEdite` est un interrupteur (booléen) qui, lorsqu'activé, affiche une courbe représentant l'angle d'inclinaison (pitch) de la caméra après l'application des modifications manuelles ou des lissages.

-   **Libellé**: Afficher Pitch (Édité)
-   **Type**: Booléen
-   **Valeur par défaut**: `true`

## ⚖️ Justification : Pourquoi afficher le pitch édité ?

L'affichage de cette courbe est essentiel pour visualiser l'impact des modifications apportées à l'angle de la caméra et pour s'assurer de la fluidité et de la cohérence du mouvement global.

### 1. 📈 Visualisation des Modifications

-   Permet de voir comment les ajustements manuels ou les lissages ont affecté l'angle d'inclinaison de la caméra.
-   Utile pour s'assurer que le pitch varie de manière fluide et naturelle le long de la trace, en fonction du relief ou des points d'intérêt.

### 2. 🔍 Contrôle de l'Expérience

-   Permet de vérifier que le pitch correspond à l'effet visuel souhaité (par exemple, un pitch plus faible pour accentuer le relief en montée).

---

## ⚠️ Recommandations

-   **Activé par défaut** : Ce paramètre est activé par défaut car il fournit une vue essentielle du comportement du pitch après édition, ce qui est souvent le résultat souhaité.
-   **Désactiver si non pertinent** : Si vous n'êtes pas intéressé par le comportement du pitch ou si vous préférez une vue plus épurée du graphe, vous pouvez désactiver ce paramètre.
