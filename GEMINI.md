# 🗺️ Projet VisuGPS : Visualisation 3D de fichiers GPX

Ce document décrit le projet **VisuGPS**, une application de bureau multiplateforme pour la visualisation 3D de fichiers GPX, axée sur la création d'animations. Le but finale de cette application sera de'avoir un flyover d'une trace gpx.

---

## 🚀 Démarrage rapide

### Prérequis

* **Node.js** (version 18 ou supérieure)
* **Rust** et **Cargo**
* **Mapbox GL JS** et **Turf.js** pour l'analyse spatiale.

### Installation et exécution

1. Clonez le dépôt : `git clone https://github.com/JMBaubet/VisuGPS`
2. Installez les dépendances : `npm install`
3. Lancez l'application en mode développement : `npm run tauri dev`

---

## 🛠️ Technologies utilisées

* **Tauri** : Pour la création d'applications de bureau natives.
* **Vue.js 3** & **Vuetify** : Pour une interface utilisateur réactive et élégante.
* **Mapbox GL JS** : Pour l'affichage de la carte 3D.
* **Turf.js** : Pour l'analyse et le traitement des données géospatiales (calcul de longueur, etc.).
* **Fichiers JSON** : Utilisés pour la sauvegarde des paramètres d'édition et de configuration.
* **Material Design Icons** : pour sa quantité d'icones disponible

---

## 📂 Organisation du projet et des vues

L'application est divisée en plusieurs vues distinctes pour une meilleure clarté et gestion du flux de travail :

- ### Vue Principale (VueJS) `src/views/MainView.vue`
  
  Cette vue affiche une liste des traces GPX déjà téléchargées.
  
  - **Liste des traces** : Chaque élément de la liste présente une vignette 2D de la trace.
  
  - **Vignette 2D** : Créée avec Mapbox ou une autre librairie, elle affiche la trace avec un fond de carte simple.
  
  - **Bouton de téléchargement** : Permet à l'utilisateur de charger un nouveau fichier GPX depuis son dossier de téléchargement.
    
    À la suite du téléchargement, les actions suivantes sont automatiquement effectuées :
1. **Calcul de la longueur** : Utilisation de la librairie **Turf.js** pour déterminer la longueur de la trace.

2. **Génération d'une vignette** : Création et affichage de la miniature 2D pour la liste.
   
   - ### Vue Édition (VueJS) `src/viewsEditView.vue`
   
   Cette vue est dédiée à la configuration de la vue 3D et des animations.
- **Visualisation 3D** : La trace GPX est affichée dans un environnement 3D interactif.

- **Positionnement de la caméra** : L'utilisateur peut ajuster la position et l'angle de la caméra.

- **Sauvegarde des paramètres** : Les paramètres de la caméra, essentiels pour la future animation, sont sauvegardés dans un fichier JSON.
  
  - ### Vue Visualisation(VueJS) `src/views/VisualizeView.vue`
  
  Cette vue permet de lire le fichier JSON créé en mode Édition pour visualiser une ou plusieurs traces avec une animation automatique.

- **Animation du marqueur** : Un marqueur se déplace le long de la trace selon les paramètres de caméra et de temps définis.
  
  - ### Vue Paramétrage (VueJS) `src/views/SettingsView.vue`
  
  Cette vue sert à configurer les paramètres globaux de l'application, qui sont stockés dans un fichier JSON dédié.

---

# Implémentation step by step

Pour créer cette application nous allons travailler étape par étape. 

Toi en tant qu'expert Tauri, javascript, vue, vuetity, tu va m'assisteras à créer le code nécessaire à la réalisation de l'étape. 

On gardera dans ce fichier l'historique des étapes, afin que tu saches quelles sont les étapes déjà réalisées  et les prinicpes mis en oeuvre. 

Lorsqu'une étape me semble complètement réalisée, cette dernère au niveau de son titre `heading 2` aura un sufixe `Réalisée`.

Les étapes seront numérotées sous la forme Etape x.y.z 

- x : représentant un avancement majeur de l'application

- y : représentéant une nouvelle feature de l'application

- z : représentant, un point de control de la feature en cours de réalisation

