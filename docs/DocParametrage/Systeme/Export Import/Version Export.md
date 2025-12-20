# 🔢 Paramètre : Version Export

Ce document détaille le paramètre `Version Export`, qui définit le format de données utilisé lors de la création des fichiers d'archive de circuit.

---

## 🎯 Rôle du Paramètre

Le paramètre `Version Export` assure la compatibilité technique des fichiers `.vgps` générés par l'application.

-   **Libellé**: Version Export
-   **Type**: Texte
-   **Valeur par défaut**: 1.0
-   **Valeurs disponibles**: 1.0 (Version actuelle)

## ⚖️ Justification : Pourquoi ce paramètre ?

Bien que ce paramètre soit technique, il est crucial pour la maintenance et l'évolution de l'application.

### 1. 🔄 Compatibilité Ascendante
Si le format des données change dans le futur (nouvelles fonctionnalités, structure différente), ce numéro permettra aux versions futures de l'application de savoir comment lire les anciens fichiers.

### 2. 🛠️ Débogage et Support
En cas de problème lors d'un import, connaître la version du format d'export permet d'identifier rapidement si le fichier est obsolète ou corrompu.

---

## ⚠️ Fonctionnement

-   **Informatif** : Ce paramètre est principalement informatif. Il est déconseillé de le modifier manuellement à moins d'instructions spécifiques du support technique.
-   **Format Fixe** : La valeur "1.0" correspond au standard initial de l'application VisuGPS pour les fichiers `.vgps`.
