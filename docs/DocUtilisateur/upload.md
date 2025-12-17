# Gestion des Traces et Importation

Cette section explique comment importer vos fichiers GPX dans VisuGPS et gérer votre bibliothèque de circuits.

[< Retour à l'accueil](./index.md)

## 1. Importation d'un Fichier GPX

Pour ajouter un nouveau circuit à votre bibliothèque :

1.  Sur l'écran d'accueil, cliquez sur le bouton **Importer** ![import](https://api.iconify.design/mdi/file-import-outline.svg?width=28) situé en haut à droite de la barre de l'application.
2.  Une fenêtre de sélection de fichier s'ouvre. Elle filtre automatiquement les fichiers `.gpx`. Par défaut c'est le dossier Téléchargements qui est sélectionné. 
Vous pouvez filtrer les noms de fichiers avec la barre de recherche en haut de la fenêtre. 
Naviguez jusqu'à votre fichier `.gpx` et validez.
3.  **Choix du Traceur** : Une boîte de dialogue s'ouvre pour attribuer un auteur à la trace.
    *   Sélectionnez un traceur existant dans la liste.
    *   Ou créez-en un nouveau en saisissant son nom.
    *   *Cette étape est obligatoire.*
4.  L'application finalise l'importation.

### Analyse Automatique
Lors de l'importation, VisuGPS effectue les calculs suivants :
*   **Distance totale** : Vérifiée et recalculée par projection.
*   **Dénivelé positif** : Cumul des gains d'altitude.
*   **Point culminant** : Identification de l'altitude maximale et de sa position.
*   **Ville de départ** : Identification automatique de la commune de départ.
*   **Code QR** : Génération d'un Code QR avec le lien source du fichier GPX.
*   **Vignette** : Une image miniature du parcours est générée pour la liste. (voir [Configuration des Vignettes](./vignette_config.md))

## Accès au Circuit

Une fois ces étapes terminées, votre nouvelle trace apparaît immédiatement dans la liste principale sur l'écran d'accueil.
Vous pouvez cliquer dessus pour consulter ses détails. (voir [Détails du Circuit](./circuit_details.md))

## Paramètres Liés

Les fonctionnalités d'importation dépendent de certains paramètres globaux (accessibles dans le menu [Paramètres](./parametres.md)) :

*   **Suppression après importation** : Si activé, le fichier GPX original est supprimé de votre ordinateur une fois importé avec succès.
*   **Dossier de stockage** : L'emplacement où VisuGPS enregistre les fichiers de configuration (JSON) et les vignettes.
*   **Clé Mapbox** : Nécessaire pour générer les fonds de carte des vignettes.
*   **Configuration des Vignettes** : Personnalisez le style, les couleurs et les marqueurs de vos miniatures. (voir [Configuration des Vignettes](./vignette_config.md))

---
[< Retour à l'accueil](./index.md) | [Suivant : Gestion des Traces >](./gestion_traces.md)
