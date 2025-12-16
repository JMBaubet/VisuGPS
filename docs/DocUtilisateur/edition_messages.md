# Gestion des Messages

Les messages permettent d'ajouter des informations textuelles contextuelles qui apparaissent à l'écran pendant l'animation.

[< Retour aux généralités Édition](./edition_intro.md)

## Types de Messages

Vous pouvez utiliser les messages pour :
*   Nommer un col ou un sommet franchi.
*   Indiquer un ravitaillement.
*   Donner une anecdote historique.

## Ajouter un Message

1.  Positionnez le curseur timeline au kilomètre souhaité.
2.  Dans l'onglet "Messages" (ou "Évènements" selon la version), cliquez sur **"Ajouter un message"**.
3.  Saisissez le **Texte** à afficher.

## Configuration du Message

Pour chaque message, vous pouvez régler :

*   **Contenu** : Le texte à afficher (supporte parfois le HTML basique ou Markdown selon la version).
*   **Ancrage (Anchor)** : Le point kilométrique exact de l'apparition.
*   **Visibilité** : Activer/Désactiver l'affichage sans supprimer le message.
*   **Apparition progressive** :
    *   Si le message est lié à une **Pause**, il peut apparaître progressivement (fade-in) pendant la durée de la pause.
    *   Si le message est au début de la trace (km 0), il peut apparaître pendant l'animation d'introduction ("FlyTo Départ").

## Paramètres Techniques

Liste des propriétés sauvegardées pour un message :
*   `id` : Identifiant unique.
*   `content` : Le texte du message.
*   `distance` : Position en km sur la trace.
*   `visible` : État d'affichage (true/false).

---
[< Retour aux généralités Édition](./edition_intro.md) | [Suivant : Visualisation >](./visualisation.md)
