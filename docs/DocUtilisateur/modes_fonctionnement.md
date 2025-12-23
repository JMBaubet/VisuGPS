# Modes de Fonctionnement

VisuGPS intègre un système de gestion d'environnements multiples, vous permettant de séparer vos données selon l'usage (Production, Évaluation, Test).

[< Retour à l'accueil](./index.md)

## 1. Principes Généraux

Chaque mode de fonctionnement agit comme un espace de travail totalement indépendant.
Lorsque vous changez de mode :
*   Vos **Traces/Circuits** sont différents.
*   Vos **Paramètres** sont spécifiques à ce mode.
*   Le dossier de stockage des données sur votre ordinateur change.

Cela vous permet de tester de nouvelles fonctionnalités ou de faire des expérimentations sans risquer de perdre ou corrompre vos données principales.

## 2. Types de Modes

Il existe deux types d'environnements accessibles à l'utilisateur :

### Mode Production (OPE)
*   **Usage** : Utilisation quotidienne et normale de l'application.
*   **Indicateur** : Aucun badge ni cadre de couleur.
*   **Dossier** : Environnement par défaut.

### Mode Évaluation (EVAL)
*   **Usage** : Pour tester une nouvelle version de l'application ou s'entraîner à utiliser de nouvelles fonctionnalités sans affecter vos données de production.
*   **Indicateur** : Badge **<span style="background-color: rgba(41, 121, 255, 0.1); color: #2979FF; padding: 2px 8px; border-radius: 4px;">EVAL</span>** (Bleu) et **cadre Bleu** autour de l'application.
*   **Dossier** : `VisuGPS/EVAL_[nom]`

## 3. Gestion des Modes

La gestion des environnements se fait depuis les **Paramètres**.

1.  Ouvrez les **Paramètres** <img src="https://api.iconify.design/mdi/cog.svg?width=20" style="vertical-align: middle; margin-bottom: 3px;">.
2.  Cliquez sur l'icône **Gestion des Modes** <img src="https://api.iconify.design/mdi/database-cog-outline.svg?width=20" style="vertical-align: middle; margin-bottom: 3px;"> dans la barre d'outils des paramètres.

Une fenêtre s'ouvre vous permettant de :
*   **Voir le mode actuel** et la liste des modes disponibles.

*   **Créer un nouveau mode** : Donnez-lui un nom et une description.
    > [!IMPORTANT]
    >
    > Le nom du mode doit **obligatoirement** commencer par le préfixe `EVAL_` et ne contenir que des caractères alphanumériques et des underscores. 
    > *   Exemple correct : `EVAL_Version_2_0`
    > *   Exemple incorrect : `EVAL_Version 2`

*   **Changer de mode** : Vous pouvez sélectionner un autre environnement dans la liste en cliquant sur l'icône <img src="https://api.iconify.design/mdi/check.svg?color=green&width=20" style="vertical-align: middle; margin-bottom: 3px;">. 
    > [!NOTE]
    >
    > Le changement de mode nécessite généralement un redémarrage de l'application pour être pris en compte intégralement.

*   **Supprimer un mode** : Vous pouvez effacer un environnement d'évaluation s'il n'est plus utile et non utilisé en cliquant sur l'icône <img src="https://api.iconify.design/mdi/delete.svg?color=red&width=20" style="vertical-align: middle; margin-bottom: 3px;">.
    > [!NOTE]
    >
    > Le mode OPE ne peut pas être supprimé.

*   **Exporter un contexte** : Sauvegardez un environnement complet (circuits, paramètres, données) sous forme d'archive `.vctx` en cliquant sur l'icône <img src="https://api.iconify.design/mdi/database-export-outline.svg?color=blue&width=20" style="vertical-align: middle; margin-bottom: 3px;">.

*   **Importer un contexte** : Restaurez un environnement depuis un fichier `.vctx` ou `.vgps` en cliquant sur l'icône <img src="https://api.iconify.design/mdi/database-import-outline.svg?color=orange&width=20" style="vertical-align: middle; margin-bottom: 3px;">.
    Une fenêtre de navigation s'ouvre pour vous permettre de sélectionner le fichier archive.
    > [!WARNING]
    >
    > L'importation **ÉCRASE IRRÉVERSIBLEMENT** toutes les données existantes du mode ciblé. Soyez vigilant, en particulier avec le mode de Production (OPE). Une demande de confirmation vous sera faite.

---
[< Retour à l'accueil](./index.md)
