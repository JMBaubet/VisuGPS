# 📂 Paramètre : Dossier d'import

Ce document détaille le paramètre `ImportDir`, qui définit le répertoire par défaut proposé lors de l'ouverture de la fenêtre d'importation (GPX, Contextes).

---

Le paramètre `ImportDir` définit le chemin du dossier utilisé par défaut pour deux opérations majeures :
- **L'importation** : l'ouverture de la fenêtre de sélection de fichiers (GPX, Contextes).
- **L'exportation** : le dossier de destination automatique pour vos circuits sauvegardés au format `.vgps`.

L'application mémorise ce chemin pour vous faire gagner du temps en ouvrant directement ce dossier.

-   **Libellé**: Dossier d'import
-   **Type**: Dossier (Directory)
-   **Valeur par défaut**: `DEFAULT_DOWNLOADS` (Votre dossier "Téléchargements")

## ⚖️ Justification : Pourquoi configurer ce dossier ?

### 1. ⏱️ Gain de temps
-   Évite de devoir naviguer dans l'arborescence de fichiers à chaque importation si vos fichiers sont toujours au même endroit.

### 2. 🔄 Flexibilité
-   Bien que ce dossier soit le défaut, vous gardez la liberté de naviguer vers d'autres répertoires via la fenêtre de sélection de fichiers.

---

## ⚠️ Recommandations

-   **Organisation** : Créez un dossier dédié (ex: `VisuGPS/Imports`) pour rassembler vos traces et contextes avant de les importer.
-   **Accès** : Assurez-vous que l'application a les droits de lecture sur le dossier choisi (surtout sur macOS/Linux).
