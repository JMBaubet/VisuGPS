# 🗑️ Paramètre : Suppression automatique après importation

Ce document détaille le paramètre `autoDelete`, qui contrôle si les fichiers GPX originaux sont automatiquement supprimés après une importation réussie dans l'application.

---

## 🎯 Rôle du Paramètre

Le paramètre `autoDelete` est un interrupteur (booléen) qui, lorsqu'activé, indique à l'application de supprimer le fichier `.gpx` de votre système de fichiers une fois qu'il a été correctement importé et traité.

-   **Libellé**: Suppression après importation
-   **Type**: Booléen
-   **Valeur par défaut**: `false`

## ⚖️ Justification : Pourquoi utiliser la suppression automatique ?

La suppression automatique peut aider à maintenir votre dossier de téléchargement ou d'importation organisé, mais elle doit être utilisée avec prudence.

### 1. 📂 Organisation du Système de Fichiers

-   Lorsque vous importez de nombreuses traces GPX, votre dossier d'importation peut rapidement se remplir de fichiers que vous n'utiliserez plus. La suppression automatique aide à garder ce dossier propre.

### 2. ⚠️ Conservation des Données Originales

-   Il est crucial de considérer si vous avez besoin de conserver le fichier GPX original pour d'autres usages (archivage, partage, utilisation dans d'autres logiciels). Si c'est le cas, la suppression automatique n'est pas recommandée.

---

## ⚠️ Recommandations

-   **Désactivé par défaut** : Il est désactivé par défaut pour des raisons de sécurité des données, afin de s'assurer que l'utilisateur ne perde pas involontairement ses fichiers originaux.
-   **Activer avec prudence** : N'activez ce paramètre que si vous êtes certain de ne pas avoir besoin des fichiers GPX originaux après leur importation dans VisuGPS.
-   **Sauvegarde** : Assurez-vous d'avoir des sauvegardes de vos fichiers GPX si vous activez cette option et que la conservation des données est importante pour vous.
