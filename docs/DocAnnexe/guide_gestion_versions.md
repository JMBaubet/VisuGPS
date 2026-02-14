# 📦 Guide de Gestion des Versions (Beta & Production)

Ce document décrit la procédure pour référencer une version spécifique (comme une Beta) et créer une Release sur GitHub pour le projet **VisuGPS**.

---

## 1. Pré-requis : Harmonisation des Versions

Avant de créer un tag, assurez-vous que les numéros de version sont identiques dans les fichiers de configuration du projet.

**Fichiers à modifier :**

1.  **`package.json`** (racine) :
    ```json
    "version": "1.0.0-beta.1",
    ```

2.  **`src-tauri/Cargo.toml`** :
    ```toml
    [package]
    version = "1.0.0-beta.1",
    ```

3.  **`src-tauri/tauri.conf.json`** :
    ```json
    "version": "1.0.0-beta.1",
    ```

---

## 2. Créer un Tag Git

Un tag ("étiquette") sert à figer un commit précis dans l'historique pour dire "Ceci est la version X".

### Cas A : Taguer le commit actuel (le plus courant)
Si vous êtes satisfait de l'état actuel de votre code :

```bash
# Syntaxe : git tag -a [NOM_DU_TAG] -m "[MESSAGE]"
git tag -a v1.0.0-beta.1 -m "Version Beta 1.0.0 - Test fonctionnalités 3D"
```

### Cas B : Taguer un commit passé
Si vous voulez référencer un commit antérieur (retrouvez son hash via `git log`) :

```bash
# Syntaxe : git tag -a [NOM_DU_TAG] [HASH_COMMIT] -m "[MESSAGE]"
git tag -a v1.0.0-beta.1 9fceb02 -m "Version Beta sur ce commit spécifique"
```

---

## 3. Pousser le Tag vers GitHub

Par défaut, `git push` n'envoie pas les tags. Il faut le faire explicitement :

```bash
git push origin v1.0.0-beta.1
```

---

## 4. Créer la Release sur GitHub

Cette étape permet de rendre la version téléchargeable pour les utilisateurs (avec les installateurs `.msi` ou `.exe` générés).

1.  Allez sur la page GitHub du projet : [https://github.com/JMBaubet/VisuGPS](https://github.com/JMBaubet/VisuGPS)
2.  Cliquez sur **Releases** (dans la barre latérale droite).
3.  Cliquez sur le bouton **Draft a new release**.
4.  **Remplir le formulaire :**
    *   **Choose a tag** : Sélectionnez `v1.0.0-beta.1` (le tag que vous venez de pousser).
    *   **Release title** : Donnez un titre clair (ex: "v1.0.0-beta.1 : Amélioration Vue 3D").
    *   **Describe this release** : Listez les changements majeurs.
5.  **Important pour une Beta** : Cochez la case **"Set as a pre-release"**. Cela indique clairement aux utilisateurs que c'est une version de test.
6.  Cliquez sur **Publish release**.

---

## 5. Automatisation (Optionnel)

Si une "Action GitHub" est configurée pour la CI/CD, la création d'une Release peut déclencher automatiquement la compilation et l'ajout des fichiers d'installation (`.msi`, `.exe`) à la release.
