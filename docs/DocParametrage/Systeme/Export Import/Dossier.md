# 📂 Paramètre : Dossier Export Import

Ce document détaille le paramètre `Dossier`, qui permet de définir l'emplacement de stockage pour les fichiers de circuits exportés ou à importer.

---

## 🎯 Rôle du Paramètre

Le paramètre `Dossier` indique au système le répertoire par défaut à utiliser lors de la création d'archives `.vgps` (exportation) ou lors de la recherche de fichiers (importation).

-   **Libellé**: Dossier Export Import
-   **Type**: Dossier
-   **Valeur par défaut**: DEFAULT_DOWNLOADS (Dossier Téléchargements)
-   **Valeurs disponibles**: Tout chemin de dossier valide sur le système.

## ⚖️ Justification : Pourquoi configurer ce dossier ?

Ce réglage offre une flexibilité dans l'organisation de vos fichiers, s'adaptant à vos habitudes de travail.

### 1. 📂 Organisation Personnalisée
Au lieu de mélanger vos circuits avec vos autres téléchargements Internet, vous pouvez créer un dossier dédié (ex: `Documents/MesCircuitsGPS`) pour tout centraliser.

### 2. ☁️ Synchronisation Cloud
Si vous définissez un dossier situé dans un espace synchronisé (Dropbox, Google Drive, OneDrive), vos exports seront automatiquement sauvegardés et accessibles sur vos autres appareils.

---

## ⚠️ Fonctionnement

-   **Mode par défaut (Téléchargements)** : Si vous ne modifiez rien, ou si vous réinitialisez le paramètre, les fichiers seront enregistrés dans le dossier standard de téléchargement de votre système d'exploitation.
-   **Sélection** : L'interface utilise une fenêtre de sélection de dossier native pour garantir que le chemin est valide et accessible.
-   **Persistance** : Le choix est mémorisé et sera proposé automatiquement lors de chaque future opération d'exportation.
