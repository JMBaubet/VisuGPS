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

Dans cette étape nous nous assurons que vuetify est correctement installé, et on va configurer un cadre en fonction du contexte d'éxécutiuon.

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

### Mise à jour (partielle) de la structure circuit dans le fichier circuits.json - Réalisé - [50fe9be0]

Pour chaque fichier corectement importé, il faut mettre à jour le fichier circuits.json avec les attributs suivants : 

- circuitId, sous la forme d'un digit de 4 chiffres de circ-0001 à circ-9999. Sa valeur sera calculée à partir de indexCircuits + 1

- nom: issue du fichier gpx 

- editeurId : trouvé dans la sequence précédente.  

- depart : les coordonnées du premier point 

- isoDateTime : l'heure de l'imporation.

A chaque importation correcte(sans erreur) de circuit l'attribut indexCircuits devra être incrémenté.

Dans le dossier de l'environnement d'éxécution, pour chaque circuit importé, nous allons créé dans le dossier data (qui est à créé si il n'existe pas) un sous dossier lié au circuit qui est en cours d'imporation. Le nom de ce dossier sera le même que circuitId

### Création du fichier lineString.json - Réalisé - [2230f202]

A partir de la trace GPX, je voudrais que tu me génères le fichier lineString.json, qui sera sauvegardé dans le dossier data/circuitId, sous le nom de lineTring.json.

Pour cela tu lis la trace gpx chargée, puis à partir des données géographique tu crées le fichier lineString avec les paramètres [longitude, latitude, altitude]

### Suite de la mise à jour du fichier circuits.json - Réalisé - [5866587f]

A partir du fichier lineString.json, je voudrais que tu calcules :

- La longueur de la trace. je te propose d'utiliser geojson et geo

- Le point le plus haut de la trace et la distance à laquelle il se situe par rapport au départ

- Le denivelé positif. On y ajoute un paramètre de lissage dans le fichier settingDefault.json

Ensuite tu mets à jour le fichier circuits.json.

---

### Poursuite de la mise à jour de circuits.json - Réalisé - [de55b315]

Pour poursuivre la mise à jour des paramètres liés a un circuit lors de son importation, il faut encore renseigner les attributs suivants :

- url 

- traceurId

Pour url je souhaite pouvoir retrouver le fichier origine sur le site de l'editeur (strava, ridewithgps, openrunner, gamin-connect, etc. ) Tu dois pouvoir trouver cette information dans le fichier gpx, mais ce n'est pas toujours le cas. Si tu as des dificultés 

pour y arriver mets un message le plus explicite possible dans la showSnackBar.

- Pour un fichier de type `Strava`  la balise link se trouve sous  gpx/trk/

- Pour un fichier de type `TestWithRideGPS` la balise link se trouve sous gpx/metadata/

- Pour un fichier `OpenRunner` l'URL est : https://www.openrunner.com/route-details/NUMBER, 

- Pour un fichier `Garmin Connect`l'URL est : https://connectgarmin.com/modern/cource/NUMBER
  
  - NUMBER se trouve dans gpx/trk/name. Dans cette balise on trouve Nom_de_la_trace-NUMBER (le caractère`-` est le séparateur)

Pour traceurId il faut que tu me le demande au moment de l'import. Tu peux à partir du fichier circuits.json me proposer une liste défilante des traceurs déjà connus. Je dois pouvoir egalement saisir le nom d'un nouveau traceur. ensuite valider. On ne peux pas rester avec ce champ vide. ou pire j'attourerai via l'interface un traceur inconnu... 

### Ville de départ - Réalisé - [804c9c58]

Pour finaliser la mise à jour des paramètres liés à un circuit lors de son importation, il faut encore renseigner l'attribut villeDepartId

A partir des coordonnées de départ déjà connu, tu dois déterminer le nom de la commune. 

Pour cela tu peux par défaut essayer avec l'API de géocodage inverse de l'IGN. 

- Si nous sommes en France, tu obtiendra le nom de la commune,

- Sinon je pense que le geoportail te renverra une erreur. Dans ce cas  tu dois pouvoir faire une requette à mapBox.

Une fois que tu as le nom de la commune, tu vérifies si elle n'est pas dejà présente dans le fichier circuits.json. 

- Si c'est le cas tu récupères son UUID . 

- Sinon tu l'ajoutes avec un nouvelle UUID 

Puis tu mets à jour VilleDepartId du cicuit.

Note : Si il est nécessaire d'avoir un token pour le geoportail de l'IGN peux tu me le préciser ? Si oui il te fraudra créer un nouveau paramètre dans settingDefault.json sous `Système/Tokens`

### Génération de la vignette - Réalisé - [64ef49b2]

lors de l'imporation d'un circuit gpx, j'ai besoin d'avoir la génération d'un vignette MapBox pour une visualisation de la trace. Pour cela tu as à ta disposition le fichier lineString.json dans le dossier du circuit qui se trouve dans le répertoire d'éxécution. 

Pour le generation de cette vignette, nous allons prendre en compte plusieurs paramétres qui sont ou seront féfinis dans le fichier settings.json. Ces paramètres sont les suivants : 

- Importation/Mapbox/styleVignette qui correspond au style de la carte. Ce paramètre est déjà présent dans le fichiers settingsDefault.json,

- Importation/MapBox/colorGPXVignette qui correspond à la couleur de la trace. Ce paramètre est déjà présent dans le fichiers settingsDefault.json,

- Importation/MapBox/largeurTrace qui correspond à la largeur de la vignette, de type entier.

- Importation/MapBox/format qui correspond au format de la vignette (1/1, 4/3, 16:9, etc.) de type string,

- Importation/MapBox/presenceDistance, un booleen qui indique si on veux mettre une information de distance sur la trace tous les n km,

- Importation/MapBox/Distance, un entier qui défini la distance entre deux informations de distance,

- Imporation/MapBox/Vignettes, un booleen, qui nous indique si nous voulons voir des pins sur les points de départ et d'arrivée.

- Imporation/MapBox/couleurDépart, une couleur MaterialDesing pour le pin de départ

- Imporation/MapBox/couleurArrivée, une couleur MaterialDesing pour le pin d'arrivée

- Imporation/MapBox/couleurDépartArrivée, une couleur MaterialDesing pour le pin de départ/Arrivée si ils sont proches.

- Imporation/MapBox/distanceMax, un entier qui correspond à une distance en m qui sépare le départ et l'arrivée, à vol d'oiseau pour déterminer si on différencie les pins depart et arrivée, ou si on met un pin commun.

Le fichier généré se nomera vignette.png et sera enregistré dans le dossier dédié au circuit importé.

### Restructuration du fichier settingDefault.json - Réalisé - [0efa71ab]

Je voudrais que l'on modifie l'arborescence de Importation/Mabox et de ces paramètres.  Respecter l'ordre des paramètres comme indiqué dans la suite du paragraphe.

Bien sûr il faudra également mettre à jour le code avec cette nouvelle organisation. 

1. renomer Imporation/Mapbox en Importation/Vignette

2. Créer un groupe Importation/Vignette/Dimentions
   
   1. Déplacer dans ce groupe : 
      
      1. Largeur de la vignette
      
      2. Largeur de la vignette

3. Créer un groupe Importation/Vignette/Trace
   
   1. Déplacer dans ce groupe : 
      
      1. Couleur de la trace sur la vignette
      
      2. Largeur de la trace, et mettre sa valeur pas défaut à 3

4. Créer un groupe Importation/Vignette/MarqueurDistance
   
   1. Déplacer dans ce groupe : 
      
      1. Afficher la distance, et mettre la valeur par défaut à true
      
      2. Intervalle distance/direction, et le renommer Intervalle distance 
      
      3. Couleur des marqueurs de distance

5. Créer un groupe Importation/Vignette/DepartArrivee
   
   1. Dépalcer dans ce groupe :
      
      1. Afficher les marqueurs
      
      2. Couleur du marqueur de départ
      
      3. Couleur du marqueur d'arrivée
      
      4. Distance max départ/arrivée (m) et mettre 250 par défaut.
      
      5. Couleur du marqueur départ/arrivée (proches)

### Génération du fichier tracking.json - Réalisé - [7bf37061]

Le fichier tracking.json doit être engistré dans le dossier dédié au circuit importé.

Il est construit à partir du fichier lineString.json.

Dans ce fichier on doit avoir une liste de paramètres. 

On aura autant de paramètres que nécessaire pour traiter toute la longueur de la trace gps. 

Chacun de ces paramètres est composé des attributs suivants. 

- `increment` un entier qui correspond au nième élément de la la liste en commençant par 0.

- `pointDeControl`un booleen qui sera utilisé ulétérieument.

- `nbrSegment`un entier qui sera utilisé ultérieument.

- `coordonnee` qui est composé de deux valeurs [longitude, latittude] avec une précision de 5 décimales.

- `altitude` un reel qui donne l'altitude de `coordonnee` avec une précision de 1 décimale.

- `commune` un string que donne le nom de la commune du point de `coordonnee`

- `cap` un entier qui sera utilisé ultérieument pour la caméra

- `zoom` un reel qui sera utilisé ultérieument pour la caméra

- `pitch` un reel qui sera utilisé ultérieument pour la caméra

- `coordonneeCamera` qui est composé de deux valeurs [longitude, latittude] et qui sera utilisé ultérieument pour la caméra, avc une précision de 5 décimales

- `altitudeCamera`en entier qui sera utilisé ultérieument pour la caméra

Le fichier sera généré avec tous les attributs décrits, mais seuls les suivants seront renseignés à sa céation :

- `incement`, `coordonnee`, `cap`, et `altitude` seront calculés,

tandis que :

- `zoom` sera initié via le paramètre Importation/Caméra/Zoom avec 16 comme valeur pas défaut

- `pitch` sera initié par le paramètre Importation/Caméra/Pitch avec 60° comme valeur par défaut.

Les attributs suivants seront initialisés avec les valeurs suivantes :

- `pointDeControl` : false

- `nbrSegment`: 0

- `commune` : ""

- `coordonneeCamera` : []

- `altitudeCamera`: 0

#### Calcul de increment

increment sera incrémenté de 1 a chaque paramètre calculé. Il commence à 0

#### Calcul de coordonnee

L'attribut `coordonnee`est déterminé par un entier (X) paramétré dans Importation/Tracking/LongueurSegment qui vaut 100m par défaut.

En partant du premier point on doit calculer les coordonnées du point suivant situé sur la lineString à un distance de X m, et ce jusqu'à la fin de la longueur de la lineString.

On va traiter des fichiers lineString de plus de 100 km.

Ne pas oublier d'y ajouter le dernier segment aura une longueur comprise entre ]0, X].

#### Calcul de altitude

`altitude` sera calculée en faisant une règle de 3 avec les deux points les plus proche en amont et en aval,  de la lineString, du nouveau point calculé. `coordonnee`.

#### Calcul de cap

Le calcul de cap s'appuie uniquement sur les coodonnées calculées. On ne prends pas en compte la lineString pour ce calcul. 

Le problème consiste à estimer une **direction moyenne** (cap) à partir d’une succession de Y segments consécutifs. Comme le cap est un **angle circulaire** défini modulo 360°, une moyenne arithmétique simple peut être trompeuse (par exemple, la moyenne de 350° et 10° ne doit pas donner 180° mais bien 0°).

Pour éviter ce piège, on utilise une **moyenne vectorielle** des orientations :

1. **Sélection des points**
   
   - On part d’un point `P[i]` de la ligne.
   
   - On considère les `y` segments suivants : `(P[i]→P[i+1]), (P[i+1]→P[i+2]), …, (P[i+y-1]→P[i+y])`.

2. **Calcul des caps individuels**
   
   - Pour chaque segment `P[k]→P[k+1]`, on calcule le **bearing géodésique** (angle par rapport au nord).
   
   - Ce bearing est exprimé en degrés dans l’intervalle `[0°, 360°)`.

3. **Passage en coordonnées vectorielles**
   
   - Chaque cap `θ` est converti en radians.
   
   - On projette l’angle sur le cercle trigonométrique :
     
     - `x = cos(θ)`
     
     - `y = sin(θ)`
   
   - On obtient ainsi un vecteur unitaire qui pointe dans la direction du segment.

4. **Somme et moyenne**
   
   - On additionne tous les vecteurs :
     
     - `X = Σ cos(θk)`
     
     - `Y = Σ sin(θk)`
   
   - Ces sommes représentent le vecteur moyen (non encore normalisé).

5. **Reconstruction de l’angle moyen**
   
   - On calcule l’angle du vecteur moyen avec :
     
     - `θmoy = atan2(Y, X)`
   
   - Cet angle est ensuite converti en degrés et normalisé dans `[0°, 360°)`.

6. **Résultat**
   
   - `θmoy` représente le **cap moyen** de l’ensemble des `y` segments considérés.
   
   - Cette approche prend correctement en compte le caractère circulaire des angles et évite les erreurs dues au franchissement de la discontinuité à 0°/360°.
   
   Y est un entier paramétré dans Importation/Tracking/LissageCap qui vaut 15 par défaut. 
   
   Pour les Y derniers on calculera le cap avec les points restants 14, 13, 12, ...

## Affichage des circuits - Réalisé - [d83003ad]

Les circuits sont à afficher dans la page d'accueil.

La page d'accueil devra être mise à jour à chaque ajout ou suppression de circuit.

L'ensemble des circuits sera affiché dans une v-page, qui présentera un extrait d'une v-list. 

Les circuits à afficher sont décrits dans le fichier circuits.json situé dans le dossier du conexte d'éxécusion.

### Affichage d'un circuit

Pour afficher les données d'un circuit déjà importé, un nouveau composant doit être créé. 

Ce composant doit afficher les données suivantes sur deux lignes maximum :

- `nom`

- `distance`

- `dénivelé`

- `Ville de départ` 

A partir de ce composant, on doit avoir des icônes pour effectuer les actions suivantes :

- `debuger le circuit` uniquement en mode Dev. Cette fonction n'est pas accessible en mode Prod.

- `Editer le tracking`

- `Voir la 3D`

- `Supprimer le circuit`

Les icones seront allignées à droite dans l'odre donné ci dessus. La description de ces actions sera donnée dans des futurs chapitres.

## Debogage de tracking.json - Réalisé - [0e001a5a]

Pour cette étape nous allons créer une nouvelle vue qui sera appelée via le bouton debug d'un circuit présent dans la v-list de la mainView.

Cette vue doit affichée une carte Mapbox, qui affichera :

- la trace à partir du fichier lineString.json situé dans le dossier lié au circuit sélectionné. La lineTring sera affichée en bleu avec une opacité de 50%

- A partir du fichier tracking.json, je souhaite que l'on puisse afficher un à un l'ensemble des points du fichier en rouge sous forme d'un cercle de 3 pixel de diamètre, , ainsi que les n points en jaune qui ont été pris en compte pour le calcul du cap. n étant le paramètre Importation/Tracking/LissageCap du fichier settings.json.

- Pour chacun des points on affiche un vecteur de 100 m qui part du point en direction du cap. Le vecteur sera de couleur Jaune et aura une épaisseur de 3.

Pour la lineString une boite à cocher permetra de l'afficher ou de la masquer. Par defaut elle sera affichée.

Pour les caps une boite à cocher permetra de les afficher ou de les masquer. Par defaut ils seront affichés .

Les touches flêche gauche et flêche droite du clavier permettront de passer d'un point à l'autre. Ctrl  + flêche Gauche ou Droite permettra de fait un saut de 10 points en arrière ou avant.

Une attention particulière pour l'affichage des n points sera apportée pour les n derniers points du fichier tracking.json.

## Suppression d'un circuit. - Réalisé - [bfa06ec9]

Depuis la vue princiaple, on peut voir les circuits déjà importés. Pour chaque circuit on a une icône `mdi-delete` qui sur un click souris appelle la fonction deleteCircuit.

Cette fonction doit supprimer :

- le cicuit du fichier circuits.json

- le répertoire relatif à ce circuit.

## Edition d'un circuit. - Réalisé - [a4639117]

Cette étape est l'une des plus compliquée. Nous allons la construire en plusieurs phases.

### Initialisation de l'édition - Réalisé - [9cb5c34f]

Cette première phase va consister à la mise en place de la vue édition

Le vue édition sera affichée lorsque l'ulisateur cliquera sur l'icône 'mdi-pencil' du composant CircuitListItem.vue/

La vue édition sera constituée d'une div mapbox qui prendra tout l'espace disponique de la vue EditView.vue.

La carte sera initialisée avec les paramètre suivant : 

- le style de la carte est défini dans le fichier settings.json sous Edition/mapbox/styleVisualisation

- le centre de la carte sera issu du paramètre coordonnee du premier point du fichier tracking.json

- le zoom de la camera sera issu du paramètre zoom du premier point de la carte

- le  pitchde la camera sera issu du paramètre pitch du premier point de la carte

- la direction de la camera sera issu du paramètre cap du premier point de la carte

A partir de là on devrait pouvoir renseigner dans le fichier tracking.json les coordonnees de la caméra et son altitude.

On ajoutera une couche pour afficher le lineString. La visualisation de la lineString aura les caractéristiques suivantes :

- Couleur de la lineString : paramètre à créer dans Edition/mapbox/Trace/couleur

- Epaisseur de la lineString : paramètre à créer dans Edition/mapbox/Trace/Epaisseur

Sur la vue EditView.vue prévoir un overlay situé en haut à gauche pour revenir sur la vue pricipale.

### Mise en place des interractions clavier - Réalisé - [0356d6d3]

Dans le vue EditView.vue, nous avons désactivé sur la carte MapBox les commandes de la souris. Nous allons les remplacer par des commandes claviers paramétrables dans le fichier settingDefault.json sous Edition/Mapbox/CommandesCamera

Les commandes caméra à réaliser sont : 

- le zoom  : touche a et z par défaut Avec 'a' on zoom et avec 'z' on dézoom  

- le pitch : touche flêche haute et flêche basse

- le bearing : touche flêche gauche  et flêche droite

Pour le zoom l'incrément par défaut sera de 0.1. Cet incrément est paramétrable

Pour le pitch et le bearing l'incrément par défaut sera de 0 Cet incrément sera paramétrable. Un paramètre pour le pitch et un paramètre pour le bearing 

Le modificateur Shift associé au touches définies ci dessus modifira l'incrément. 

- Pour le zoom ce sera 10 (paramétrable)

- Pour le pitch ce sera 5 (paramétrable)

- Pour le bearing ce sera 5 (paramétrable)

### Affichage des paramétres Caméra - Réalisé - [9a4783b4]

Avec les touches du clavier on peut modifier les paramètres de la camera pour modifier notre point de vue. 

Afin que l'utilisateur sache si il est loin ou proche des valeurs par défaut pour le zoom et le pitch, et qu'il sache dans quelle direction l ecaméra est pointée, nous avons afficher ces paramètres dans un widget qui sera situé en haut à droite de la vue 

ce witget affichera de haut en bas :

- Le bearing

- Le zoom. Si la valeur du zoom est égale à sa valeur par défaut affichage en Vert

- Le pitch. Si la valeur du pitchest égale à sa valeur par défaut affichage en Vert

### Graphe camera - réalisé - [f5bdf092]

Pour avoir une vue synthétique de tous les paramètres de la caméra, nous allons créer sous forme de widget qui sera affiché en bas de la vue un graphe multiple  à 2 dimentions.

Sur l'axe  X des abcisses, nous allons avoir la distance. 30 pixels pour 1 km. Bien sûr nous allons avoir une scrollBar horizontale pour afficher des distances > 90 km environ.

Sur l'axe  Y plusieurs données sont être affichées : 

- Le zoom qui aura en référence en Y sa valeur par défaut sur l'axe des absicces , et nous afficherons au dessus de l'axe des abcisses les zooms supérieurs à la valeur par défaut (16) et en dessous les valeurs inférieures. un pixel pour 0.1 

- Le bearing. La référence en Y du  bearing sera le bearing du point 0. Ensuite pour chaque incrément de distance on affichera :
  
  - Le delta avec le point précédent 3 pixels pour 1° d'ecart
  
  - Le delta depuis le Km 0 (1 pixels pour 1°)

- Le  pitch qui aura en référence en Y sa valeur par défaut sur l'axe des absicces , et nous afficherons au dessus de l'axe des abcisses les pitchs supérieurs à la valeur par défaut (16) et en dessous les valeurs inférieures. un pixel pour 1°

Un barre verticale ou un rectangle d'une opacité de 15% doit permettre de nous indiquer ou nous en sommes sur la progression de la caméra.

### Ajout d'un point de control - Réalisé  - [ecb509d2]

Le but de la vue EditView est de mettre à jour le fichier tracking.json.

un point de control est un point qui sert de référence pour le tracking. 

Son ajout est déclenché par le click sur le bouton *mdi-content-save*

Chaque point enregistré aura son attribut pointDeControl à true. 

Un point de control à true, sera représenté sur le graphe par un trait vertical de 20 pixels et d'une épaisseur de 3 pixels sur la partie supérieure du graphe. Par défaut il sera en rouge. 

Lors de la progression si nous sommes  sur un point de control à true un bouton delete devra être affiché.

L'enregistrement d'un point de control sera déclenché par le bouton save. 

Si on fait un enregistrement sur un point de control dejà à true une confirmation devra être demandée.

Quand on enregistre un point de control :

- on met a jour les paramètres de la camera pour le point en cours de traitement. Les paramètres à mettre à jour sont :
  
  - Le zoom, le heading le pictch, les coordonnées de la camera dans coordonneeCamera et l'altitude de la camera dans altitudeCamera. Pour cela tu dois pouvoir t'appuyer sur :
  
  - - position =  map.getFreeCameraOptions().position, 
    
    - position.toLngLat(),
    
    - position.toAltitude. 

- on cherche dans le fichier tracking.json le point de control précédent. Sur ce point de control précedent on met à jour nbrSegment qui correspont au nombre de segment entre les deux points de control. 

- On cherche dans le fichier tracking.json si il existe un point de control suivant. Si c'est le cas on calcul le nombre de segment qui nous sépare de lui et on met à jour notre paramètres nbrSegment. 

- Pour chaque points intermédaires entre le nouveau point de control et ses voisins  on calcule le bearing, le zoom et le pictch  des points intermédiaires pour avoir une progression linéaire entre les points.

- on mets à jour le graphe avec le nouveau point de control et les nouvelles valeurs calculées

### Suppression d'un point de control - Réalisé - [a4639117]

Sur clique du bouton `mdi-delete` il faut mettre à jour : 

- le fichier tracking.json

- le graphe.

La mise à jour du fichier tracking.jon consiste à :

- mettre à false pointDeControl pour le point considéré

- Si le point de control supprimé est le denier du fichier tracking.json, il faut remettre pour les paramète edited*  les valeurs originales (cap, pitch et zoom)  pour l'ensembles des points qui sont au delà du nouveau dernier point de control, et mettre à 0 nbrSegment pour le nouveau dernier point de controle. Le

- Sinon, rechercher le point de control précédents et le point de control suivant pour mettre à jour lpour les point intermédiaire les paramètres :
  
  - editedCap
  
  - editedPitch
  
  - editedZoom 

- des points intermédiares en faisant une règle lineaire entre les points de control voisins. 'précédent et prochain)

- Mettre à jour nbrSegements pour le point de control précecent. nbrSegement étant le nombre de point entre le point de control précédent et le point de control suivant. 

## Visualisation d'un circuit - Réalisé - [0d732741]

Dans ce chapitre nous allons travailler principalement sur la vue VisualizeView. 

Elle reçoit en paramètre in circuitId qui lui fournit l'accès au répertoire du circuit où sont archivés les fichiers json liés au circuit à traiter. (`lineString.json`, `tracking.json`). Le répertoire du circuit est situé dans le dossier `data` du répertoire liè au contexte d'éxécution.

La vue visualize.view doit :

- Afficher une carte mapbox, avec le relief et un ciel. 
  
  - Le style de la carte est précisé dans le paramètre Visualisation/Mapbox/styleVisualisation, déjà défini dans le fichier `settingsDefault.json`
  
  - l'éxagération du relief est précisé dans le paramètre Edition/Mapbox/Relif/exaggeration, déjà défini dans le fichier `settingsDefault.json`

- Afficher la trace lineString à partir du dossier json
  
  - La couleur de la trace sera définie dans le fichier `settingsDefault.json` sous Visualisation/Mapbox/Traces/couleurTrace. orange par défaut
  
  - l'épaisseur de la trace sera définie dans le fichier `settingsDefault.json` sous Visualisation/Mapbox/Traces/epaisseurTrace, 8 par defaut
  
  - l'opacité de la trace sera définie dans le fichier `settingsDefault.json` sous Visualisation/Mapbox/Traces/opacitéTrace 0.8 par defaut

- Afficher l'avancement avec la lineString issue du fichier `lineString;json`. Pour cela pour les km non encore parcourus la lineString aura une opacité nulle. Un effet de comète suivra le point parcouru en faisant varier l'opacite de la lineString de x à 0 sur une distance paramétrée.
  
  - La couleur de la comete sera définie dans le fichier `settingsDefault.json` sous Visualisation/Mapbox/Traces/couleurComete. yellow par défaut
  
  - a l'épaisseur de la comete sera définie dans le fichier `settingsDefault.json` sous Visualisation/Mapbox/Traces/epaisseurComete. 8 par défaut
  
  - l'opacité de la comete sera définie dans le fichier `settingsDefault.json` sous Visualisation/Mapbox/Traces/opacitéComete. 1 par défaut
  
  - la longueur de la comete sera définie dans le fichier `settingsDefault.json` sous Visualisation/Mapbox/Traces/longueurComete. 1000 m par défaut

- Positionner la caméra en fonction du contenur du fichier `tracking.json` et de l'avancement pour avoir un effet de suivi de l'avancement  (flyby)

### Analyse de la Synchronisation entre Tracé, Caméra, Temps et Distance

Ce chapitre explique comment les différents éléments de l'animation "flyby" sont synchronisés. Comprendre cette relation est essentiel pour reproduire ou modifier l'effet.

#### 1. Les Trois Piliers de l'Animation

L'animation repose sur la synchronisation de trois concepts fondamentaux :

1. **Le Tracé au Sol (`LineString.json`)** :

   * **Ce que c'est** : Une simple liste de coordonnées `[lon, lat, elevation]` qui représente le parcours physique. )`elevation` ne semble pas utile)

   * **Son Unité de Mesure** : La **distance** (en kilomètres ou mètres). C'est la "vérité terrain". Chaque point du tracé est situé à une distance `d` du point de départ.

2. **Le Chemin de la Caméra (`tracking.json`)** :

   * **Ce que c'est** : Une liste de "points de contrôle" ou "keyframes" quand `pointDeControl = true`pour la caméra. Ce n'est PAS un simple tracé. Chaque point de contrôle définit un état complet de la caméra (`position`, `point regardé`, `altitude`) (`coordonneeCamera`, `coordonnee`, `altitudeCamera`) à un moment précis.

   * **Son Lien au Tracé** : Chaque point de contrôle de la caméra est **mappé à une distance spécifique sur le tracé au sol**. Par exemple, "quand le coureur est à 2.5km, la caméra doit être à telle position et regarder tel point".

3. **Le Temps (`time`)** :

   * **Ce que c'est** : Le temps réel qui s'écoule, fourni par `requestAnimationFrame`. C'est le moteur, l'horloge qui fait avancer l'animation.

   * **Son Rôle** : Il sert de base pour calculer la progression globale de l'animation.

#### 2. Le Chef d'Orchestre : La Variable `phase`

Le lien entre le **Temps** et la **Distance** est une variable normalisée appelée `phase`. C'est le concept le plus important.

* **Définition** : `phase` est un nombre qui évolue de **0** (début de l'animation) à **1** (fin de l'animation).

* **Calcul** : `phase = (temps_ecoule) / duree_totale_animation`

La `duree_totale_animation` est directement proportionnelle à la `distance_totale_trace`. C'est le point de connexion crucial :

`duree_totale_animation = distance_totale_trace * coefficient_de_vitesse`

> **Conclusion** : Grâce à la `phase`, on a transformé le **temps qui passe** en un **pourcentage de progression** de l'animation. Un temps `t` correspond désormais à `x%` de l'animation totale.

#### 3. La Synchronisation en Pratique

Voici comment la `phase` est utilisée pour synchroniser le tracé et la caméra à chaque image :

##### A. Positionner le Marqueur sur le Tracé

1. On calcule la distance actuelle à parcourir sur le tracé :

   `distance_actuelle = distance_totale_trace * phase`

2. On utilise `turf.along()` pour trouver les coordonnées exactes du point situé à cette `distance_actuelle` sur le `LineString` du tracé.

   `comete_sur_trace = turf.along(trace, distance_actuelle)`

3. Ce `comete_sur_trace` est utilisé pour mettre à jour la position du marqueur au sol.

> **Flux** : `Temps -> phase -> Pourcentage de Distance -> Coordonnées sur le Tracé`

##### B. Positionner la Caméra

1. On utilise la même `distance_actuelle` calculée précédemment pour trouver où se situer dans le fichier de données de la caméra (`tracking`). Le tableau `tracking` agit comme une table de correspondance ("lookup table") indexée par la distance.

2. On identifie les deux "keyframes" de caméra (`tracking[i]` et `tracking[i+1]`) entre lesquels le coureur se trouve actuellement.

3. On calcule une **phase de segment** (`nbrSegment`), qui est la progression (de 0 à 1) *entre ces deux keyframes de caméra*.

4. On interpole pour trouver la position et l'orientation exactes de la caméra :

   * La position de la caméra est interpolée entre la `positionCamera` de `tracking[i]` et `tracking[i+1]`.

   * Le point regardé par la caméra est interpolé entre le `lookAt` de `tracking[i]` et `tracking[i+1]`.

5. Ces valeurs interpolées sont appliquées à la caméra via `map.setFreeCameraOptions()`.

> **Flux** : `Temps -> phase -> Pourcentage de Distance -> Index dans 'tracking' -> Interpolation entre les keyframes -> Position & Orientation de la Caméra`

## 4. Schéma Récapitulatif

```
  Temps (fourni par le navigateur)

    |

    v

+-------------------------------------------------------------+

|   phase (0 à 1)                                             |  <-- Le chef d'orchestre

| (Basée sur la durée, qui dépend de la DISTANCE totale)      |

+-------------------------------------------------------------+

    |

    +--------------------------------+---------------------------------------+

    |                                |                                       |

    v                                v                                       v

+---------------------------+      +-----------------------------------+     +-------------------------+

| Position sur le Tracé     |      | Position de la Caméra             |     | Style du Tracé          |

|---------------------------|      |-----------------------------------|     |-------------------------|

| distance = total * phase  |      | 1. Trouve l'index dans 'tracking' |     | Le gradient de la ligne |

| point = turf.along(tracé, |      |    via la `distance`.             |     | progresse avec la `phase`|

|          distance)        |      | 2. Interpole entre les points     |     +-------------------------+

+---------------------------+      |    clés de la caméra.             |

    |                              +-----------------------------------+

    v                                    |

+---------------------------+            v

| Mise à jour du marqueur   |      +--------------------------------+

| (source de données)       |      | Mise à jour de la caméra       |

+---------------------------+      | (setFreeCameraOptions)         |

                                   +--------------------------------+
```

## Création du QRcode et ajout de commune dans circuits.json - Réalisé - [f773b733]

Lors de importation d'un fichier gpx dans l'application, nous faisons actuellement :

- La création des fichiers `lineString.json` et `tracking.json`

- La mise à jour du fichier `circuits.json`

Les travaux à réaliser dans cette étape sont : 

- Ajouter dans `tracking.json` un attribut `"commune": null`, pour chaque point. 

- Créer un QRCode de l'URL du circuit, dans un fichier urlQrcode.png. Il sera sauvegardé dans le dossier du circuit.

- Vérifier avant de commencer les traitements sur le fichier gpx, qu'il n'a pas déjà été importé. Pour cela on s'appuiera sur son nom.  On traitera en priorité lors de l'import la détection du nom de la trace, et on vérifiera qu'elle n'est pas déjà présente, via le fichier `circuits.json`. Si c'est le cas on informera l'utilisateur via un message à acquiter.

- Ajouter dans le fichier `circuits.json` le champ `"nomCommmunes": false`

- Remplacer dans le fichier `circuits.json` le champ `distanceVerifieeKm` par `trackingKm`. Normalement le champ `distanceVerifieeKm` n'était pas encore utilisé par l'application. Il faut quand même faire une vérification et faire les modification nécessaire à ce changement de nom. 

## Amélioration de CircuitListItem - Réalisé - [787788b8]

Il faut réaménager les informations présentées dans le composant `CircuitListItem`. Les informations à afficher sont issues du fichier circuits.json et seront affichées sur n colonne

- Colonne 1 : sur deux lignes  (aligné a gauche)
  
  - `**nom**`,
  
  - Ville de départ : `VilleDeDépart`

- Colonne 2 :  sur deux lignes (alignée à gauche)
  
  - **Distance : `distance`, Dénivelé : `dénivelé`**
  
  - Sommet : `sommet.alitude`, à `sommet.km` km

- Colonne 3 : sur une ligne (aligné à doite)
  
  - une jauge du rapport `trackingKm`/`distanceKm`
  
  - Par : `traceur`

On ajoute un v-btn informations apres le mdi-bug qui :

- quand on le survol affiche la vignette

- quand on clique dessus ouvre un composant information qui sera à créer plus tard.

le bouton d'edition sera :

- rouge (error) si  trackingKm== 0

- bleu  (primary) si trackingKm== distanceKm

- sinon orange ( warning)

le bouton view3D sera 

- rouge (error) si trackingKm== 0

- vert (success) si trackingKm== distanceKm

- sinon Orange (warning)

## Filtrage des circuits - Réalisé - [08151201]

Par défaut `CircuitListItem` affiche tous les circuits qui ont été importés page par page. Si  le nombre totoal de circuit est > au paramètre `Accuei:/circuitsPerPage` on affiche avant la v-list-item un bandeau horizontal Filtrage, qui va nous permettre de filtrer les circuits importés.

Le filtre pourra porté sur :

- Le nom du circuit : Un champ de saisie doit permettre de saisir une chaine de caractère. Si la chaine de caractère n'est pas présente dans le nom du circuit, ce dernier sera filtré

- Un interval de distance : un v-range-slider sera initialiser avec la plus petite et la plus grande distance des circuits importés. Lors de l'importation ou de la suppression d'un circuit les valeurs min et max du v-range-slider devront éventuellement être mise à jour. 

- Un intervalde dénivelé : un v-range-slider sera initialiser avec le plus petit et le plus grand dévinevlé des circuits importés. Lors de l'importation ou de la suppression d'un circuit les valeurs min et max du v-range-slider devront éventuellement être mise à jour.

- Le nom de la ville de départ : Un liste présentant les villes de départ connues sera crée. L'id de la ville permettra de filter les circuits qui ne partent pas de cette ville.

- Le nom du traceur : Un liste présentant les traceurs connus sera crée. L'id du traceur permettra de filter les circuits qui n'ont pas été tracé par lui. 

Un bouton permettre de réinitialiser tous les filtres

Un tri croissant décroissant pourra être appliqué a l'ensemble des champs de filtrage 

Les filtres seront disposés en trois colonnes comme les cicuits. La largeur des colonne devra être adaptée à la largeur des colonne du v-llist-item.

Le bouton de reinitialisation sera sur l'extrémité droite du bandeau. 

## Mise à jour de circuits.json lors de l'édition des circuits - Réalisé - [7f3ad688]

Dans la vue EditViews.vue on réalise des mises à jour du fichier tracking.json du circuit qui est en cours d'édition.

 C'est la mise à jour de pointDeControle, qui va nous interresser ici. 

A chaque mise à jour d'un pointDeControle, Si c'est le pointDeControl le plus eloigné du km 0 il faut mettre à jour le fichier circuits.json, pour le circuit consiéré en mettant à jour l'attribut  trackingKm, avec la valeur de  increment  * 1000 / Importation/Tracking/LongueurSegment qui correspond au paramètre de settingDeault.json. Cela nous servira à mettre à jour la jauge et les couleurs des boutons edition et visualisation 3D des circuit de la v-list-item de la vue MainView.vue.

## Préparation de la mise à jour des communes - Réalisé - [ fbc635e5]

Avant de se lancer dans la mise à jour des communes, nous avons besoin :

- de préparer des données de controle de cette fonctionnalité

- d'avoir une IHM de  lancement. pour la tâche de fond

- d'avoir un visuel sur l'avancement de la tâche de fond

### Données de control

Pour cette fonctionnalite, nous avons besoin :

- de modifier le fichier `circuits.json` pour y ajouter deux nouveaux attributs à la racine du fichier circuits.json :
  
  - `majCommunes` : un booleen qui est mis à true au lancement de la tache. 
    
    Il sera initialisé à false
  
  - `circuitId` : une string qui contient le circuitId du circuit qui est en cours de traitement.
    
    Il sera initialisé à "".

- d'un nouvel attribut pour circuits : un entier `avancementCommunes` 
  
  Il sera initialisé à 0

### IHM de lancement

Pour pourvoir lancer la tache de fond il faut ajouter un nouveau v-btn (mdi-city) sur la v-list-item du composant CircuitListItem. Ce bouton sera situé apres le mdi-information

Tous les boutons mdi-city seront disabled si la tâche de fond est active.

### Visuel d'avancement

Une nouvelle jauge devra être ajoutée sur la v-list-item du composant CircuitListItem en colonne 3  pour afficher``avancementCommunes` qui va de 0 à 100. 

## Mise à jour des communes - Réalisé - [1c39bd32]

La mise à jour des communes est à réaliser pour tous les points du fichier tracking.json du circuit considéré. C'est un processus qui peut se révéler extrèmement long, surtout pour les communes étrangères, car les services internet liés à cette fonctionnalite nous limitent grandement. (Au pire, une requete par seconde pour les communes étrangères à la france.)

On va donc créer une tache de fond qui va s'en charger. 

L'activité de cette tache de fond sera matérialisée par la présence ou pas d'une icone (verte) sur la toolbar de la mainView, qui sera déterminé par une variable globale `majCommuneIsRunning` par exemple.

### Déclenchement de la tâche

Trois cas sont à considérés :

- <u>Au  lancement de l'application</u> : Une vérification du fichier circuits.json, sera nécessaire, pour savoir si il faut lancer la tache de fond, en vérifiant les attributs `majCommunes`et `circuitId` situés à la racine du fichier settings.json. 
  
  Si `majCommunes` est à true, on relance la tache de fond pour le cicuit, pointé par `circuitId`.

- <u>Sur demande de l'utilisateur </u> : Si la tâche de fond n'est pas active, l'utilisateur peut déclencher la tache de fond via un bouton sur la v-list-item du composant CircuitListItem

- <u>Au changement de vue</u> : Quand on quitte la vue VisualizeView.vue

### Interuption de la tâche

- <u>Au changement de vue</u> : Quand on passe sur la vue VisualizeView.vue

### Fonctionnement de la tâche

**A son lancement** la tâche doit :

- Mettre la vaiable globale `majCommuneIsRunnig` à true, afin que l'icône puisse s'aficher sur la toolBar de la MainView.

- Mettre à jour les attributs `"majCommunes": true`et `"circuitId" : "circ-xxxx"` du fichier circuits.json.

- Calculer le nombre de points total à traiter via le fichier tracking.json

- Déactiver les boutons mdi-city

**A la fin de son éxecution** la tâche doit :

- Mettre la vaiable globale `majCommuneIsRunnig` à false pour suppirmer l'icone de la toolBar de la MainView.

- Mettre à jour les attributs `"majCommunes": false`et `"circuitId" : ""` du fichier circuits.json.

- Prévenir l'utilisateur de la fin de la Mise à jour des communes pour le circuit.nom, et lui rappeler qu'il doit selectionner un nouveau circuit si il veut quil ait une maj de ses communes. L'utilisateur devra acquitter le message.

- Réactiver les boutons mdi-city 

### Exécution de la tâche

Pour optimiser un niveau d'information peu précis au début mais un peu plus rapide, de  mise a jour des communes le long de la trace je propose que : 

- A partir du point 0 et tous les  16 points  on demande le nom de la commune si il n'est pas déjà connu (cas de relance de l'application)

- Puis à partir du point 8 et tous les 16 points on demande le nom de la commune si il n'est pas déjà connu,

- puis à partir du point 4 et tous les 8 points, on demande le nom de la commune si il n'est pas déjà connu,

- Puis à partir du point 2 et tous les 4 points, on demande le nom de la commune si il n'est pas déjà connu,

- Aet enfin à partir du point 1 et tous les 2 points, on demande le nom de la commune  si il n'est pas déjà connu.

Ainsi au bout du processus nous devrions avoir toutes les communes connues, avec une precision qui s'améliore au fils du temps.

A chaque maj d'un point on calcule le pourcentage d'avancement, et on met à jour dans circuits.json l'attribut `avancementCommunes` pour le circuit considéré.

Une attention particulière devra être portée à un possible accès multiple en écriture au fichiers tracking.json. et circuits.json, mais peut être que le backend n'est pas multithread, et que ce risque n'en est pas un !

### algorithme d'obtension d'un nom de commune.

la commune sera déterminée via l'attribut coordonnée du point.

En priorité on fait une requette au geoportail de l'IGN  toutes les 200 ms :

`https://api-adresse.data.gouv.fr/reverse/?lon=2.2945&lat=48.8584`

A cette requette deux cas peuvent se présenter :

- La réponse est OK : 
  
  la réponse est un json sous la forme :

{
"type": "FeatureCollection",
  "features": [
    {
      "geometry": {"type": "Point", "coordinates": [2.2945, 48.8584]},
      "properties": {
        "label": "Tour Eiffel, 75007 Paris",
        "city": "Paris",
        "context": "75, Île-de-France"
      },
      "type": "Feature"
    }
  ]
}

    On a le nom de la commune sur `city`

- La réponse est NOK : 

    La réponse est le json suivant : 

{
"type": "FeatureCollection",
  "features": []
}

    Dans ce cas on fait les prochaines requettes à mapbox toutes les 200 ms :

`https://api.mapbox.com/geocoding/v5/mapbox.places/-3.7038,40.4168.json?access_token=TON_TOKEN`

A cette requette deux cas peuvent se présenter : 

- La réponse est OK :

    La réponse est un json de ce type :

{
"type": "FeatureCollection",
  "query": [-3.7038, 40.4168],
  "features": [
    {
      "id": "place.1234567890",
      "type": "Feature",
      "place_type": ["place"],
      "relevance": 1,
      "properties": {},
      "text": "Madrid",
      "place_name": "Madrid, Community of Madrid, Spain",
      "center": [-3.7038, 40.4168],
      "geometry": {
        "type": "Point",
        "coordinates": [-3.7038, 40.4168]
      },
      "context": [
        {
          "id": "region.987654321",
          "text": "Community of Madrid"
        },
        {
          "id": "country.11223344",
          "short_code": "es",
          "text": "Spain"
        }
      ]
    }
  ],
  "attribution": "NOTICE: © 2025 Mapbox and its suppliers. All rights reserved."
}

- La réponse est NOK :

    On reçoit un code HTTP 429 (Quota dépassé) ou 403 (Pas de droits pour l'API)

    Dans ce cas on fait les prochaines requettes à openStreetMap, toutes les secondes :

`https://nominatim.openstreetmap.org/reverse?format=json&lat=40.4168&lon=-3.7038&zoom=10&addressdetails=1`

- La réponse est un json de ce type : 

{
"place_id": 123456789,
  "licence": "Data © OpenStreetMap contributors, ODbL 1.0. https://osm.org/copyright",
  "osm_type": "relation",
  "osm_id": 12345,
  "lat": "40.4168",
  "lon": "-3.7038",
  "display_name": "Madrid, Área metropolitana de Madrid y Corredor del Henares, Community of Madrid, Spain",
  "address": {
    "city": "Madrid",
    "municipality": "Madrid",
    "county": "Área metropolitana de Madrid y Corredor del Henares",
    "state": "Community of Madrid",
    "country": "Spain",
    "country_code": "es"
  },
  "boundingbox": ["40.3121","40.5733","-3.8880","-3.5179"]
}

    Avec openStreetMap selon le lieu, la clé peut être `city`, `town`, `village`, ou même `hamlet`.

## Création des évènements - En cours - []

Au cours de visualisation 3D de la trace, j'aimerai avoir 4  types d'évènements programmés qui sont :

- les `pause`, qui mettent automatiquement la visualisation 3D en pause

- Les `zoom` qui permettent de faire un zoom sur un point particulier puis de revenir à la vision normale de la trace. Durant cette séquence le déroulement de la visualisation 3D est en pause.

- Les `infoGeo`, qui permettre d'afficher une information sur la visualisation 3D durant un certain temps à une coordonnées géographique précise.

- les `info` qui affichent une information dans un widget dédié durant un certain temps. 

Les paramètres de ces évènements seront regroupés dans le fichier evt.json qui sera sauvegardé dans le repertoire du circuit. Si le fichier n'existe pas il faudra le créer. Je propose de créer dans le backend, un fichier event.rs pour gérer ce fichier

C'est dans EditView que l'on va ajouter et parametrer ces évèments.

Pour cela nous allons créer un nouveau widget  identique au control-widget disposé avant le graphe

Ce widget aura un v-switch `Evènement`, qui permettre d'ouvrir un div sur lequel nous allons avoir notre IHM pour ajouter/supprimer/modifier les évènements

### Ajout d'un évènement Pause - Réalisé -

Au cours de visualisation 3D de la carte, j'aimerai avoir des pauses programmées synchronisées avec le fichier tracking.json.

L'ajout d'une pause se fera via la touche `p` on récupère la progression et on l'ajoute dans le fichier evt.json. Les points devront être classés dans l'ordre croissant.

Le format des données pour ce type d'évènement sera : 

`[

    {

        "pause" :  [99, 190,139...]

    }

]`

### Suppression d'un évènement Pause - Réalisé -

Dans le widget Evenement si la progresson correspond à un évènement pause un bouton sera affiché dans le wiget pour supprimer la pause. On supprime la pause du tableau pause . 

### Ajout d'un évènement Flyto

Pour ajouter un évènement flyto, nous avons besoin de paramétrer dans l'onglet EVENEMENT/FLYTO sa durée. Pour cela nous allons ajouter un slider seconde, qui sera paramétré dans setting.json/ sous Edition/Evenement/Flyto/durée en seconde de 0.2 à 10 secondes avec un pas de 0.2 Seconde et une valeur par défaut de 2.

Ensuite via la touche F on ajoute l'evenement dans le fichier evt.json



Le flyto sera matérialisé dans le graphe comme la pause avec ses propres paramètres dans Edition/Evenements/Flyto/  avec une couleur orange. 

Les flyto et les pause ne peuvent pas être déclarés pour un même point. Ce cas doit être détecté avant la mise à jour du fichier evt.json, et un v-dialog devra prévenir l'utilisateur si il vaut faire le changement ou pas ! 

Si présence d'un flyto sur la progression on doit pourvoir le supprimer via l'onglet EVENEMENT/FLYTO.

Un refonte du fichier evt.json peut être nécessaire. Si c'est le cas bien penser à vérifier l'ensemble des impact sur le graphe etc... 

### 

### Ajout de balises texte (Geo et widget)

## 

## Zooms automatiques au départ et à l'arrivée.

## Profil d'altitude

## Détection des pentes

## Suppession des fichiers après l'import

## Fiche circuit

---

## 🤝 Contribution

Pour contribuer, veuillez vous référer au guide de contribution dans `CONTRIBUTING.md`

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
