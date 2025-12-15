# Annexe des Paramètres de VisuGPS

Ce document détaille l'ensemble des paramètres configurables dans l'application VisuGPS. Ils sont accessibles depuis la vue "Paramètres".

## Accueil

### Paramètres Généraux

-   **Nombre de circuits par page** (`circuitsPerPage`): Nombre de circuits à afficher par page sur l'écran d'accueil.
    -   *Défaut*: 10
    -   [Documentation détaillée](docs/Accueil_circuitsPerPage.md)
-   **Taille de la vignette** (`TailleVignette`): Largeur en pixels de la vignette affichée au survol dans la liste des circuits.
    -   *Défaut*: 512
    -   [Documentation détaillée](docs/Accueil_TailleVignette.md)

### MajCommunes

#### Timers

-   **Timer IGN (ms)** (`timerIGN`): Délai en millisecondes entre chaque interrogation du géoportail de l'IGN.
    -   *Défaut*: 200
    -   [Documentation détaillée](docs/Accueil_MajCommunes_Timers_TimerIGN.md)
-   **Timer Mapbox (ms)** (`timerMapbox`): Délai en millisecondes entre chaque interrogation de l'API Mapbox.
    -   *Défaut*: 200
    -   [Documentation détaillée](docs/Accueil_MajCommunes_Timers_TimerMapbox.md)
-   **Timer OpenStreetMap (ms)** (`timerOSM`): Délai en millisecondes entre chaque interrogation de l'API OpenStreetMap.
    -   *Défaut*: 1000
    -   [Documentation détaillée](docs/Accueil_MajCommunes_Timers_TimerOSM.md)

#### APIs

-   **Activer l'API IGN par défaut** (`ignActif`): Si coché, les requêtes vers l'API de l'IGN seront activées.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/Accueil_MajCommunes_APIs_IgnActif.md)
-   **Activer l'API Mapbox par défaut** (`mapboxActif`): Si coché, les requêtes vers l'API Mapbox seront activées.
    -   *Défaut*: false
    -   [Documentation détaillée](docs/Accueil_MajCommunes_APIs_MapboxActif.md)

## Importation

### Paramètres Généraux

-   **Suppression après importation** (`autoDelete`): Suppression automatique du fichier \*.gpx quand il a été correctement importé.
    -   *Défaut*: false
    -   [Documentation détaillée](docs/Importation_autoDelete.md)
-   **Dossier des fichiers GPX** (`GPXFile`): Dossier pour l'importation des fichiers GPX. 'DEFAULT\_DOWNLOADS' utilise le dossier de téléchargement de l'utilisateur.
    -   *Défaut*: "DEFAULT\_DOWNLOADS"
    -   [Documentation détaillée](docs/Importation_GPXFile.md)
-   **Distance de lissage du dénivelé** (`denivele_lissage_distance`): Distance minimale (en mètres) entre deux points pour le calcul du dénivelé positif.
    -   *Défaut*: 10
    -   [Documentation détaillée](docs/Importation_denivele_lissage_distance.md)

### Vignette

-   **Style de la vignette** (`styleVignette`): Style de carte pour la création des vignettes.
    -   *Défaut*: "mapbox://styles/mapbox/streets-v12"

#### Dimensions

-   **Largeur de la vignette** (`largeur`): Largeur de la vignette générée en pixels.
    -   *Défaut*: 512
    -   [Documentation détaillée](docs/Importation_Vignette_Dimensions_largeur.md)
-   **Format de la vignette** (`format`): Format de la vignette (ratio largeur/hauteur).
    -   *Défaut*: "1/1"
    -   [Documentation détaillée](docs/Importation_Vignette_Dimensions_format.md)

#### Trace

-   **Couleur de la trace sur la vignette** (`colorGPXVignette`): Couleur de la trace GPX pour la création des vignettes.
    -   *Défaut*: "orange-darken-4"
    -   [Documentation détaillée](docs/Importation_Vignette_Trace_colorGPXVignette.md)
-   **Largeur de la trace** (`largeurTrace`): Largeur de la trace GPX sur la vignette en pixels.
    -   *Défaut*: 3
    -   [Documentation détaillée](docs/Importation_Vignette_Trace_largeurTrace.md)

#### MarqueurDistance

-   **Afficher la distance** (`presenceDistance`): Indique si la distance doit être affichée sur la trace.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/Importation_Vignette_MarqueurDistance_presenceDistance.md)
-   **Intervalle distance** (`Distance`): Intervalle en km pour l'affichage de la distance ou de la direction sur la trace.
    -   *Défaut*: 10
    -   [Documentation détaillée](docs/Importation_Vignette_MarqueurDistance_Distance.md)
