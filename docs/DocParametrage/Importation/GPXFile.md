# 📂 Paramètre : Dossier des fichiers GPX

Ce document détaille le paramètre `GPXFile`, qui définit le dossier par défaut à partir duquel l'application importe les fichiers GPX.

---

## 🎯 Rôle du Paramètre

Le paramètre `GPXFile` permet de sélectionner via une interface de sélection de dossier le répertoire que l'application ouvrira par défaut.

-   **Libellé**: Dossier des fichiers GPX
-   **Type**: Dossier (Directory)
-   **Valeur par défaut**: "DEFAULT_DOWNLOADS"

## ⚖️ Justification : Pourquoi définir un dossier par défaut ?

Définir un dossier par défaut améliore l'ergonomie et la rapidité d'accès à vos fichiers GPX.

### 1. 🚀 Rapidité d'Accès

-   Évite de devoir naviguer manuellement vers le dossier de vos traces à chaque importation.
-   Si toutes vos traces sont stockées au même endroit, cela simplifie grandement le processus.

### 2. 📁 Organisation

-   Encourage une meilleure organisation de vos fichiers GPX en les regroupant dans un dossier dédié.

### 3. "DEFAULT_DOWNLOADS"

-   La valeur spéciale "DEFAULT_DOWNLOADS" indique à l'application d'utiliser le dossier de téléchargement par défaut de votre système d'exploitation. C'est un choix pratique si vous téléchargez vos traces directement depuis un site web.

---

## ⚠️ Recommandations

-   **Valeur par défaut ("DEFAULT_DOWNLOADS")** : C'est un bon point de départ, car c'est souvent là que les fichiers téléchargés sont stockés.
-   **Personnaliser si nécessaire** : Si vous avez un dossier spécifique pour vos traces GPX (par exemple, "Mes Traces GPS" sur un disque dur externe), il est recommandé de définir ce chemin ici pour un accès plus rapide.
-   **Chemin absolu** : Le chemin est stocké de manière absolue.
-   **Sélection** : Cliquez sur le paramètre pour ouvrir la fenêtre de sélection de dossier de votre système d'exploitation.