Afin que tu n'ancitipes pas le codage de certaines features, les étapes te seront décrite au fur et à mesure. 

A chaque réalisation d'étape, j'enregistrerai dans ce fichier l'identifiant du commit git

## Etape 0 - Réalisée

l'étape 0 consiste à mettre en place l'environement technique. Elle n'a pas vocation à créer les ihm. les vues seront crées uniquement afin de pouvoir configurer le router. 

### Etape 0.1 - Réalisée - 8afea17

Mise en place de git :

`.gitignore`

`CONTRIBUTING.md`

`README.md`

`LISENCE`

### Etape 0.2 - Réalisée - 59988f9

Mise en place de gemini

`GEMINI.md`

### Etape 0.3 - Réalisée - 50a5f12

le npm initiale pour avoir les packages nécessaires.

### Etape 0.4 - Réalisée - dd389466

Configuration de

Tauri, vueJS, du routeur

Création des vues :

- `src/views/MainView.vue` pour la vue principale

- `src/views/EditView.vue` pour l'edition et la vérification du flyover

- `src/views/VisualizeView.vue`pour le rendu 3D du flyover. 

- `src/views/settingsView.vue` pour le paramétrage de l'application

### Etape 0.5 - Réalisée - [74249be4]

Mise en place de l'environnement d\'éxécution

Dans le processus de développement de l'application nous avons deux **Environnements de travail** :

- L'environnement de  **Dev** lorsqu'on lance la commande `npm run dev`

- L'environnemnt de **Prod**  lorsqu'on lance la commande `npm run tauri build` pour générer la livraison de l'application.

Afin de pouvoir faire des tests fonctionnels ou de la validation de configuration sans impacter les fichiers de configuration utilisés en production, nous allons avoir besoin d'un **Environnement d\'éxécution**. Il faut bien faire la distinction entre ces deux types d'environnement.

Les **Environnements d\'éxécution** devront être disponibles dans les deux **Environnements de travail**. 

Pour changer d'environnement d\'éxecution, il sera nécessaire de relancer l'application.

L'environnement d'exécution sera défini par la variable `APP_ENV` dans le fichier `.env`. Les valeurs possibles pour cette variable suivent une convention précise :

- `OPE` : Pour l\'utilisation normale de l\'application. Ce sera la valeur par défaut si le fichier `.env` est absent.
- `EVAL_NOM_FEATURE` : Pour la validation d\'une configuration ou d\'une fonctionnalité spécifique (ex: `EVAL_Camera`).
- `TTEST_NOM_TEST` : Pour l\'exécution d\'un test fonctionnel particulier (ex: `TEST_CalculLongueur`).

À partir de cette variable, l'application devra déduire un **mode d\'exécution** (par exemple, dans une variable `executionMode`) qui pourra être `OPE`, `EVAL`, ou `TEST`. Cela permettra d'adapter le comportement de l'application en fonction du contexte (ex: charger des données de test, afficher des informations de debug, etc.).

Le fichier `.env`, ne sera pas livré avec l'application, car il contiendra plus tard des tokens privés. 

Le fichier .env si il existe sera présent sous `C:\Users\username\AppData\Roaming\com.visugps.app\VisuGPS`

Il devra être lu à partir de ce dossier que l'on soit en mode de travail Dev ou Prod. Si ce ficheier n'existe pas au lancement de l'application, il devra être créé, pour avoir un mode d\'éxécution `OPE` que l'on soit en mode de travail dev ou  Prod

#### Répertoires d'Environnement

En plus de déterminer le mode d\'exécution, l'application crée un dossier de travail dont le nom correspond à la valeur de la variable `APP_ENV`. Ce dossier est créé à l\'intérieur du répertoire de données de l\'application (`C:\Users\username\AppData\Roaming\com.visugps.app\VisuGPS\`).

