# ➕ Paramètre : Afficher la croix centrale en pause (Visualisation)

Ce document détaille le paramètre `afficherCroixCentrale`, qui contrôle l'affichage d'une croix au centre de l'écran lorsque l'animation est en pause en mode visualisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `afficherCroixCentrale` est un interrupteur (booléen) qui, lorsqu'activé, affiche une croix discrète au centre de la vue Mapbox. Cette croix sert de repère visuel pour le centre de l'écran, ce qui peut être utile pour le positionnement manuel de la caméra ou pour des captures d'écran précises.

-   **Libellé**: Afficher la croix centrale en pause
-   **Type**: Booléen
-   **Valeur par défaut**: `true`

## ⚖️ Justification : Pourquoi afficher une croix centrale en pause ?

L'affichage d'une croix centrale en pause offre un repère visuel utile pour diverses opérations, notamment le positionnement précis de la caméra.

### 1. 🎯 Repère de Centrage

-   Facilite le positionnement manuel de la caméra sur un point d'intérêt spécifique lorsque l'animation est en pause.
-   Utile pour aligner des éléments ou pour des captures d'écran où un centrage précis est requis.

### 2. 🖐️ Aide à la Composition

-   Peut servir d'aide à la composition visuelle pour les utilisateurs qui souhaitent créer des vues spécifiques.

---

## ⚠️ Recommandations

-   **Valeur par défaut (`true`)** : L'affichage par défaut de la croix offre un outil utile sans être trop intrusif.
-   **Désactiver pour une vue épurée** : Si vous préférez une vue complètement dégagée sans aucun repère visuel, vous pouvez désactiver ce paramètre.
