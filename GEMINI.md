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

## Etape 0

l'étape 0 consiste à mettre en place l'environement technique. Elle n'a pas vocation à créer les ihm. les vues seront crées uniquement afin de pouvoir configurer le router. 

## Etape 0.1 - Réalisée - 8afea17

Mise en place de git :

`.gitignore`

`CONTRIBUTING.md`

`README.md`

`LISENCE`

## Etape 0.2 - Réalisée - 59988f9

Mise en place de gemini

`GEMINI.md`

## Etape 0.3 - Réalisée - 50a5f12

le npm initiale pour avoir les packages nécessaires.

## Etape 0.4 - Réalisée - dd389466

Configuration de

Tauri, vueJS, du routeur

Création des vues :

- `src/views/MainView.vue` pour la vue principale

- `src/views/EditView.vue` pour l'edition et la vérification du flyover

- `src/views/VisualizeView.vue`pour le rendu 3D du flyover.

- `src/views/settingsView.vue` pour le paramétrage de l'application

## Etape 0.5 - Réalisée - [74249be4]

Mise en place de l'environnement d'éxécution

Dans le processus de développement de l'application nous avons deux **Environnements de travail** :

- L'environnement de  **Dev** lorsqu'on lance la commande `npm run dev`

- L'environnemnt de **Prod**  lorsqu'on lance la commande `npm run tauri build` pour générer la livraison de l'application.

Afin de pouvoir faire des tests fonctionnels ou de la validation de configuration sans impacter les fichiers de configuration utilisés en production, nous allons avoir besoin d'un **Environnement d'éxécution**. Il faut bien faire la distinction entre ces deux types d'environnement.

Les **Environnements d'éxécution** devront être disponibles dans les deux **Environnements de travail**. 

Pour changer d'environnement d'éxecution, il sera nécessaire de relancer l'application.

L'environnement d'exécution sera défini par la variable `APP_ENV` dans le fichier `.env`. Les valeurs possibles pour cette variable suivent une convention précise :

- `OPE` : Pour l'utilisation normale de l'application. Ce sera la valeur par défaut si le fichier `.env` est absent.
- `Sandbox_NOM_FEATURE` : Pour la validation d'une configuration ou d'une fonctionnalité spécifique (ex: `Sandbox_Camera`).
- `Test_NOM_TEST` : Pour l'exécution d'un test fonctionnel particulier (ex: `Test_CalculLongueur`).

À partir de cette variable, l'application devra déduire un **mode d'exécution** (par exemple, dans une variable `executionMode`) qui pourra être `OPE`, `EVAL`, ou `TEST`. Cela permettra d'adapter le comportement de l'application en fonction du contexte (ex: charger des données de test, afficher des informations de debug, etc.).

Le fichier `.env`, ne sera pas livré avec l'application, car il contiendra plus tard des tokens privés. 

Le fichier .env si il existe sera présent sous `C:\Users\username\AppData\Roaming\com.visugps.app\VisuGPS`

Il devra être lu à partir de ce dossier que l'on soit en mode de travail Dev ou Prod. Si ce ficheier n'existe pas au lancement de l'application, il devra être créé, pour avoir un mode d'éxécution `OPE` que l'on soit en mode de travail dev ou  Prod

### Répertoires d'Environnement

En plus de déterminer le mode d'exécution, l'application crée un dossier de travail dont le nom correspond à la valeur de la variable `APP_ENV`. Ce dossier est créé à l'intérieur du répertoire de données de l'application (`C:\Users\username\AppData\Roaming\com.visugps.app\VisuGPS\`).

Par exemple, si `APP_ENV` est `Test_Setup`, le dossier suivant sera créé : `C:\Users\username\AppData\Roaming\com.visugps.app\VisuGPS\Test_Setup\`.

Tous les fichiers de configuration, de test ou de données spécifiques à un environnement devront être lus et écrits dans ce répertoire.

### État de l'Application Partagé

Les informations sur l'environnement sont centralisées dans un état (`AppState`) géré par Tauri et accessible depuis le frontend. Cet état contient :

- `app_env`: La valeur brute de la variable `APP_ENV`.
- `execution_mode`: Le mode déduit (`OPE`, `EVAL`, `TEST`).
- `app_env_path`: Le chemin complet vers le répertoire de l'environnement de travail.

---

## 🤝 Contribution

Pour contribuer, veuillez vous référer au guide de contribution dans `CONTRIBUTING.md`.

---

## 📄 Licence

Ce projet est sous licence **Apache 2.0**. Pour plus de détails, consultez le fichier **`LICENSE`** à la racine de ce projet.