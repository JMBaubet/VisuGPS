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

On gardera dans ce fichier l'historique des étapes, afin que tu saches quelles sont les étapes déjà réalisées et les principes mis en oeuvre. 

Lorsqu'une étape me semble complètement réalisée, cette dernère au niveau de son titre `heading 2` aura un sufixe `Réalisée`.

Les étapes seront numérotées sous la forme Etape x.y.z 

- x : représentant un avancement majeur de l'application

- y : représentéant une nouvelle feature de l'application

- z : représentant, un point de control de la feature en cours de réalisation

Afin que tu n'ancitipes pas le codage de certaines features, les étapes te seront décrite au fur et à mesure. 

A chaque réalisation d'étape, j'enregistrerai dans ce fichier l'identifiant du commit git

## Visualisation des Flyto Réalisée

Nous allons travailler sur la vue VisualizeView, pour la prise en compte des evenements décrits dans le fichier evt.json

## Visualisation des Messages

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