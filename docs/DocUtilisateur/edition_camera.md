# Configuration de la Caméra

Ce document détaille comment contrôler les mouvements de la caméra pour créer des effets dynamiques.

[< Retour aux généralités Édition](./edition_intro.md)

## Principe des Clés d'Animation (Keyframes)

L'animation de la caméra repose sur des "clés". Une clé est un enregistrement de la position de la caméra à un kilomètre précis de la trace.

*   Si vous placez la caméra en vue de dessus au km 0 et en vue rasante au km 5, l'application fera transiter la caméra progressivement de l'un à l'autre entre le km 0 et le km 5.

## Outils de Caméra

Dans l'onglet "Caméra" du panneau latéral, vous disposez de plusieurs curseurs :

1.  **Zoom** : Rapproche ou éloigne la caméra du sol.
2.  **Pitch (Inclinaison)** : Incline la caméra vers l'horizon (0° = vue de dessus, 60°+ = vue rasante).
3.  **Bearing (Cap)** : Oriente la caméra (Nord, Sud, etc.) ou la fait tourner autour du sujet.

## Comment créer une clé ?

1.  Déplacez le curseur de la timeline (en bas) à l'endroit désiré sur la trace.
2.  Ajustez la vue 3D avec la souris ou les curseurs du panneau latéral pour obtenir le cadrage parfait.
3.  Cliquez sur le bouton **"Enregistrer la vue"** (souvent une icône de caméra ou de disquette).
4.  Un repère visuel apparaît sur la timeline pour indiquer qu'une clé existe à cet endroit.

Pour supprimer une clé, placez-vous dessus et cliquez sur l'icône de suppression (poubelle).

## Paramètres Techniques

Les valeurs suivantes sont stockées pour chaque clé dans le fichier de configuration :
*   `camera.zoom` : Niveau de zoom (ex: 12 à 16).
*   `camera.pitch` : Angle en degrés (0 à 85).
*   `camera.bearing` : Angle en degrés (0 à 360).
*   `camera.target` : Point central visé par la caméra.

---
[< Retour aux généralités Édition](./edition_intro.md) | [Suivant : FlyTo & Pauses >](./edition_flyto_pause.md)
