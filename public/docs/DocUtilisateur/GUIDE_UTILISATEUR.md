# Guide Utilisateur de VisuGPS

Ce guide vous accompagne à travers les fonctionnalités principales de l'application VisuGPS, de l'importation d'une trace GPX à sa visualisation animée en 3D.

## 1. Écran d'Accueil : La Bibliothèque de Circuits

L'application s'ouvre sur l'écran d'accueil, votre bibliothèque personnelle de circuits. L'interface est organisée en trois zones principales pour une navigation intuitive.

<svg width="100%" viewBox="0 0 600 400" xmlns="http://www.w3.org/2000/svg" font-family="sans-serif" font-size="14">
<defs>
<style>
.label { font-size: 16px; font-weight: bold; text-anchor: middle; fill: #444; }
.box { fill: #FFFFFF; stroke: #BDBDBD; stroke-width: 1.5; rx: 8; }
</style>
</defs>
<rect width="600" height="400" fill="#F7F9FA"/>
<rect x="10" y="10" width="580" height="70" class="box"/>
<text x="300" y="50" class="label">1. Barre d'outils</text>
<rect x="10" y="90" width="580" height="60" class="box"/>
<text x="300" y="125" class="label">2. Panneau de filtres</text>
<rect x="10" y="160" width="580" height="230" class="box"/>
<text x="300" y="280" class="label">3. Liste des circuits</text>
</svg>

### 1.1 Barre d'outils

La barre d'outils, située en haut, regroupe les indicateurs de statut et les actions globales.

<img src="./images/ToolBar.png" alt="Barre d'outils de l'application" style="width:100%;">
 
1.  **Statut des Services** : Une série d'icônes vous informe de l'état des connexions.
    *   ![web-check](https://api.iconify.design/mdi/web-check.svg?color=green&width=24) **Connecté**
    *   ![web-off](https://api.iconify.design/mdi/web-off.svg?color=red&width=24) **Hors ligne**
    *   ![mapbox](https://api.iconify.design/mdi/mapbox.svg?color=blue&width=24) **Mapbox injoignable**
    *   ![key-alert](https://api.iconify.design/mdi/key-alert.svg?color=red&width=24) **Token Mapbox invalide**
2.  **Statut de la Télécommande** :
    *   ![remote](https://api.iconify.design/mdi/remote.svg?color=green&width=24) **Connectée**
    *   ![remote-off](https://api.iconify.design/mdi/remote-off.svg?color=blue&width=24) **Déconnectée** (cliquez pour afficher le QR Code de connexion)
3.  **Panneau de Mise à Jour des Communes** : Apparaît lorsqu'une recherche de communes est lancée. Il contient le nom du circuit, une barre de progression, des interrupteurs pour les API IGN/Mapbox et un bouton pour arrêter.
4.  **Mode d'Exécution** : Un badge indique un mode spécial (ex: `EVAL`).
5.  **Actions Principales** :
    *   ![file-upload](https://api.iconify.design/mdi/file-upload.svg?color=currentColor&width=24) **Importer un circuit**
    *   ![cog](https://api.iconify.design/mdi/cog.svg?color=currentColor&width=24) **Paramètres**

### 1.2 Filtrer et Trier les Circuits

Ce panneau vous permet d'affiner et d'organiser la liste des circuits.

<svg width="100%" viewBox="0 0 800 60" xmlns="http://www.w3.org/2000/svg" font-family="sans-serif" font-size="12px">
<rect x="1" y="1" width="798" height="58" fill="#FFF" stroke="#BDBDBD" rx="8"/>
<g transform="translate(10, 15)">
<rect x="0" y="0" width="180" height="30" fill="#F5F5F5" rx="4" stroke="#E0E0E0"/>
<text x="10" y="20" fill="#757575">Filtrer par nom...</text>
</g>
<text x="100" y="12" text-anchor="middle" font-weight="bold">1</text>

<g transform="translate(200, 15)">
<rect x="0" y="0" width="100" height="30" fill="#F5F5F5" rx="4" stroke="#E0E0E0"/>
<text x="10" y="20" fill="#757575">Ville de départ</text>
<image href="https://api.iconify.design/mdi/chevron-down.svg" x="75" y="5" width="20" height="20"/>

<rect x="110" y="0" width="100" height="30" fill="#F5F5F5" rx="4" stroke="#E0E0E0"/>
<text x="120" y="20" fill="#757575">Traceur</text>
<image href="https://api.iconify.design/mdi/chevron-down.svg" x="185" y="5" width="20" height="20"/>

<text x="225" y="20">Distance:</text>
<line x1="280" y1="15" x2="380" y2="15" stroke="#BDBDBD" stroke-width="4" stroke-linecap="round"/>
<circle cx="300" cy="15" r="6" fill="#1976D2" stroke="white" stroke-width="2"/>
<circle cx="360" cy="15" r="6" fill="#1976D2" stroke="white" stroke-width="2"/>
</g>
<text x="340" y="12" text-anchor="middle" font-weight="bold">2</text>

<g transform="translate(600, 15)">
<text x="0" y="20">Trier par:</text>
<rect x="60" y="0" width="120" height="30" fill="#F5F5F5" rx="4" stroke="#E0E0E0"/>
<text x="70" y="20" fill="#757575">Date d'ajout</text>
<image href="https://api.iconify.design/mdi/chevron-down.svg" x="155" y="5" width="20" height="20"/>
<image href="https://api.iconify.design/mdi/arrow-up.svg" x="185" y="5" width="20" height="20"/>
</g>
<text x="695" y="12" text-anchor="middle" font-weight="bold">3</text>
</svg>

1.  **Filtrer par nom** : Champ de recherche textuel.
2.  **Filtres avancés** :
    *   Listes déroulantes pour la **Ville de départ** et le **Traceur**.
    *   Curseurs de fourchette pour la **Distance** et le **Dénivelé**.
3.  **Trier par** : Boutons pour trier par date, nom, etc., avec un sélecteur pour l'ordre (ascendant/descendant).

### 1.3 La Liste des Circuits

La zone principale affiche les circuits sous forme de liste. Chaque ligne est une carte interactive.

<svg width="100%" viewBox="0 0 800 100" xmlns="http://www.w3.org/2000/svg" font-family="sans-serif" font-size="12px">
<rect x="1" y="1" width="798" height="98" fill="#FFF" stroke="#BDBDBD" rx="8"/>

<g transform="translate(20, 20)">
<text font-size="16px" font-weight="bold">Nom du Circuit</text>
<text y="18" fill="#757575">Ville de départ</text>
</g>
<text x="20" y="12" font-weight="bold">A</text>

<g transform="translate(20, 60)">
<image href="https://api.iconify.design/mdi/map-marker-distance.svg" width="16" height="16" y="-13"/>
<text x="20" y="0">Distance: 123.4 km</text>
<image href="https://api.iconify.design/mdi/elevation-rise.svg" x="150" width="16" height="16" y="-13"/>
<text x="170" y="0">Dénivelé: 5678 m</text>
<image href="https://api.iconify.design/mdi/peak.svg" x="300" width="16" height="16" y="-13"/>
<text x="320" y="0">Sommet: 1234m (à 56.7 km)</text>
</g>
<text x="200" y="88" text-anchor="middle" font-weight="bold">B</text>

<g transform="translate(450, 25)">
<text fill="#757575">Traceur:</text>
<text x="50" font-weight="bold">Nom de l'auteur</text>
</g>
<text x="450" y="15" font-weight="bold">C</text>

<g transform="translate(450, 45)">
<rect x="0" y="0" width="150" height="8" fill="#E0E0E0" rx="4"/>
<rect x="0" y="0" width="100" height="8" fill="#FFC107" rx="4"/>
<text x="160" y="8" font-size="10px">Édition (66%)</text>
</g>
<text x="450" y="65" font-weight="bold">D</text>

<g transform="translate(630, 35)">
<title>E. Mettre à jour les communes</title>
<image href="https://api.iconify.design/mdi/city.svg?color=orange" width="24" height="24"/>
<title>F. Afficher la vignette</title>
<image href="https://api.iconify.design/mdi/information.svg?color=grey" x="35" width="24" height="24"/>
<title>G. Éditer le circuit</title>
<image href="https://api.iconify.design/mdi/pencil.svg?color=orange" x="70" width="24" height="24"/>
<title>H. Visualiser en 3D</title>
<image href="https://api.iconify.design/mdi/eye.svg?color=orange" x="105" width="24" height="24"/>
<title>I. Supprimer le circuit</title>
<image href="https://api.iconify.design/mdi/delete.svg?color=red" x="140" width="24" height="24"/>
</g>
<text x="710" y="25" text-anchor="middle" font-weight="bold">E, F, G, H, I</text>
</svg>
#### Informations sur la carte du circuit :

*   **A. Nom du circuit et Ville de départ**.
*   **B. Statistiques** : Distance, Dénivelé, Sommet (altitude et km).
*   **C. Traceur** : Auteur de la trace.
*   **D. Jauge d'édition** : Barre de progression indiquant le niveau d'édition du circuit.

#### Actions et Icônes :

*   **E. Mettre à jour les communes** (![city](https://api.iconify.design/mdi/city.svg?color=currentColor&width=18)) : L'icône change de couleur (Rouge, Orange, Vert) selon l'état.
*   **F. Afficher la vignette** (![information](https://api.iconify.design/mdi/information.svg?color=currentColor&width=18)) : Affiche un aperçu 2D au survol.
*   **G. Éditer le circuit** (![pencil](https://api.iconify.design/mdi/pencil.svg?color=currentColor&width=18)) : L'icône change de couleur (Rouge, Orange, Bleu) selon la progression.
*   **H. Visualiser en 3D** (![eye](https://api.iconify.design/mdi/eye.svg?color=currentColor&width=18)) : L'icône change de couleur (Rouge, Orange, Vert) selon la qualité de l'édition.
*   **I. Supprimer le circuit** (![delete](https://api.iconify.design/mdi/delete.svg?color=red&width=18)).

### Fonctionnement du Couplage de la Télécommande

Le couplage de la télécommande dans VisuGPS est un processus sécurisé qui permet à votre appareil mobile de se connecter à l'application de bureau pour la contrôler. Voici comment cela fonctionne en coulisses :

1.  **Démarrage du Serveur de Télécommande** : Lorsque l'application VisuGPS démarre, un serveur WebSocket est lancé en arrière-plan sur votre ordinateur. Ce serveur écoute les connexions entrantes sur un port spécifique (par défaut 9001, configurable dans les paramètres). Il sert également les fichiers nécessaires à l'interface de la télécommande sur votre appareil mobile.
2.  **Génération du QR Code et de l'URL** : Lorsque vous cliquez sur l'icône de télécommande déconnectée, une fenêtre de dialogue s'ouvre. L'application génère une URL unique (contenant l'adresse IP de votre ordinateur et le port du serveur) et un QR Code correspondant, qui sont affichés.
3.  **Connexion depuis l'Appareil Mobile** : Sur votre appareil mobile, vous scannez le QR Code ou saisissez l'URL dans un navigateur web. Cela charge l'interface de la télécommande et tente d'établir une connexion WebSocket avec le serveur de VisuGPS.
4.  **Demande de Couplage** : L'appareil mobile envoie une "demande de couplage" à l'application de bureau, incluant un identifiant unique et un code de couplage.
5.  **Approbation de l'Utilisateur** : Pour des raisons de sécurité, une notification ou une boîte de dialogue apparaît sur votre bureau, vous demandant d'approuver ou de refuser la connexion de cet appareil mobile.
6.  **Établissement de la Connexion et Communication** : Une fois le couplage approuvé, la connexion WebSocket est établie. L'icône de la télécommande dans l'application de bureau passe au vert ![remote](https://api.iconify.design/mdi/remote.svg?color=green&width=24). L'appareil mobile peut alors envoyer des commandes et recevoir des mises à jour d'état.
7.  **Déconnexion** : Vous pouvez déconnecter la télécommande à tout moment en cliquant sur l'icône verte ![remote](https://api.iconify.design/mdi/remote.svg?color=green&width=24) et en confirmant la déconnexion.


## 2. Processus d'Importation d'un Fichier GPX

L'importation d'un nouveau circuit est un processus guidé en quatre étapes, conçu pour assurer la qualité et la cohérence de vos données.

<svg width="100%" viewBox="0 0 800 500" xmlns="http://www.w3.org/2000/svg" font-family="sans-serif">
<defs>
<marker id="arrow-down" viewBox="0 0 10 10" refX="5" refY="8" markerWidth="6" markerHeight="6" orient="auto">
<path d="M 0 0 L 5 10 L 10 0 z" fill="#555" />
</marker>
<style>
.step-box { fill: #F7F9FA; stroke: #BDBDBD; stroke-width: 1; rx: 8; }
.icon-bg { fill: #E3F2FD; }
.icon { fill: #1E88E5; }
.step-title { font-size: 16px; font-weight: bold; fill: #333; }
.step-desc { font-size: 13px; fill: #555; }
.arrow-line { stroke: #555; stroke-width: 2; marker-end: url(#arrow-down); }
</style>
</defs>

<g transform="translate(50, 20)">
<rect class="step-box" width="700" height="80"/>
<circle class="icon-bg" cx="60" cy="40" r="25"/>
<image href="https://api.iconify.design/mdi/file-search-outline.svg" x="40" y="20" width="40" height="40" class="icon"/>
<text class="step-title" x="110" y="35">1. Sélection du Fichier</text>
<text class="step-desc" x="110" y="60">Cliquez sur l'icône d'import, trouvez votre fichier .gpx et cliquez sur "Importer".</text>
</g>

<line class="arrow-line" x1="400" y1="105" x2="400" y2="135" />

<g transform="translate(50, 140)">
<rect class="step-box" width="700" height="80"/>
<circle class="icon-bg" cx="60" cy="40" r="25"/>
<image href="https://api.iconify.design/mdi/chart-line.svg" x="40" y="20" width="40" height="40" class="icon"/>
<text class="step-title" x="110" y="35">2. Analyse Automatique</text>
<text class="step-desc" x="110" y="60">Le système analyse la trace, calcule la distance, le dénivelé et la ville de départ.</text>
</g>

<line class="arrow-line" x1="400" y1="225" x2="400" y2="255" />

<g transform="translate(50, 260)">
<rect class="step-box" width="700" height="80"/>
<circle class="icon-bg" cx="60" cy="40" r="25"/>
<image href="https://api.iconify.design/mdi/account-question-outline.svg" x="40" y="20" width="40" height="40" class="icon"/>
<text class="step-title" x="110" y="35">3. Sélection du Traceur</text>
<text class="step-desc" x="110" y="60">Choisissez l'auteur de la trace dans la liste ou créez-en un nouveau. Cette étape est obligatoire.</text>
</g>

<line class="arrow-line" x1="400" y1="345" x2="400" y2="375" />

<g transform="translate(50, 380)">
<rect class="step-box" width="700" height="80"/>
<circle class="icon-bg" cx="60" cy="40" r="25"/>
<image href="https://api.iconify.design/mdi/cogs.svg" x="40" y="20" width="40" height="40" class="icon"/>
<text class="step-title" x="110" y="35">4. Finalisation et Génération</text>
<text class="step-desc" x="110" y="60">L'application sauvegarde le circuit, génère une vignette 2D et prépare les fichiers pour l'animation.</text>
</g>
</svg>
Une fois ces étapes terminées, le nouveau circuit est prêt et apparaît dans votre bibliothèque.

---

## 3. Vue d'Édition : Personnaliser l'Animation

C'est ici que vous devenez le réalisateur de votre film. La vue d'édition vous permet de sculpter l'expérience de visualisation en définissant les mouvements de caméra et en ajoutant des événements le long du parcours.

<svg width="100%" viewBox="0 0 800 400" xmlns="http://www.w3.org/2000/svg" font-family="sans-serif">
<rect width="800" height="400" fill="#F5F5F5"/>
<rect x="10" y="10" width="580" height="380" fill="#DDEEDD" stroke="#BDBDBD"/>
<path d="M 50,350 C 150,200 300,250 500,100" stroke="#F44336" stroke-width="4" fill="none" stroke-linecap="round"/>
<text x="290" y="200" text-anchor="middle" font-size="24px" fill="#333" opacity="0.5">Vue 3D Interactive</text>

<rect x="600" y="10" width="190" height="380" fill="#FFF" stroke="#BDBDBD"/>
<text x="695" y="40" text-anchor="middle" font-weight="bold" font-size="16px">Panneau d'Édition</text>

<g transform="translate(610, 70)">
<text font-weight="bold">Caméra</text>
<text x="10" y="20">Pitch:</text><rect x="50" y="10" width="100" height="15" fill="#E0E0E0" rx="3"/>
<text x="10" y="45">Bearing:</text><rect x="60" y="35" width="90" height="15" fill="#E0E0E0" rx="3"/>
<text x="10" y="70">Zoom:</text><rect x="50" y="60" width="100" height="15" fill="#E0E0E0" rx="3"/>
</g>

<g transform="translate(610, 160)">
<text font-weight="bold">Événements</text>
<rect x="0" y="15" width="170" height="150" fill="#F5F5F5" stroke="#E0E0E0"/>
<text x="85" y="90" text-anchor="middle" fill="#757575">Timeline des événements</text>
<rect x="20" y="30" width="130" height="20" fill="#BBDEFB" rx="4"/>
<text x="25" y="44">Fly-To: Sommet</text>
</g>

<g transform="translate(610, 340)">
<rect x="0" y="0" width="170" height="40" fill="#4CAF50" rx="5"/>
<text x="85" y="25" text-anchor="middle" fill="white" font-weight="bold">Sauvegarder</text>
</g>
</svg>
Les fonctionnalités clés incluent :

-   **Visualisation 3D Interactive** : Explorez votre trace librement en 3D.
-   **Positionnement de la Caméra** : À n'importe quel point du parcours, ajustez le tangage (pitch), le cap (bearing) et le zoom de la caméra.
-   **Création d'Événements** : Ajoutez des événements spéciaux comme des "Fly-To" vers des points d'intérêt ou des pauses.
-   **Sauvegarde des Paramètres** : Toutes vos modifications sont enregistrées dans un fichier `visu.json` et `evt.json`, prêtes à être utilisées par la vue de visualisation.

## 4. Vue de Visualisation : Le Spectacle Commence

Cette vue est le cœur de l'expérience VisuGPS, où votre trace GPX prend vie à travers une animation 3D immersive. Elle lit les paramètres que vous avez définis en édition pour créer un "survol" dynamique de votre parcours.

### 4.1 Le Moteur de l'Animation

L'animation est orchestrée par une variable interne appelée `phase`, qui progresse de 0 (début) à 1 (fin) de la trace. Cette `phase` est synchronisée avec le temps et la distance parcourue, permettant de :
*   **Positionner le marqueur** : Un marqueur se déplace le long de la trace, indiquant votre progression.
*   **Contrôler la caméra** : La position, l'orientation (cap, tangage) et le zoom de la caméra sont ajustés en continu, en suivant les points de contrôle que vous avez définis.

### 4.2 Les Étapes de l'Animation

La visualisation suit une séquence d'états prédéfinie pour offrir une expérience cohérente :

1.  **Séquence d'Introduction** :
    *   **Vol vers la vue globale** : La caméra effectue un premier vol pour présenter une vue d'ensemble de la trace.
    *   **Pause d'observation** : Une courte pause permet d'assimiler le parcours global.
    *   **Vol vers le départ** : La caméra se positionne ensuite précisément au point de départ (km 0).
2.  **Boucle Principale de l'Animation** :
    *   **En animation** : La caméra suit la trace, le marqueur avance.
    *   **En pause** : L'animation peut être mise en pause (touche `P`) ou par un événement programmé.
    *   **Survol événementiel (Fly-To)** : L'animation principale est suspendue et la caméra effectue un vol dynamique vers un point d'intérêt.
3.  **Séquence de Fin** :
    *   **Vol final** : Une fois la fin de la trace atteinte, la caméra effectue un dernier vol pour revenir à une vue d'ensemble.
    *   **Terminé** : L'animation est terminée. Vous pouvez la réinitialiser (touche `R`).

### 4.3 Les Événements "Fly-To"

Moments clés de l'animation, les "Fly-To" sont des vols de caméra vers un point d'intérêt spécifique, offrant une perspective différente avant de revenir au parcours. Chaque événement est défini par un cap, des coordonnées, une durée, un tangage et un zoom.

