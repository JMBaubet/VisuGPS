# Suppression de Circuits

Il est important de pouvoir gérer votre bibliothèque en supprimant les circuits obsolètes ou importés par erreur.

[< Retour au guide d'exploitation](./exploitation.md) 

## Supprimer un Circuit

L'action de suppression se fait directement depuis l'écran d'accueil (Bibliothèque) :

1.  Repérez la carte du circuit que vous souhaitez supprimer.
2.  Cliquez sur l'icône **Corbeille** <img src="https://api.iconify.design/mdi/delete.svg?color=red&width=20" style="vertical-align: middle; margin-bottom: 3px;"> (généralement rouge) située dans les actions de la carte.
3.  Une boîte de dialogue de confirmation apparaît :
    *   **Confirmer** : Le circuit sera définitivement retiré de l'application.
    *   **Annuler** : L'action est abandonnée.

## Ce qui est supprimé

Lorsque vous supprimez un circuit dans VisuGPS, voici ce qui se passe techniquement :

*   **Entrée de base de données** : La référence au circuit est effacée de la liste interne.
*   **Fichiers de configuration** : Les fichiers JSON générés pour ce circuit (paramètres de caméra, évènements, etc.) sont supprimés du dossier de stockage de l'application.
*   **Vignette** : L'image miniature générée est supprimée.

> [!NOTE]
>
>
> Le fichier GPX original sur votre ordinateur n'est jamais impacté par cette suppression. Seules les données importées et les configurations créées dans VisuGPS sont effacées.

---
[< Retour au guide d'exploitation](./exploitation.md)
