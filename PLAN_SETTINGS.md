# Plan d'action pour la migration des paramètres (settings.json)

Ce document décrit le plan d'action pour la mise en place d'un système de migration automatique des fichiers de paramètres `settings.json` de l'utilisateur lors d'une mise à jour de l'application.

## Objectif

L'objectif est d'automatiser la mise à jour du fichier `settings.json` de l'utilisateur lorsqu'une nouvelle version du fichier de référence `src-tauri/settingsDefault.json` est disponible. Cette automatisation doit préserver les surcharges de l'utilisateur tout en garantissant leur compatibilité avec les nouvelles contraintes (limites, options) des paramètres.

---

## Phases du projet

### Phase 1 : Détection et déclenchement de la migration

Cette phase consiste à déterminer si une migration est nécessaire au démarrage de l'application.

**Actions :**
1.  Au lancement, comparer le champ `référence.version` du fichier `settings.json` de l'utilisateur avec celui de `src-tauri/settingsDefault.json`.
2.  Si les versions sont différentes, déclencher le processus de migration.
3.  Si le fichier `settings.json` de l'utilisateur n'existe pas, le créer à partir de `settingsDefault.json` sans lancer de migration.

**Mise en œuvre des tests de validation :**
Les tests pour cette phase seront des tests unitaires (avec `vitest`). Il faudra mocker les fonctions de lecture du système de fichiers pour simuler les différents scénarios.

-   **Test 1.1 : Versions identiques**
    -   **Implémentation :** Mocker la lecture de `settings.json` et `settingsDefault.json` pour qu'ils retournent des objets avec le même numéro de version (ex: `1.0.0`).
    -   **Attendu :** La fonction principale de migration `migrateSettings()` n'est jamais appelée.
-   **Test 1.2 : Version utilisateur inférieure**
    -   **Implémentation :** Mocker la lecture pour que `settings.json` ait une version `1.0.0` et `settingsDefault.json` une version `1.1.0`.
    -   **Attendu :** La fonction `migrateSettings()` est appelée.
-   **Test 1.3 : Fichier `settings.json` absent**
    -   **Implémentation :** Mocker la lecture de `settings.json` pour qu'elle retourne une erreur (fichier non trouvé). Mocker la fonction d'écriture.
    -   **Attendu :** La fonction de création de fichier est appelée avec le contenu de `settingsDefault.json`. La fonction `migrateSettings()` n'est pas appelée.

---

### Phase 2 : Préparation et analyse des données

Cette phase a pour but de charger et de structurer les données des deux fichiers de configuration pour faciliter leur comparaison.

**Actions :**
1.  Charger en mémoire le contenu de `src-tauri/settingsDefault.json` (la nouvelle référence).
2.  Charger en mémoire le contenu du `settings.json` de l'utilisateur (l'ancienne configuration).
3.  "Aplatir" la structure hiérarchique des paramètres des deux fichiers dans des Maps (ou dictionnaires) où la clé est `l'identifiant` du paramètre et la valeur est l'objet paramètre complet. Cela permettra un accès direct et rapide.
4.  Identifier les listes de paramètres :
    -   `newParams`: Paramètres présents uniquement dans la nouvelle référence.
    -   `deletedParams`: Paramètres présents uniquement dans l'ancienne configuration.
    -   `commonParams`: Paramètres présents dans les deux fichiers.

**Mise en œuvre des tests de validation :**
Tests unitaires (`vitest`) sur la logique de comparaison, en utilisant des objets JSON fictifs.

-   **Test 2.1 : Test de la fonction d'aplatissement**
    -   **Implémentation :** Créer un objet de settings hiérarchique fictif. Appeler la fonction d'aplatissement dessus.
    -   **Attendu :** La Map retournée contient tous les paramètres, accessibles par leur `identifiant`.
-   **Test 2.2 : Test de la comparaison**
    -   **Implémentation :** Créer deux Maps de paramètres fictives : `oldSettingsMap` et `newSettingsMap`.
        -   `oldSettingsMap` contiendra les paramètres A (commun), B (supprimé).
        -   `newSettingsMap` contiendra les paramètres A (commun), C (nouveau).
    -   **Attendu :** La fonction de comparaison retourne un objet `{ newParams: [C], deletedParams: [B], commonParams: [A] }`.

---

### Phase 3 : Processus de migration des paramètres communs

C'est le cœur du processus. On construit le nouveau fichier de configuration en fusionnant l'ancienne et la nouvelle version.

