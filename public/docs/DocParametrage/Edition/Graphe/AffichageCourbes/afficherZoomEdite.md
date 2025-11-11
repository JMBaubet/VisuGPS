# 📊 Paramètre : Afficher Zoom (Édité)

Ce document détaille le paramètre `afficherZoomEdite`, qui contrôle l'affichage de la courbe du niveau de zoom édité sur le graphe d'édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `afficherZoomEdite` est un interrupteur (booléen) qui, lorsqu'activé, affiche une courbe représentant le niveau de zoom de la caméra après l'application des modifications manuelles ou des lissages.

-   **Libellé**: Afficher Zoom (Édité)
-   **Type**: Booléen
-   **Valeur par défaut**: `true`

## ⚖️ Justification : Pourquoi afficher le zoom édité ?

L'affichage de cette courbe est essentiel pour visualiser l'impact des modifications apportées au zoom de la caméra et pour s'assurer de la fluidité et de la cohérence du mouvement global.

### 1. 📈 Visualisation des Modifications

-   Permet de voir comment les ajustements manuels ou les lissages ont affecté le niveau de zoom de la caméra.
-   Utile pour s'assurer que le zoom varie de manière fluide et naturelle le long de la trace.

### 2. 🔍 Contrôle de l'Expérience

-   Permet de vérifier que le zoom correspond à l'effet visuel souhaité (par exemple, un zoom plus important dans les virages serrés ou les points d'intérêt).

---

## ⚠️ Recommandations

-   **Activé par défaut** : Ce paramètre est activé par défaut car il fournit une vue essentielle du comportement du zoom après édition, ce qui est souvent le résultat souhaité.
-   **Désactiver si non pertinent** : Si vous n'êtes pas intéressé par le comportement du zoom ou si vous préférez une vue plus épurée du graphe, vous pouvez désactiver ce paramètre.