Par exemple, si `APP_ENV` est `Test_Setup`, le dossier suivant sera créé : `C:\Users\username\AppData\Roaming\com.visugps.app\VisuGPS\Test_Setup\`.

Tous les fichiers de configuration, de test ou de données spécifiques à un environnement dev devront être lus et écrits dans ce répertoire.

#### État de l'Application Partagé

Les informations sur l\'environnement sont centralisées dans un état (`AppState`) géré par Tauri et accessible depuis le frontend. Cet état contient :

- `app_env`: La valeur brute de la variable `APP_ENV`.
- `execution_mode`: Le mode déduit (`OPE`, `EVAL`, `TEST`).
- `app_env_path`: Le chemin complet vers le répertoire de l\'environnement de travail.

### Etape 0.6 - Réalisée - [4a4db936]

Dans cette étape nous nous assurons que vuetify est correctement installé, et on va configurer un cadre en fonction du contexte d\'éxécutiuon.

On ajoute juste un composant `v-btn` à la mainView, pour vérifier qu'il s'affiche correctement.

Dans `App.vue` on ajoute un `v-container` pour avoir ou pas (en prod) un cadre autour de l'application qui devra changer en fonction du contexte d\'éxécution.

### Etape 0.7 -Réalisée - [705e12e2]

Dans `App.vue` on ajoute un `v-container` pour avoir ou pas (en prod) un cadre autour de l'application qui devra changer en fonction du contexte d\'éxécution.

---

## Etape 1

Dans cette étape nous allons travailler sur le composant AppBar pour la MainView.vue.

Dans cette AppBar on doit retrouver : 

- Un indicateur d'état des services réseau 

- un chip pour nous indiquer le nom du contexte d'execution 

- Un switch pour passer d'un mode sombre à un mode claire

### Etape 1.1 - Réalisée -[1ab7eb9a]

Dans cette étape nous allons créer l'AppMainBar pour y inclure les modes sombre/clair, et le chip pour nous afficher la variable APP_ENV si nous sommes en mode EVAL ou TEST

### Etape 1.2 - Réalisée - [d7f6f3b6]

Pour cette étape nous allons nous attaché à la connexion avec mapbox.

Le token sera enregistré dans le fichier .env situé pour rappel dans : `C:\Users\username\AppData\Roaming\com.visugps.app\VisuGPS`.

On va ajouter un icone à la gauche de la toolBar pour indiquer l'état des services. 

Nous allons également utiliser ShowSnackBar pour informer l'utlisateur des chagement d'état.

Nous allons verifier en permance: 

- L'état de la connexion internet.

- La presence du serveur MapBox

- La validité du token MapBox 

## Etape 3

Dans cette étape nous allons travailler principalement sur le vue SettingsView.vue.

A partir de cette vue nous allons pouvoir paramétrer l'application. Tous les paramètres de l'application (sauf les tokens et le mode d'execution qui sont dans le fichier .env) seront enregistrés dans un fichier json.

Ce fichier json settings.json, aura une structure arborescente.

### Description du fichier json settings.json

Le fichier settings.json doit permettre de gérer les paramètres de l'application. 

A sa racine, nous avons deux noeuds :

- un noeud `référence` qui contient des informations sur le fichier lui même qui contient :
  - une version
  - une date de création,
  - une date de révision
  - une description
- un noeud `data` qui contient des groupes (Branches)
  - Chaque groupe est défini par un nom

Dans un groupe on peut retrouver :

- de 0 à n Groupes

- de 0 à n parametres 

Un paramètre est défini par :

- Un identifiant, obligatoire. C'est le nom de la variable.
- un nom,  obligatoire. C'est le libellé de l'arbre.
- une descrition, obligatoire
- un type, obligatoire
- une valeur_par_defaut, obligatoire
- une surcharge, optionnel
- min, optionnel
- max, optionel
- critique, optionnel
- documentation, optionnel. 

### Etape 3.1 - Réalisée - [1ff8aa4c]

L'étape 3.1 consiste à commencer la composition de la vue `SettingsView.vue`, et tout particulièrement  le composant l'arbre des paramètres `SettingsTree.vue`.

La vue sera composée : 

- sur sa partie haute dun composant `SettingsToolbar.vue`. Le contenu de ce composant sera décrit dans une étape ultérieure.

en dessous 1 colonnne qui contient :

- un `composant` `SettingsTree.vue` qui affichera sous forme d'arbre les groupes et les paramètres du noeud `data` . Les branches de l'arbres seront fermées à la création. Une icône sous forme de dossier fermer/ouvert sera présent devant chaque groupe, (Prepend Icons). Les actions icons seront masqués.

### Etape 3.2 - Réalisée - [c7353538]

Actuellement nous avons pour les environnements d'execution Prod, Sandbox et Test.  Il faut les remplacer par OPE, EVAL et TEST. 

### Etape 3.3 - Réalisée - [1098e7e8]

 Au lancement de l'application, il faut vérifier si le fichier .env existe sous C:\Users\$USER\AppData\Roaming\com.visugps.app\VisuGPS. 

Si il n'existe pas il faut le créer en recopiant le fichier envDefault situé dans src-tauri.

Ensuite on lit la variable, pour connaitre le mode d'execution. 

### Etape 3.4 - Réalisé - [a726d07f]

Par défaut, au premier démarage, on est par défaut en mode OPE. 

Dans la toolbar de la vue Settings, il faut ajouter sur la droite, une icône pour accéder au composant ExeMode. 

Ce composant doit permettre : 

- de créer un nouveau mode d'exécution EVAL ou TEST. 

- de supprimer les modes d'éxécution EVAL et TEST.

- de selectionner un mode d'éxécution, déjà créé.

Le mode TEST ne peut être créé/supprimé uniquement si nous sommes en dev (voir : process.env.TAURI_ENV).

Le mode OPE n'est pas supprimable.

Pour créer un nouveau mode il faut saisir : 

- son nom : à travers un v-text

- un description à travers un v-textarea

Une liste des modes d'éxéction sera créée afin que l'on puisse changer de mode. (Ne pas oublier d'inclure le mode OPE). 

Une liste des modes d'execution dejà créés sera réalisée afin que l'on puisse séléctionner un ou plusieurs mode pour les supprimer.(On ne pourra pas supprimer le mode d'éxécution actif.)

 Ce composant sera dans une fenêtre nodale.

A la suite de la création d'un nouveau contexte, une dialog box demandera à l'utilisateur si il veut redémarer l'application pour prendre en compte le nouveau mode créé.

A la suite de la séléction d'un nouveau contexte, une dialog box demandera à l'utilisateur si il veut redémarer l'application pour prendre en compte le nouveau mode séléectionné.

### Etape 3.5 - Réalisé - [221f1590]

Dans cette étape nous allons travailler sur les fichiers settings.json.

Ce fichier est crée lors de la création d'un contexte d'éxécution. A partir du fichier 

`src-tauri/SettingsDefaut.json`

Lors de sa création il faut mettre à jour les attributs date-creation, context (Le nom du contexte d'éxécution) et description, qui est données lors de la creation du contexte d'éxécution via le composant ExeMode.vue.

### Etape 3.6 - Réalisée - [d2f629f7]

Actuellement le token mapBox est lu dans le fichiers .env.

Il faut qu'il soit gérer dans le fichiers Json sous  le groupe Système/Token(s). 

Faire les modifications nécessaire pour le lire dans ce fichier. 

Lors de la création d'un nouveau mode d'éxécution, le nouveau fichier settings.json devra récupérer le token mpabos existant pour le mettre à jour, dans le fichier crée

l'attribut crtique sera à true 

. En effet pour des raison de sécurité le ficheir settingsDefault.json ne le connait pas. 

### Etape 3.7 - Réalisée - [541217d5]

Actuellement nous avons dans le compossant AppMainBar.vue un switch et ses icones  pour sélectionner choisir le mode clair/sombre. Il lfaudrait déplacer  ce switch et ses icônes dans le settingsToolbar.vue. 

### Etape 3.8 - Réalisée - [3b763a46]

Dans le composant SettingsTree.vue, il faut afficher dans une infobulle le chemin complet des groupes.  

### Etape 3.9 - Réalisée - []

Réalisaton d'un composant pour modifier un paramètre string (MapBox par exemple)

#### Etape 3.9.1 - Réalisé - [f364f1cd]

Cette étape consite à créer un nouveau composant pour éditer les paramètres de type string. Seront  pris en compte les données suivantes d'un paramètre de type string :

- identifiant, libelle, description, defaut, surcharge, critique, min, max. 

#### Etape 3.9.2 - Réalisée - [e720245c]

Cette étape consiste à ajouter de la documentation sur les paramètres. Les documents seront au format .md et sont stockés sous /public/docs.

Seule de frontend devrait être mis à contribution pour afficher la documentation.

Si un paramètre a un attibut doc,  cet attribut contiendra le nom de fichier.

Si le paramètre existe une icone  `mdi-book-open-outline` sera affichée en haut à droite du composant. l'icone sera en bleu (info) . La documentation s'affichera dans un composant flotant.

#### Etape 3.9.3 - Réalisée - [3eb39713]

Cette étape consite à créer un nouveau composant pour éditer les paramètres de type string, en s'inspirant du composant EditStringDialog.

Seront pris en compte si elles existent et non null les données suivantes d'un paramètre de type string :

- identifiant, libelle, description, defaut, surcharge, critique, min, max, step, doc et unité.

Les valeurs min et max permettront de contrôler la valeur saisie.

#### Etape 3.9.4 - Réalisée - [2afcc5c5]

Cette étape consite à créer un nouveau composant pour éditer les paramètres de type Booleen, en s'inspirant du composant EditStringDialog et EditIntDialog

Seront pris en compte si elles existent et non null les données suivantes d'un paramètre de type booleen :

- identifiant, libelle, description, defaut, surcharge, critique et doc.

#### Etape 3.9.5 - Réalisée - [fb301eb6]

Cette étape consite à créer un nouveau composant pour éditer les paramètres de type couleur, en s'inspirant des composants EditStringDialog,  EditIntDialog et EditBoolDialog

Seront pris en compte si elles existent et non null les données suivantes d'un paramètre de type couleur:

- identifiant, libelle, description, defaut, surcharge, critique, materialDesing, et doc..

Si materialDesing  vaut true, alors la couleur sélectionnée devra être convertie en une chaine de caractère connue de la palette de couleur de Vuetify. Sinon elle restera en #RVBA

#### Etape 3.9.6 - Réalisée - [1b936b6]

Cette étape consite à créer un nouveau composant pour éditer les paramètres de type reel, en s'inspirant principalement du composant EditStringDialog.

Seront pris en compte si elles existent et non null les données suivantes d'un paramètre de type reel :

- identifiant, libelle, description, defaut, surcharge, critique, min, max, decimales, doc et unité.

Les valeurs min et max permettront de contrôler la valeur saisie. 

'décimales' indique le nombre maximum de décimales pour la valeur. Lors de la saisie si le nombre max de décimales est atteind, il faut rejeter les nouvelles décimales. 

La scrollbar ne me semble pas nécessaire pour une valeur réelle. 

#### Etape 3.9.7 - Réalisée - [7444c3a7]

Cette étape consite à créer un nouveau composant pour éditer les paramètres de type coord, en s'inspirant des  composants  Edit*Dialog.

les coordonnées seront données sous la forme "longitude, latittude". exemple "`[-77.0364,38.8951]`"

Seront pris en compte si elles existent et non null les données suivantes d'un paramètre de type reel :

- identifiant, libelle, description, defaut, surcharge, critique,   decimales, doc.

'décimales' indique le nombre maximum de décimales pour les valeurs. 

Une carte mapbox sera affichée avec un objet graphique qui identifie le nouveau point. Un pin sera positioné sur la valeur par défaut. l'utilisateur pour zoomer/dézoomer sur la carte (avec la molette) et la déplacer avec la souris  (clic droit et déplacement de la souris)

---

## Etape 4

L'étape 4 va consister à gerer les fichiers gpx et leurs données associées.

### Selection d'un gpx - Réalisée - [87fe5f57]

Dans le container AppMainBar.vue une nouvelle icône (`mdi-file-import-outline`) devra être insérée à la gauche du `mdi-cog`. 

- Attention à ne pas réinsérer le composant `@src/components/LightDarkSwitch.vue`.

- Attention à l'alignment des icônes.

Qaund on clique sur cette icône, un nouveau composant devra s'ouvrir ouvir et lister les fichiers *.gpx présent dans le dossier defini par la variable Importation/Dossier/GPXFile définie dans le fichier setting.json. Pour ce paramètre la valeur par defaut sera le dossier de téléchargement de l'utilisateur. Dans ce cas le nom sera `DEFAULT_DOWNLOADS` .Pour que cette solution fonctionne quelque soit le système d'exploitation, je te propose dirs::download_dir()

- Sinon,  penser que la surcharge peut être une chaine de caractères de la forme E:/Loisir/MesDossiers

La sélection d'un fichier déclenchera son traitement, uniquement après avoir cliquer sur un bouton Impoter.

Un bouton Annuler doit permettre de sortir du container sans déclencher d'action.

Dans cette partie il faut générer tout le code frontend et backend pour pouvoir sélectionner un fichier dans le dossier spécifé dans le fichier settings.json sans enchainer sur son traitement que l'on verra plus tard.

Attention à la syntaxe pour atteindre les variables du fichier settings.json (`/`vs `.`). Il ne faut pas la modifier au risque d'avoir un effet de bord avec le composant EditCoordDialog.

---

### Fichier circuits.json - Réalisé - [ 7782c290]

La description du fichier circuits.json est donnée dans les fichiers 

- `@documentation/fichiers_json/ circuits_analysis.md` 

- `@documentation/fichiers_json/ circuits.schema.yaml`

Au lancement de l'application, si dans le dossier du contexte d'éxécution le fichiers circuits.json n'existe pas il faut le créer. Il sera par la suite mis à jour en fonction de l'import ou de la suppression de traces GPX. 

### Reconnaissance de l'éditeur gpx - Réalisé - [ec145a9b]

lors de l'imporatition d'un fichier gpx, il faut reconnaitre la source du GPX 

Il faut déterminer si le fichier provient de : 

1. Strava

2. openrunner

3. Garmin connect

4. ridewithgps

5. ... 

Quand l'origine est trouvée, il faut mettre à jour si nécessaire la rubrique editeurs du fichiers circuits.json pour l'id la syntaxe suivante sera utilisée `ed-0001`, `ed-0002`, `ed-0003`, etc.

---

### Mise à jour (partielle) de la structure circuit dans le fichier circuits.json

Pour chaque fichier corectement importé, il faut mettre à jour le fichier circuits.json avec les attributs suivants : 

- circuitId, sous la forme d'un digit de 6 chiffres de 000001 à 999999. Sa valeur sera calculée à partir de indexCircuits + 1

- nom: issue du fichier gpx 

- editeurId : trouvé dans la sequence précédente.

- url : Url de la trace vers l'editeur de la trace. 

- distanceKm : Calcul de la distance de la trace. ( en nodeJS on peut utiliser la librairie turf. en Rust tu peux utliser geo ). Un fichier lineString.json va être créé dans le chapitre suivant. Tu pouuras peut-être t'en servir pour tes calcul..  

- depart : les coordonnées du premier point 

- deniveleM : le denivelé positif du circuit, calculé à partir de la 3 données du fichier gpx.

- sommet : le point le plus haut de la trace GPX avec sa distance cummulée depuis le point de départ

A chaque importation correcte(sans erreur) de circuit l'attribut indexCircuits devra être incrémenté.

Dans le dossier de l'environnement d'éxécution, pour chaque circuit importé, nous allons créé dans le dossier data (qui est à créé si il n'existe pas) un sous dossier lié au circuit qui est en cours d'imporation. Le nom de ce dossier sera le même que circuitId

### Création du fichier lineString.json

A partir de la trace GPX, je voudrais que tu me génères un fichier lineString.json, qui sera sauvegardé dans le dossier data/circuitId

---

## 🤝 Contribution

Pour contribuer, veuillez vous référer au guide de contribution dans `CONTRIBUTING.md`.

---

## 📄 Licence

Ce projet est sous licence **Apache 2.0**. Pour plus de détails, consultez le fichier **`LICENSE`** à la racine de ce projet.

---

## Résumé des Modifications Apportées le 09/09/2025

### **1. Améliorations de l'Interface Utilisateur (UI)**

* **Cadre Visuel Conditionnel (Étape 0.7)**

  * Ajout d'un cadre autour de l'application dans `App.vue` (`v-container`).

  * Ce cadre est visible uniquement dans les environnements `EVAL` (orange) et `TEST` (rouge).

  * Configuration du cadre : 5px d'épaisseur, sans bords arrondis ni ombres, et prenant toute la hauteur de la fenêtre.

  * Suppression du padding par défaut du `v-container` dans `App.vue`.

* **Barre d'Outils Principale (`AppMainBar`) (Étape 1.1)**

  * Création du composant `src/components/AppMainBar.vue`.

  * Intégration de `AppMainBar` dans `src/views/MainView.vue` (initialement envisagé dans `App.vue`, puis ajusté pour répondre au besoin spécifique de l'utilisateur).

  * **Mode Sombre/Clair :**

    * Remplacement du bouton (`v-btn`) par un interrupteur (`v-switch`) pour basculer entre les modes sombre et clair.

    * Utilisation d'icônes (Lune/Soleil) à la place du texte sur le `v-switch`.

    * **Persistance du mode :** Le mode choisi est désormais sauvegardé localement (`window.localStorage`) et restauré au redémarrage de l'application.

    * Ajustement de la couleur de l'icône Soleil pour une meilleure visibilité en mode clair (ambre foncé).

  * **Indicateur d'Environnement (Chip) :**

    * Affichage d'un `v-chip` indiquant la valeur de `APP_ENV` (ex: `Sandbox_Camera`, `Test_CalculLongueur`).

    * Le chip est visible uniquement si l'environnement est `EVAL` (couleur orange) ou `TEST` (couleur rouge).

    * Le chip est animé pour clignoter de manière abrupte.

    * Positionnement du chip à l'extrême gauche de la barre d'outils.

  * **Titre "Accueil" :**

    * Déplacement du titre "Accueil" de `MainView.vue` vers `AppMainBar.vue`.

    * Implémentation d'une structure `v-row`/`v-col` pour un centrage plus précis du titre dans la barre d'outils.

### **2. Gestion des Services et Connexion Mapbox (Étape 1.2)**

* **Lecture du Token Mapbox :**

  * Le `MAPBOX_TOKEN` est désormais lu depuis le fichier `.env` (situé dans le répertoire de données de l'application) par le backend Rust (`src-tauri/src/lib.rs`).

  * Le token est exposé au frontend via le `AppState` et le composable `useEnvironment`.

* **Vérification de la Connectivité Internet :**

  * Implémentation d'une commande Rust (`check_internet_connectivity`) pour une vérification fiable de la connexion Internet (en pingant un service externe).

  * Utilisation de cette vérification dans le composable `useServiceStatus`.

  * **Polling :** L'état des services est vérifié toutes les 10 secondes.

* **Vérification du Serveur Mapbox et Validité du Token :**

  * Implémentation d'une commande Rust (`check_mapbox_status`) qui effectue une requête à l'API Mapbox.

  * La fonction différencie désormais clairement un serveur Mapbox inaccessible (blocage par pare-feu, problème réseau) d'un token invalide, même si le serveur répond avec un statut 200 OK mais un message d'erreur.

* **Indicateur d'État des Services (Icône) :**

  * Ajout d'une icône dynamique à gauche de la barre d'outils (`AppMainBar.vue`) dont l'icône et la couleur changent en fonction de l'état des services (connecté, déconnecté, Mapbox inaccessible, token invalide, vérification).

  * L'icône `mdi-earth-off` est utilisée pour l'état "Mapbox inaccessible".

  * Taille de l'icône forcée à 36px.

  * Ajout d'un padding gauche de 16px à l'icône.

* **Notifications (`ShowSnackBar`) :**

  * Création des composables `useSnackbar.js` et du composant `SnackbarContainer.vue`.

  * Intégration du `SnackbarContainer` dans `App.vue` pour afficher les messages de statut.

  * Les messages sont affichés uniquement lors d'un *changement d'état* réel (pas de répétition si l'état est stable).

  * Le message "Serveur Mapbox inaccessible" s'affiche en bleu.

  * La durée d'affichage des messages est de 5000 ms.

  * Le bouton "Fermer" a été supprimé du snackbar.

### **3. Optimisations et Corrections Diverses**

* **Correction d'Avertissement Rust :** Suppression de l'avertissement `unused import: serde_json::Value` dans `src-tauri/src/lib.rs`.

* **Icône de l'Exécutable :** Explication du processus de génération d'icônes multiplateformes (`tauri icon`) et des problèmes de cache en mode développement.

* **Largeur Minimale de la Fenêtre :** Définition d'une largeur minimale de 1024 pixels pour la fenêtre de l'application dans `tauri.conf.json`.

---

## Résumé des travaux du mercredi 10 septembre 2025

### Implémentation de l'Étape 3.1

* **Création des composants :** Mise en place de `SettingsToolbar.vue` et `SettingsTree.vue`, intégrés dans `SettingsView.vue`.
* **Lecture des paramètres :** Implémentation d'une commande Tauri (`read_settings`) pour lire le fichier `settings.json` depuis le backend.
* **Affichage de l'arbre :** Affichage récursif des groupes et paramètres dans `SettingsTree.vue` via `SettingsNode.vue`.

### Gestion des fichiers de configuration

* **Renommage :** `src-tauri/settings.json` a été renommé en `src-tauri/settingsDefault.json`.
* **Intégration dans l'exécutable :** `settingsDefault.json` est désormais embarqué directement dans le binaire Rust (`visugps.exe`) au lieu d'être une ressource séparée.
* **Logique de copie au démarrage :** La fonction `setup_environment` (Rust) copie le contenu embarqué de `settingsDefault.json` vers `settings.json` dans le dossier de l'utilisateur (`~/AppData/Roaming/com.visugps.app/VisuGPS/$APP_ENV`) uniquement si `settings.json` n'existe pas.
* **Lecture contextuelle :** La commande `read_settings` lit maintenant le fichier `settings.json` spécifique au contexte d'exécution de l'application.

### Améliorations de l'affichage et de la logique des paramètres

* **Adaptation à la structure JSON :** Les composants `SettingsTree.vue` et `SettingsNode.vue` ont été adaptés pour utiliser les champs `identifiant` (pour les clés uniques) et `libelle` (pour l'affichage) des paramètres et groupes.
* **Logique de surcharge centralisée :** Création d'un composable `useSettings.js` avec une fonction `getSettingValue(path)` qui gère la priorité `surcharge` > `defaut` pour tous les paramètres.
* **Utilisation du paramètre de timer :** Le composant `AppMainBar.vue` utilise désormais `getSettingValue` pour récupérer l'intervalle de polling réseau depuis les paramètres.
* **Indicateurs visuels :**
  * Le libellé des paramètres marqués `critique: true` est affiché en couleur `warning` (orange adaptatif au thème).
  * L'icône des paramètres dont la valeur est surchargée (`surcharge` non nul) est affichée en couleur `info` (bleu).
* **Améliorations de l'interface utilisateur :**
  * Ajustement de la largeur du composant `SettingsTree` dans `SettingsView.vue`.
  * Icône de dossier ouvert/fermé dynamique pour les groupes dans l'arbre.
  * Fond transparent pour le `v-card` et le `v-list` dans `SettingsTree.vue` pour une meilleure intégration visuelle.

### Navigation

* **Bouton "Paramètres" :** Ajout d'un bouton dans `AppMainBar.vue` pour naviguer vers la vue des paramètres.
* **Bouton "Accueil" :** Ajout d'un bouton avec une icône de maison dans `SettingsToolbar.vue` pour revenir à la vue principale.

### Corrections de bugs et raffinements

* **Erreur de compilation Rust :** Résolution de l'avertissement `unused imports: AppHandle and Wry` dans `src/lib.rs`.
* **Problème d'importation Vite :** Correction de l'erreur `Failed to resolve import "@tauri-apps/api/tauri"` en ajustant l'alias `@` dans `vite.config.js` et en corrigeant le chemin d'importation de l'API Tauri (`@tauri-apps/api/core`).
* **Problème d'affichage de l'icône de surcharge :** Débogage et correction de l'icône bleue pour les paramètres surchargés en utilisant un `v-icon` explicite dans le slot `prepend`.

---

## 🤝 Contribution

Pour contribuer, veuillez vous référer au guide de contribution dans `CONTRIBUTING.md`.

---

## 📄 Licence

Ce projet est sous licence **Apache 2.0**. Pour plus de détails, consultez le fichier **`LICENSE`** à la racine de ce projet.
