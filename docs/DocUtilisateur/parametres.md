# Paramètres Globaux

Le menu Paramètres (accessible via l'icône d'engrenage <img src="https://api.iconify.design/mdi/cog.svg?width=20" style="vertical-align: middle; margin-bottom: 3px;"> sur l'accueil) permet de configurer le comportement général de VisuGPS.

[< Retour à l'accueil](./index.md)

## Configuration Générale

### 1. Affichage
*   **Thème** : Basculer entre Mode Sombre (Dark) et Mode Clair (Light).
*   **Unités** : Système Métrique (km, m) ou Impérial (miles, ft).

### 2. Carte et Terrain (Mapbox)
VisuGPS utilise Mapbox pour l'affichage 3D.
*   **Token Mapbox** : Votre clé d'API personnelle. Indispensable pour charger les tuiles de carte et les reliefs.
*   **Style de Carte** : Choix du fond de carte (Satellite, Outdoor, Streets...).

### 3. Dossiers et Stockage
*   **Dossier de Travail** : L'emplacement sur votre ordinateur où sont stockés :
    *   Les fichiers JSON de configuration des traces.
    *   Les vignettes générées.
    *   Les caches de données.

### 4. Mise à jour des Communes
*   **APIs** : Activation/Désactivation des services IGN et Mapbox.
*   **Timers** : Réglage des délais entre les requêtes pour respecter les quotas des APIs.
    *   *En savoir plus sur la **[Mise à jour des Communes](./maj_communes.md)**.*

### 5. Importation et Vignettes
*   **Vignettes** : Configuration du style, des dimensions et des couleurs des miniatures générées.
    *   *Voir le détail dans **[Configuration des Vignettes](./vignette_config.md)**.*

### 6. Gestion des Modes
*   **Environnements** : Création et bascule entre les modes Production (OPE), Évaluation (EVAL) et Test (TEST).
    *   *En savoir plus sur les **[Modes de Fonctionnement](./modes_fonctionnement.md)**.*

## Paramètres de la Télécommande

*   **Port Serveur** : Le port réseau utilisé pour communiquer avec le smartphone (par défaut 9001).
*   **Adresse IP** : L'adresse locale de votre ordinateur (détectée automatiquement).

## Sauvegarde

Les paramètres sont enregistrés automatiquement dans un fichier `settings.json` via le stockage local de l'application.

---
[< Retour à l'accueil](./index.md) | [Suivant : Télécommande >](./telecommande.md)
