# Configuration de la Caméra

Ce document détaille comment contrôler les mouvements de la caméra pour créer des effets dynamiques et des transitions fluides.

[< Retour aux généralités Édition](./edition_intro.md)

## Navigation et Commandes (Onglet Caméra)

Lorsque l'onglet **Caméra** est actif, vous disposez de contrôles précis pour ajuster votre vue :

### À la Souris
*   **Clic Droit + Glisser Horizontalement** : Pivoter la vue (Cap / Bearing).
*   **Molette** : Zoomer / Dézoomer d'un pas fin (0.1).
*   **Maj (Shift) + Molette** : Zoomer / Dézoomer rapidement (pas de 1.0).
*   **Survol du Panneau** : Si votre souris est au-dessus du panneau de configuration, la **Molette** seule permet de faire pivoter la vue (Cap) avec précision.

> [!NOTE]
> Le **clic gauche** est volontairement désactivé dans cet onglet pour éviter de perdre le centrage de la caméra sur la trace pendant vos réglages.

### Au Clavier
*   **Flèches Gauche / Droite** : Avancer ou reculer sur la trace (1 point).
*   **Flèches Haut / Bas** : Modifier l'inclinaison (Pitch) de 1°.
*   **Maj (Shift) + Flèches** : Mouvements rapides (10 points pour la distance, 5° pour le Pitch).

---

## Principe des Points Clés (Keyframes)

L'animation repose sur l'enregistrement de positions à des endroits précis de la trace :

1.  **Positionnement** : Déplacez-vous sur la trace (clavier ou graphes) jusqu'au kilomètre souhaité.
2.  **Cadrage** : Ajustez le zoom, le cap et le pitch pour obtenir la vue idéale.
3.  **Enregistrement** : Cliquez sur le bouton **Enregistrer la vue** ![camera](https://api.iconify.design/mdi/camera-plus-outline.svg?width=20) dans l'onglet Caméra.
4.  **Transition** : L'application calcule automatiquement le mouvement fluide entre ce point et le point clé suivant.

### Synchronisation Animée
Par défaut, l'éditeur est en mode **Synchronisation Animée**. Cela signifie que lorsque vous vous déplacez sur la trace, la caméra suit fidèlement la trajectoire que vous avez programmée. Cela permet de vérifier immédiatement le rendu de vos transitions.

---

## Affichage des Courbes

Dans l'onglet Caméra, un panneau repliable **"Affichage des courbes"** vous permet de superposer des informations sur les graphes :

*   **Courbe Éditée** : Affiche les variations de Zoom, Pitch ou Cap que vous avez programmées via vos points clés.
*   **Courbe Calculée** (pour le Cap) : Indique le cap "brut" de la trace, utile pour aligner votre caméra sur la direction réelle du chemin.

---

## Paramètres Techniques

Chaque point clé enregistre les valeurs suivantes :
*   `zoom` : Niveau de rapprochement (ex: 14.5).
*   `pitch` : Inclinaison (0° vertical à 85° rasante).
*   `bearing` : Orientation (en degrés, 0 à 360).
*   `target` : Coordonnées du point visé sur la trace.

---
[< Retour aux généralités Édition](./edition_intro.md) | [Suivant : FlyTo & Pauses >](./edition_flyto_pause.md)