-   **Couleur des marqueurs de distance** (`couleurPinDistance`): Couleur de base pour les marqueurs de distance numérotés.
    -   *Défaut*: "red"
    -   [Documentation détaillée](docs/Importation_Vignette_MarqueurDistance_couleurPinDistance.md)

#### DepartArrivee

-   **Afficher les marqueurs** (`Vignettes`): Indique si les marqueurs de départ et d'arrivée doivent être affichés.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/Importation_Vignette_DepartArrivee_Vignettes.md)
-   **Couleur du marqueur de départ** (`couleurDépart`): Couleur du marqueur de départ.
    -   *Défaut*: "green-darken-2"
    -   [Documentation détaillée](docs/Importation_Vignette_DepartArrivee_couleurDépart.md)
-   **Couleur du marqueur d'arrivée** (`couleurArrivée`): Couleur du marqueur d'arrivée.
    -   *Défaut*: "red-darken-2"
    -   [Documentation détaillée](docs/Importation_Vignette_DepartArrivee_couleurArrivée.md)
-   **Distance max départ/arrivée (m)** (`distanceMax`): Distance maximale en mètres entre le départ et l'arrivée pour les considérer comme proches.
    -   *Défaut*: 250
    -   [Documentation détaillée](docs/Importation_Vignette_DepartArrivee_distanceMax.md)
-   **Couleur du marqueur départ/arrivée (proches)** (`couleurDépartArrivée`): Couleur du marqueur de départ/arrivée si proches.
    -   *Défaut*: "blue-darken-2"
    -   [Documentation détaillée](docs/Importation_Vignette_DepartArrivee_couleurDépartArrivée.md)

### Tracking

-   **Longueur du segment** (`LongueurSegment`): Longueur d'un segment pour le tracking en mètres.
    -   *Défaut*: 100
    -   [Documentation détaillée](docs/Importation_Tracking_LongueurSegment.md)
-   **Lissage du cap** (`LissageCap`): Nombre de segments pour le lissage du cap.
    -   *Défaut*: 15
    -   [Documentation détaillée](docs/Importation_Tracking_LissageCap.md)

### Caméra

-   **Zoom** (`Zoom`): Niveau de zoom de la caméra par défaut pour le tracking.
    -   *Défaut*: 16
    -   [Documentation détaillée](docs/Importation_Camera_Zoom.md)
-   **Pitch** (`Pitch`): Angle de la caméra (pitch) par défaut pour le tracking.
    -   *Défaut*: 60
    -   [Documentation détaillée](docs/Importation_Camera_Pitch.md)

### QRCode

-   **Taille du QR code** (`taille`): Taille en pixels du côté du QR code généré.
    -   *Défaut*: 512
    -   [Documentation détaillée](docs/Importation_QRCode_taille.md)

## Edition

### Caméra

-   **Zoom** (`Zoom`): Niveau de zoom de la caméra par défaut.
    -   *Défaut*: 16.0
    -   [Documentation détaillée](docs/parametre_zoom.md)
-   **Activer le zoom au départ** (`zoomDepart`): Active un dézoom automatique au début du parcours.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/Edition_Camera_zoomDepart.md)
-   **Valeur du zoom au départ** (`zoomDepartValeur`): Niveau de zoom appliqué au point de départ (km 0).
    -   *Défaut*: 18
    -   [Documentation détaillée](docs/Edition_Camera_zoomDepartValeur.md)
-   **Distance du zoom au départ (x100m)** (`zoomDepartDistance`): Distance sur laquelle le dézoom s'applique.
    -   *Défaut*: 20
    -   [Documentation détaillée](docs/Edition_Camera_zoomDepartDistance.md)
-   **Activer le zoom à l'arrivée** (`zoomArrivee`): Active un zoom automatique à l'approche de l'arrivée.
    -   *Défaut*: false
    -   [Documentation détaillée](docs/Edition_Camera_zoomArrivee.md)
-   **Valeur du zoom à l'arrivée** (`zoomArriveeValeur`): Niveau de zoom appliqué au point d'arrivée.
    -   *Défaut*: 18
    -   [Documentation détaillée](docs/Edition_Camera_zoomArriveeValeur.md)
-   **Distance du zoom à l'arrivée (x100m)** (`distanceZoomArrivee`): Distance avant l'arrivée sur laquelle le zoom s'applique.
    -   *Défaut*: 20
    -   [Documentation détaillée](docs/Edition_Camera_distanceZoomArrivee.md)

### Graphe

#### Affichage courbes

-   **Afficher Delta Bearing (Calculé)** (`afficherBearingDeltaCalcule`): Affiche la courbe du delta de cap calculé.
    -   *Défaut*: false
    -   [Documentation détaillée](docs/Edition_Graphe_AffichageCourbes_afficherBearingDeltaCalcule.md)