**Actions :**
1.  Créer un nouvel objet de configuration, initialisé comme une copie profonde de `settingsDefault.json`.
2.  Parcourir la liste `commonParams`. Pour chaque paramètre commun :
    a. Récupérer la surcharge de l'utilisateur depuis l'ancienne configuration (`surcharge` n'est pas `null`).
    b. Si une surcharge existe :
        i. Valider cette surcharge par rapport aux nouvelles contraintes (`min`, `max`, `options`, `type`) définies dans la nouvelle référence.
        ii. **Si la surcharge est valide :** La recopier dans le nouvel objet de configuration.
        iii. **Si la surcharge est invalide :** Marquer ce paramètre comme "requérant une intervention utilisateur" et conserver l'ancienne valeur temporairement.
3.  Les paramètres de `newParams` sont déjà dans le nouvel objet avec leurs valeurs par défaut.
4.  Les paramètres de `deletedParams` sont ignorés et ne seront pas dans le nouveau fichier.

**Mise en œuvre des tests de validation :**
Tests unitaires (`vitest`) sur la logique de fusion.

-   **Test 3.1 : Surcharge valide conservée**
    -   **Implémentation :** Définir un paramètre commun avec une surcharge valide (ex: `valeur: 5`, `min: 0`, `max: 10` dans les deux versions).
    -   **Attendu :** Le paramètre dans la configuration finale a bien la surcharge `5`.
-   **Test 3.2 : Surcharge invalide (limites)**
    -   **Implémentation :** Définir un paramètre avec une surcharge `15`. L'ancienne `max` était `20`, la nouvelle est `10`.
    -   **Attendu :** La fonction de migration retourne une liste de "paramètres à résoudre" qui contient ce paramètre.
-   **Test 3.3 : Surcharge invalide (type)**
    -   **Implémentation :** Définir un paramètre avec une surcharge de type `string` "ancienne_valeur". Le nouveau type est `entier`.
    -   **Attendu :** Le paramètre est ajouté à la liste des "paramètres à résoudre".

---

### Phase 4 : Gestion de l'intervention utilisateur

Pour les cas où une surcharge est devenue invalide, il faut demander à l'utilisateur de choisir une nouvelle valeur.

**Actions :**
1.  Créer un composant Vue (ex: `MigrationDialog.vue`) qui s'affichera en superposition au démarrage si des interventions sont nécessaires.
2.  Ce composant itérera sur la liste des paramètres marqués comme "requérant une intervention utilisateur".
3.  Pour chaque paramètre, il affichera dynamiquement le dialogue d'édition approprié (`EditIntDialog`, `EditFloatDialog`, `EditBoolDialog`, etc.) en lui passant les nouvelles contraintes.
4.  Le dialogue d'édition devra empêcher l'utilisateur de sauvegarder une valeur qui ne respecte pas les nouvelles contraintes.
5.  La nouvelle valeur choisie par l'utilisateur mettra à jour le nouvel objet de configuration.
6.  Le processus de migration ne se termine que lorsque toutes les interventions ont été traitées.

**Mise en œuvre des tests de validation :**
Tests de composants avec `@vue/test-utils` et `vitest`.

-   **Test 4.1 : Affichage du dialogue principal**
    -   **Implémentation :** Monter le composant `App.vue` en simulant un état de "migration requise avec intervention".
    -   **Attendu :** Le composant `MigrationDialog.vue` est présent dans le DOM.
-   **Test 4.2 : Affichage du sous-dialogue correct**
    -   **Implémentation :** Monter `MigrationDialog.vue` avec un paramètre invalide de type `entier`.
    -   **Attendu :** Le composant `EditIntDialog.vue` est rendu par le dialogue principal.
-   **Test 4.3 : Validation dans le sous-dialogue**
    -   **Implémentation :** Monter `EditIntDialog.vue` avec les props `min: 0` et `max: 10`. Simuler une saisie de `15` par l'utilisateur.
    -   **Attendu :** Un message d'erreur est visible et l'événement `save` ne peut pas être émis (bouton désactivé).

---

### Phase 5 : Finalisation et rapport de migration

Une fois la migration terminée (avec ou sans intervention utilisateur), il faut la finaliser et en informer l'utilisateur.

**Actions :**
1.  Sauvegarder le nouvel objet de configuration dans le fichier `settings.json` de l'utilisateur, en écrasant l'ancien. Le numéro de version doit être mis à jour.
2.  Générer un rapport de migration en Markdown (`migration-report-V1.x.x-to-V2.x.x.md`) dans un dossier dédié (ex: `user_data/migration_reports/`).
3.  Ce rapport listera :
    -   Les nouveaux paramètres ajoutés avec leur valeur par défaut.
    -   Les paramètres supprimés.
    -   Les paramètres dont la surcharge a été invalidée et qui ont nécessité une mise à jour manuelle.
4.  **Créer un nouveau composant `MigrationReportDialog.vue`**. Ce composant sera appelé à la fin du processus de migration. Il affichera un résumé clair du rapport (nombre d'ajouts, de suppressions, etc.) et proposera un bouton pour ouvrir le fichier de rapport Markdown complet.

**Mise en œuvre des tests de validation :**
Tests unitaires pour la génération du rapport et tests de composants pour son affichage.

-   **Test 5.1 : Écriture du fichier final**
    -   **Implémentation :** Mocker la fonction d'écriture du système de fichiers. Lancer une migration complète fictive.
    -   **Attendu :** La fonction d'écriture est appelée avec un objet de configuration qui contient les bonnes données (surcharges conservées, nouvelles valeurs, etc.) et le nouveau numéro de version.
-   **Test 5.2 : Génération du rapport**
    -   **Implémentation :** Lancer la fonction de génération de rapport avec des listes fictives de paramètres ajoutés, supprimés et modifiés.
    -   **Attendu :** La chaîne de caractères Markdown générée contient les bonnes sections et les bonnes informations.
-   **Test 5.3 : Affichage du rapport**
    -   **Implémentation :** Monter le composant `MigrationReportDialog.vue` avec un résumé de rapport fictif.
    -   **Attendu :** Le composant affiche correctement les informations du résumé et le bouton pour voir le rapport complet est fonctionnel (il émet un événement ou appelle une commande Tauri pour ouvrir le fichier).
