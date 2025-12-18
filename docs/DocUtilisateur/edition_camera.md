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

## Modes de Synchronisation

En haut de l'onglet, vous pouvez choisir comment la caméra se comporte pendant que vous naviguez sur la trace :

*   **Fixe** : La caméra reste à la position (Zoom, Pitch, Cap) que vous avez définie manuellement. Elle ne suit pas l'animation. C'est idéal pour préparer un nouveau point clé sans être "emporté" par le mouvement existant.
*   **Animée** : C'est le mode par défaut. La caméra suit fidèlement la trajectoire et les angles que vous avez programmés. Cela permet de vérifier immédiatement le rendu de vos transitions.

---

## Principe des Points Clés (Keyframes)

L'animation repose sur l'enregistrement de positions à des endroits précis de la trace :

1.  **Positionnement** : Déplacez-vous sur la trace (clavier ou graphes) jusqu'au kilomètre souhaité.
2.  **Cadrage** : Ajustez le zoom, le cap et le pitch pour obtenir la vue idéale.
3.  **Enregistrement** : Cliquez sur le bouton **Ajouter Point** ![plus](https://api.iconify.design/mdi/plus.svg?width=20) en bas de l'onglet.
4.  **Transition** : L'application calcule automatiquement le mouvement fluide entre ce point et le point clé précédent.

Pour retirer une caméra, positionnez-vous exactement sur son emplacement (elle apparaît en vert sur le graphe) et cliquez sur **Supprimer Point** ![delete](https://api.iconify.design/mdi/delete.svg?width=20).

---

## Zoom Départ et Arrivée

En bas de l'onglet, deux sous-onglets permettent de configurer des effets de transition automatiques :

*   **Zoom Départ** : Si activé, la caméra effectue une transition fluide entre le niveau de zoom choisi et le premier point clé de la trace, sur la distance (en mètres) définie par le curseur.
*   **Zoom Arrivée** : Similaire au départ, mais s'applique à la fin du parcours, entre le dernier point clé et le niveau de zoom final souhaité.

---

## Affichage des Courbes

Le panneau repliable **"Affichage des courbes"** permet de superposer des informations sur les graphes pour vous aider dans vos réglages :

*   **Colonne "Calculée"** :
    *   **Cap** : Affiche le cap "brut" de la trace GPX (sa direction réelle). Utile pour aligner votre caméra sur le chemin.
*   **Colonne "Éditée"** (en couleur) :
    *   **Δ Cap** : Affiche les variations de rotation (changements de direction) que vous avez programmées.
    *   **Cap** : Affiche l'orientation absolue de la caméra.
    *   **Zoom / Pitch** : Visualise les courbes de progression de ces paramètres.

---

## Paramètres Techniques

Chaque point clé enregistre les valeurs suivantes :
*   `zoom` : Niveau de rapprochement (ex: 14.5).
*   `pitch` : Inclinaison (0° vertical à 85° rasante).
*   `bearing` : Orientation (en degrés, 0 à 360).
*   `target` : Coordonnées du point visé sur la trace.

---
[< Retour aux généralités Édition](./edition_intro.md) | [Suivant : FlyTo & Pauses >](./edition_flyto_pause.md)