-   **Afficher Delta Bearing (Édité)** (`afficherBearingDeltaEdite`): Affiche la courbe du delta de cap édité.
    -   *Défaut*: false
    -   [Documentation détaillée](docs/Edition_Graphe_AffichageCourbes_afficherBearingDeltaEdite.md)
-   **Afficher Somme Delta Bearing (Calculé)** (`afficherBearingTotalDeltaCalcule`): Affiche la courbe de la somme des deltas de cap calculés.
    -   *Défaut*: false
    -   [Documentation détaillée](docs/Edition_Graphe_AffichageCourbes_afficherBearingTotalDeltaCalcule.md)
-   **Afficher Somme Delta Bearing (Édité)** (`afficherBearingTotalDeltaEdite`): Affiche la courbe de la somme des deltas de cap édités.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/Edition_Graphe_AffichageCourbes_afficherBearingTotalDeltaEdite.md)
-   **Afficher Zoom (Édité)** (`afficherZoomEdite`): Affiche la courbe du zoom édité.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/Edition_Graphe_AffichageCourbes_afficherZoomEdite.md)
-   **Afficher Pitch (Édité)** (`afficherPitchEdite`): Affiche la courbe du pitch édité.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/Edition_Graphe_AffichageCourbes_afficherPitchEdite.md)

#### Couleur courbes

-   ... (Liste complète des couleurs de courbes) ...

### Evenements

#### Flyto

-   **Durée du survol (ms)** (`duree`): Durée par défaut de l'animation de survol (flyto).
    -   *Défaut*: 2000
    -   [Documentation détaillée](docs/Edition_Evenements_Flyto_duree.md)

#### Message

-   ... (Liste complète des paramètres de message) ...

### Commandes clavier

-   ... (Liste complète des commandes clavier) ...

### Commandes souris

-   ... (Liste complète des commandes souris) ...

### Mapbox

-   **Style de la carte** (`styleVisualisation`): Style de carte lors de l'édition.
    -   *Défaut*: "mapbox://styles/mapbox/satellite-v9"
    -   [Documentation détaillée](docs/Edition_Mapbox_styleVisualisation.md)

## Visualisation

### Mapbox

-   **Style de la carte** (`styleVisualisation`): Style de carte lors de la visualisation.
    -   *Défaut*: "mapbox://styles/mapbox/satellite-v9"
    -   [Documentation détaillée](docs/Visualisation_Mapbox_styleVisualisation.md)

#### Traces

-   **Colorer la trace selon la pente** (`colorerSelonPente`): Si activé, la couleur de la trace représente la pente.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/Visualisation_Mapbox_Traces_colorerSelonPente.md)
-   ... (et autres paramètres de trace) ...

### Animation

-   **Vitesse de l'animation (ms/km)** (`vitesse`): Nombre de millisecondes pour parcourir 1 kilomètre.
    -   *Défaut*: 3730
    -   [Documentation détaillée](docs/Visualisation_Animation_vitesse.md)
-   ... (et autres paramètres d'animation) ...

### Initialisation

-   ... (Paramètres de la séquence de démarrage de l'animation) ...

### Finalisation

-   ... (Paramètres de la séquence de fin de l'animation) ...

## Altitude

### Visualisation

-   **Afficher le profil** (`Affichage`): Affiche ou masque le profil altimétrique.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/Altitude_Visualisation_Affichage.md)
-   ... (et autres paramètres de visualisation du profil) ...

### Couleurs

-   **Pente <= 0%** (`TrancheNegative`): Couleur pour les pentes négatives ou nulles.
    -   *Défaut*: "light-blue"
    -   [Documentation détaillée](docs/Altitude_Couleurs_TrancheNegative.md)
-   ... (et autres tranches de couleur de pente) ...

## Système

### Timers

-   **Interval de vérification du réseau** (`networkPolling`): Intervalle de temps entre chaque vérification de l'état du réseau.
    -   *Défaut*: 30000
    -   [Documentation détaillée](docs/polling_timer_documentation.md)

### Tokens

-   **Token Mapbox** (`mapbox`): Token d'accès pour les services Mapbox.
    -   *Défaut*: ""
    -   [Documentation détaillée](docs/obtenir_token_mapbox.md)

### Télécommande

-   **Port du serveur de télécommande** (`Port`): Port utilisé par le serveur WebSocket pour la télécommande.
    -   *Défaut*: 9001
    -   [Documentation détaillée](docs/Systeme_Telecommande_Port.md)
-   ... (et autres paramètres de sensibilité de la télécommande) ...
