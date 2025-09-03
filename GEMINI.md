# 🗺️ Projet VisuGPS : Visualisation 3D de fichiers GPX

Ce document décrit le projet **VisuGPS**, une application de bureau multiplateforme pour la visualisation 3D de fichiers GPX, axée sur la création d'animations.

---

## 🚀 Démarrage rapide

### Prérequis

* **Node.js** (version 18 ou supérieure)
* **Rust** et **Cargo**
* **Mapbox GL JS** et **Turf.js** pour l'analyse spatiale.

### Installation et exécution

1.  Clonez le dépôt : `git clone https://github.com/JMBaubet/VisuGPS`
2.  Installez les dépendances : `npm install`
3.  Lancez l'application en mode développement : `npm run tauri dev`

---

## 🛠️ Technologies utilisées

* **Tauri** : Pour la création d'applications de bureau natives.
* **Vue.js 3** & **Vuetify** : Pour une interface utilisateur réactive et élégante.
* **Mapbox GL JS** : Pour l'affichage de la carte 3D.
* **Turf.js** : Pour l'analyse et le traitement des données géospatiales (calcul de longueur, etc.).
* **Fichiers JSON** : Utilisés pour la sauvegarde des paramètres d'édition et de configuration.

Le contenu de ce fichier devra pouvoir être modifié via la vue  Paramétrage.vue en dev et en prod. il doit également être livré avec l'application. 

La structure du fichier JSON sera la suivante :
pour chaque paramètre nous devons avoir les attributs suivant : 
⦁	le nom du paramètre. Ce nom devra être unique dans le fichier
⦁	La description du paramètre. Ce champs doit permettre d'expliquer à l'utilisateur le rôle du paramètre dans l'application
⦁	Le type de donnée du paramètre. Ce champ doit permettre de vérifier la cohérence du paramètre. Il doit également servir à l'éditeur de paramètre. vue Setting.vue
⦁	L'unité du paramètre. Par exemple des ms pour les timer, RGBA pour une couleur
⦁	La valeur par défaut du paramètre. Valeur définie par le développeur. Elle ne peut pas être null. Cette valeur sera utilisée si la valeur de surcharge est à null
⦁	La valeur de surcharge. Valeur définie par l'utilisateur pour adapter l'application a ces préférences.
⦁	Valeur min : Valeur minimum que peut prendre le paramètre.
⦁	Valeur Max : Valeur maximum que peut prendre le paramètre
⦁	Warning : Booleen pour indiquer à l'utilisateur que la modification de cette valeur peut engendrer des dysfonctionnement de l'application. 
⦁	Arbre : Une chaine de caractère du type niveau_1/niveau_2/../niveau_n pour regrouper les paramètres en fonction e leur utilisation dans l'application.
L'utilisateur ne pourra modifier que le champ valeur de surcharge.

---

## 📂 Organisation du projet et des vues

L'application est divisée en plusieurs vues distinctes pour une meilleure clarté et gestion du flux de travail :

* ### Vue Principale (VueJS)
    Cette vue affiche une liste des traces GPX déjà téléchargées.
    * **Liste des traces** : Chaque élément de la liste présente une vignette 2D de la trace.
    * **Vignette 2D** : Créée avec Mapbox ou une autre librairie, elle affiche la trace avec un fond de carte simple.
    * **Bouton de téléchargement** : Permet à l'utilisateur de charger un nouveau fichier GPX depuis son dossier de téléchargement.
    
    À la suite du téléchargement, les actions suivantes sont automatiquement effectuées :
    1.  **Calcul de la longueur** : Utilisation de la librairie **Turf.js** pour déterminer la longueur de la trace.
    2.  **Génération d'une vignette** : Création et affichage de la miniature 2D pour la liste.

* ### Vue Édition
    Cette vue est dédiée à la configuration de la vue 3D et des animations.
    * **Visualisation 3D** : La trace GPX est affichée dans un environnement 3D interactif.
    * **Positionnement de la caméra** : L'utilisateur peut ajuster la position et l'angle de la caméra.
    * **Sauvegarde des paramètres** : Les paramètres de la caméra, essentiels pour la future animation, sont sauvegardés dans un fichier JSON.

* ### Vue Visualisation
    Cette vue permet de lire le fichier JSON créé en mode Édition pour visualiser une ou plusieurs traces avec une animation automatique.
    * **Animation du marqueur** : Un marqueur se déplace le long de la trace selon les paramètres de caméra et de temps définis.

* ### Vue Paramétrage
    Cette vue sert à configurer les paramètres globaux de l'application, qui sont stockés dans un fichier JSON dédié.

---

## 🤝 Contribution

Pour contribuer, veuillez vous référer au guide de contribution dans `CONTRIBUTING.md`.

---

## 📄 Licence

Ce projet est sous licence **Apache 2.0**. Pour plus de détails, consultez le fichier **`LICENSE`** à la racine de ce projet.