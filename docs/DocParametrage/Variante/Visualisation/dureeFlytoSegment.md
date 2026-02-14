# ⏱️ Paramètre : Durée du flyto vers le segment (sec)

Ce document détaille le paramètre `Durée du flyto vers le segment`, qui configure la vitesse de déplacement de la caméra lors de la sélection d'un segment de variante.

---

## 🎯 Rôle du Paramètre

Il définit la durée, en secondes, que met la caméra pour se déplacer depuis sa position actuelle vers le premier point du segment de variante sélectionné dans l'interface de visualisation.

-   **Type** : Réel (décimal)
-   **Valeur par défaut** : 2.0 sec
-   **Plage autorisée** : 0.0 sec - 10.0 sec
-   **Pas** : 0.1 sec

## ⚖️ Justification

Ce délai permet d'offrir une transition visuelle fluide, aidant l'utilisateur à comprendre géographiquement où se situe le début de la variante par rapport à sa position précédente.

## ⚠️ Recommandations

-   **1.5 - 3.0 sec** offre généralement une transition agréable sans être trop lente.
-   Une valeur de **0** entraîne un saut instantané, ce qui peut être désorientant.
-   Une valeur trop élevée (> 5 sec) peut donner une impression de lenteur inutile.
