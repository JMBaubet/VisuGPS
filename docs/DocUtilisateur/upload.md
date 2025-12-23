# Gestion des Traces et Importation

Cette section explique comment importer vos fichiers de parcours dans VisuGPS. L'application accepte deux formats de fichiers : les traces brutes (**GPX**) et les archives complètes (**VGPS**).

[< Retour à l'accueil](./index.md)

## 1. Définition des Formats

*   **Fichier GPX (.gpx)** : C'est le format standard universel pour les tracés GPS. Il contient uniquement les coordonnées géographiques et l'altitude. Lors de son importation, VisuGPS effectue une analyse automatique pour calculer les statistiques et générer une vignette.
*   **Fichier VGPS (.vgps)** : C'est un format propre à VisuGPS. Il s'agit d'une archive exportée qui contient non seulement la trace, mais aussi toute la "mise en scène" associée (positions de caméras, événements de survol, messages textuels).

## 2. Procédure d'Importation

L'interface de sélection est identique pour les deux types de fichiers.

1.  **Lancer l'import** : Sur l'écran d'accueil, cliquez sur le bouton **Importer** <img src="https://api.iconify.design/mdi/file-import-outline.svg?width=20" style="vertical-align: middle; margin-bottom: 3px;"> situé en haut à droite de la barre de l'application.
2.  **Parcourir les fichiers** : Une fenêtre de navigation s'ouvre sur votre dossier d'importation par défaut (configurable dans les paramètres).
3.  **Choisir le type de fichier** : En bas à gauche de la fenêtre, un sélecteur permet de choisir entre **"GPX"** (sélectionné par défaut) et **"VGPS"**. Assurez-vous d'avoir sélectionné le format correspondant à votre fichier.
4.  **Navigation et Filtrage** :
    *   **Navigation** : Vous pouvez cliquer sur les dossiers pour naviguer dans votre arborescence.
    *   **Filtrage** : Utilisez la barre de recherche en haut pour retrouver rapidement un fichier par son nom.
5.  **Sélectionner le fichier** : Cliquez sur le fichier souhaité pour valider votre choix.

### Distinction selon le format sélectionné

Après la sélection, le comportement de l'application diffère :

*   **Pour un fichier GPX** : 
    *   **Choix du Traceur** : Une boîte de dialogue s'ouvre pour attribuer obligatoirement un auteur à la trace (sélectionnez un traceur existant ou créez-en un nouveau).
    *   **Analyse** : L'application lance l'analyse automatique des données (voir section suivante).
*   **Pour un fichier VGPS** : 
    *   **Restauration** : L'importation est immédiate. L'application recrée le circuit avec tous ses paramètres d'édition originaux. Aucune saisie ni analyse n'est requise.

## 3. Analyse Automatique (GPX uniquement)

Lors de l'importation d'une trace brute, VisuGPS calcule les indicateurs suivants :
*   **Distance totale** : Vérifiée et recalculée par projection.
*   **Dénivelé positif** : Cumul des gains d'altitude.
*   **Point culminant** : Identification de l'altitude maximale.
*   **Ville de départ** : Identification automatique de la commune de départ.
*   **Vignette** : Une image miniature du parcours est générée pour la liste. (voir [Configuration des Vignettes](./vignette_config.md))

## Accès au Circuit

Une fois l'importation terminée, votre circuit apparaît immédiatement dans la liste principale. Vous pouvez cliquer dessus pour consulter ses détails ou passer en mode édition.

---
[< Retour à l'accueil](./index.md) | [Suivant : Gestion des Traces >](./gestion_traces.md)
