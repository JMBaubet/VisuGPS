# Gestion des Traces et Importation

Cette section explique comment importer vos fichiers GPX dans VisuGPS et gérer votre bibliothèque de circuits.

[< Retour à l'accueil](./index.md)

## 1. Importation d'un Fichier GPX

Pour ajouter un nouveau circuit à votre bibliothèque :

1.  Sur l'écran d'accueil, cliquez sur le bouton **Importer** ![import](https://api.iconify.design/mdi/file-import-outline.svg?width=28) situé en haut à droite de la barre de l'application.
2.  Une fenêtre de sélection de fichier s'ouvre. Naviguez jusqu'à votre fichier `.gpx` et validez.
3.  **Choix du Traceur** : Une boîte de dialogue s'ouvre pour attribuer un auteur à la trace.
    *   Sélectionnez un traceur existant dans la liste.
    *   Ou créez-en un nouveau en saisissant son nom.
    *   *Cette étape est obligatoire.*
4.  L'application finalise l'importation.

### Analyse Automatique
Lors de l'importation, VisuGPS effectue les calculs suivants :
4.  L'application finalise l'importation.

### Analyse Automatique
Lors de l'importation, VisuGPS effectue les calculs suivants :
*   **Distance totale** : Calculée en projetant la trace sur la carte.
*   **Dénivelé** : Cumul des gains d'altitude positifs.
*   **Vignette** : Une image miniature du parcours est générée pour la liste.

## Paramètres Liés

Les fonctionnalités d'importation dépendent de certains paramètres globaux (accessibles dans le menu [Paramètres](./parametres.md)) :

*   **Suppression après importation** : Si activé, le fichier GPX original est supprimé de votre ordinateur une fois importé avec succès.
*   **Dossier de stockage** : L'emplacement où VisuGPS enregistre les fichiers de configuration (JSON) et les vignettes.
*   **Clé Mapbox** : Nécessaire pour générer les fonds de carte des vignettes et récupérer les altitudes si elles manquent dans le GPX.

---
[< Retour à l'accueil](./index.md) | [Suivant : Gestion des Traces >](./gestion_traces.md)
