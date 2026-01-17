# Annexe des Paramètres de VisuGPS

Ce document détaille l'ensemble des paramètres configurables dans l'application VisuGPS. Ils sont accessibles depuis la vue "Paramètres".

## Accueil

-   **Nombre de circuits par page** (`circuitsPerPage`): Nombre de circuits à afficher par page sur l'écran d'accueil.
    -   *Défaut*: 10
    -   [Documentation détaillée](docs/DocParametrage/Accueil/circuitsPerPage.md)
-   **Taille de la vignette** (`TailleVignette`): Largeur en pixels de la vignette affichée au survol dans la liste des circuits.
    -   *Défaut*: 512
    -   [Documentation détaillée](docs/DocParametrage/Accueil/TailleVignette.md)

### MajCommunes

#### Timers

-   **Timer IGN (ms)** (`timerIGN`): Délai en millisecondes entre chaque interrogation du géoportail de l'IGN.
    -   *Défaut*: 200
    -   [Documentation détaillée](docs/DocParametrage/Accueil/MajCommunes/Timers/TimerIGN.md)
-   **Timer Mapbox (ms)** (`timerMapbox`): Délai en millisecondes entre chaque interrogation de l'API Mapbox.
    -   *Défaut*: 200
    -   [Documentation détaillée](docs/DocParametrage/Accueil/MajCommunes/Timers/TimerMapbox.md)
-   **Timer OpenStreetMap (ms)** (`timerOSM`): Délai en millisecondes entre chaque interrogation de l'API OpenStreetMap.
    -   *Défaut*: 1000
    -   [Documentation détaillée](docs/DocParametrage/Accueil/MajCommunes/Timers/TimerOSM.md)

#### APIs

-   **Activer l'API IGN par défaut** (`ignActif`): Si coché, les requêtes vers l'API de l'IGN seront activeés.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Accueil/MajCommunes/APIs/IgnActif.md)
-   **Activer l'API Mapbox par défaut** (`mapboxActif`): Si coché, les requêtes vers l'API Mapbox seront activées.
    -   *Défaut*: false
    -   [Documentation détaillée](docs/DocParametrage/Accueil/MajCommunes/APIs/MapboxActif.md)

## Importation

-   **Suppression après importation** (`autoDelete`): Suppression automatique du fichier *.gpx quand il a été corerectement importé.
    -   *Défaut*: false
    -   [Documentation détaillée](docs/DocParametrage/Importation/autoDelete.md)
-   **Dossier d'import** (`ImportDir`): Dossier pour l'importation des fichiers GPX. 'DEFAULT_DOWNLOADS' utilise le dossier de téléchargement de l'utilisateur.
    -   *Défaut*: "DEFAULT_DOWNLOADS"
    -   [Documentation détaillée](docs/DocParametrage/Importation/ImportDir.md)
-   **Distance de lissage du dénivelé** (`denivele_lissage_distance`): Distance minimale (en mètres) entre deux points pour le calcul du dénivelé positif. Permet de lisser les imprécisions de l'altitude.
    -   *Défaut*: 10
    -   [Documentation détaillée](docs/DocParametrage/Importation/denivele_lissage_distance.md)
-   **Filtre Médian Altitude** (`altitude_smoothing_median_window`): Taille de la fenêtre glissante pour le filtre médian de l'altitude (nombre de points). Permet de supprimer les pics aberrants.
    -   *Défaut*: 20
    -   [Documentation détaillée](docs/DocParametrage/Importation/altitude_smoothing_median_window.md)
-   **Lissage Moyenne Altitude** (`altitude_smoothing_avg_window`): Taille de la fenêtre glissante pour le lissage par moyenne de l'altitude (nombre de points).
    -   *Défaut*: 20
    -   [Documentation détaillée](docs/DocParametrage/Importation/altitude_smoothing_avg_window.md)
-   **Pente Max Autorisée (%)** (`max_gradient_percent`): Pente maximale autorisée en pourcentage. Les segments dépassant cette pente seront considérés comme suspects.
    -   *Défaut*: 20
    -   [Documentation détaillée](docs/DocParametrage/Importation/max_gradient_percent.md)

### Vignette

-   **Style de la vignette** (`styleVignette`): Style de carte pour la création des vignettes
    -   *Défaut*: "mapbox://styles/mapbox/streets-v12"
    -   [Documentation détaillée](docs/DocParametrage/Importation/Vignette/styleVignette.md)

#### Dimensions

-   **Largeur de la vignette** (`largeur`): Largeur de la vignette générée en pixels.
    -   *Défaut*: 512
    -   [Documentation détaillée](docs/DocParametrage/Importation/Vignette/Dimensions/largeur.md)
-   **Format de la vignette** (`format`): Format de la vignette (ratio largeur/hauteur).
    -   *Défaut*: "1/1"
    -   [Documentation détaillée](docs/DocParametrage/Importation/Vignette/Dimensions/format.md)

#### Trace

-   **Couleur de la trace sur la vignette** (`colorGPXVignette`): Couleur de la trace GPX pour la création des vignettes
    -   *Défaut*: "orange-darken-4"
    -   [Documentation détaillée](docs/DocParametrage/Importation/Vignette/Trace/colorGPXVignette.md)
-   **Largeur de la trace** (`largeurTrace`): Largeur de la trace GPX sur la vignette en pixels.
    -   *Défaut*: 3
    -   [Documentation détaillée](docs/DocParametrage/Importation/Vignette/Trace/largeurTrace.md)

#### MarqueurDistance

-   **Afficher la distance** (`presenceDistance`): Indique si la distance doit être affichée sur la trace.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Importation/Vignette/MarqueurDistance/presenceDistance.md)
-   **Intervalle distance** (`Distance`): Intervalle en km pour l'affichage de la distance ou de la direction sur la trace.
    -   *Défaut*: 10
    -   [Documentation détaillée](docs/DocParametrage/Importation/Vignette/MarqueurDistance/Distance.md)
-   **Couleur des marqueurs de distance** (`couleurPinDistance`): Couleur de base pour les marqueurs de distance numérotés.
    -   *Défaut*: "red"
    -   [Documentation détaillée](docs/DocParametrage/Importation/Vignette/MarqueurDistance/couleurPinDistance.md)

#### DepartArrivee

-   **Afficher les marqueurs** (`Vignettes`): Indique si les marqueurs de départ et d'arrivée doivent être affichés.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Importation/Vignette/DepartArrivee/Vignettes.md)
-   **Couleur du marqueur de départ** (`couleurDépart`): Couleur du marqueur de départ.
    -   *Défaut*: "green-darken-2"
    -   [Documentation détaillée](docs/DocParametrage/Importation/Vignette/DepartArrivee/couleurDepart.md)
-   **Couleur du marqueur d'arrivée** (`couleurArrivée`): Couleur du marqueur d'arrivée.
    -   *Défaut*: "red-darken-2"
    -   [Documentation détaillée](docs/DocParametrage/Importation/Vignette/DepartArrivee/couleurArrivee.md)
-   **Distance max départ/arrivée (m)** (`distanceMax`): Distance maximale en mètres entre le départ et l'arrivée pour les considérer comme proches.
    -   *Défaut*: 250
    -   [Documentation détaillée](docs/DocParametrage/Importation/Vignette/DepartArrivee/distanceMax.md)
-   **Couleur du marqueur départ/arrivée (proches)** (`couleurDépartArrivée`): Couleur du marqueur de départ/arrivée si proches.
    -   *Défaut*: "blue-darken-2"
    -   [Documentation détaillée](docs/DocParametrage/Importation/Vignette/DepartArrivee/couleurDepartArrivee.md)

### Tracking

-   **Longueur du segment** (`LongueurSegment`): Longueur d'un segment pour le tracking en mètres.
    -   *Défaut*: 100
    -   [Documentation détaillée](docs/DocParametrage/Importation/Tracking/LongueurSegment.md)
-   **Lissage du cap** (`LissageCap`): Nombre de segments pour le lissage du cap.
    -   *Défaut*: 15
    -   [Documentation détaillée](docs/DocParametrage/Importation/Tracking/LissageCap.md)
-   **Seuil de détection des superpositions** (`seuilDetectionSuperposition`): Distance maximale en mètres pour considérer deux segments comme superposés lors de la détection des allers-retours.
    -   *Défaut*: 20
    -   [Documentation détaillée](docs/DocParametrage/Importation/Tracking/seuilDetectionSuperposition.md)

### Caméra

-   **Zoom** (`Zoom`): Niveau de zoom de la caméra par défaut pour le tracking.
    -   *Défaut*: 16
    -   [Documentation détaillée](docs/DocParametrage/Importation/Camera/Zoom.md)
-   **Pitch** (`Pitch`): Angle de la caméra (pitch) par défaut pour le tracking.
    -   *Défaut*: 60
    -   [Documentation détaillée](docs/DocParametrage/Importation/Camera/Pitch.md)

### QRCode

-   **Taille du QR code** (`taille`): Taille en pixels du côté du QR code généré.
    -   *Défaut*: 512
    -   [Documentation détaillée](docs/DocParametrage/Importation/QRCode/taille.md)

### Label Départ Arrivée

-   **Afficher le départ** (`afficherDepart`): Indique si un message de départ doit être ajouté automatiquement au début du circuit.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Importation/LabelDepartArrivee/afficherDepart.md)
-   **Message départ** (`messageDepart`): Identifiant du message à afficher au départ.
    -   *Défaut*: "_Départ_green"
    -   [Documentation détaillée](docs/DocParametrage/Importation/LabelDepartArrivee/messageDepart.md)
-   **Orientation départ à droite** (`orientationDepartDroite`): Orientation du message de départ (true pour Droite, false pour Gauche).
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Importation/LabelDepartArrivee/orientationDepartDroite.md)
-   **Post affichage départ** (`postAffichageDepart`): Durée d'affichage après le départ (en incréments).
    -   *Défaut*: 10
    -   [Documentation détaillée](docs/DocParametrage/Importation/LabelDepartArrivee/postAffichageDepart.md)
-   **Afficher l'arrivée** (`afficherArrivee`): Indique si un message d'arrivée doit être ajouté automatiquement à la fin du circuit.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Importation/LabelDepartArrivee/afficherArrivee.md)
-   **Message arrivée** (`messageArrivee`): Identifiant du message à afficher à l'arrivée.
    -   *Défaut*: "_Arrivée_red"
    -   [Documentation détaillée](docs/DocParametrage/Importation/LabelDepartArrivee/messageArrivee.md)
-   **Orientation arrivée à droite** (`orientationArriveeDroite`): Orientation du message d'arrivée (true pour Droite, false pour Gauche).
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Importation/LabelDepartArrivee/orientationArriveeDroite.md)
-   **Pré affichage arrivée** (`preAffichageArrivee`): Durée d'affichage avant l'arrivée (en incréments).
    -   *Défaut*: 10
    -   [Documentation détaillée](docs/DocParametrage/Importation/LabelDepartArrivee/preAffichageArrivee.md)

## Edition

### Vue 3D

#### Trace

-   **Épaisseur de l'avancement** (`epaisseurAvancement`): Épaisseur de la ligne d'avancement de la caméra en pixels.
    -   *Défaut*: 5
    -   [Documentation détaillée](docs/DocParametrage/Edition/Vue 3D/Trace/epaisseurAvancement.md)
-   **Couleur de l'avancement** (`couleurAvancement`): Couleur de la ligne d'avancement de la caméra.
    -   *Défaut*: "yellow"
    -   [Documentation détaillée](docs/DocParametrage/Edition/Vue 3D/Trace/couleurAvancement.md)
-   **Épaisseur de la trace** (`epaisseur`): Épaisseur de la trace GPX pour l'édition en pixels.
    -   *Défaut*: 3
    -   [Documentation détaillée](docs/DocParametrage/Edition/Vue 3D/Trace/epaisseur.md)
-   **Couleur de la trace** (`couleur`): Couleur de la trace GPX pour l'édition (si coloration par pente désactivée).
    -   *Défaut*: "orange-darken-4"
    -   [Documentation détaillée](docs/DocParametrage/Edition/Vue 3D/Trace/couleur.md)
-   **Colorer la trace selon la pente** (`colorerSelonPente`): Si activé, la couleur de la trace représente la pente. Sinon, une couleur unique est utilisée.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Edition/Vue 3D/Trace/colorerSelonPente.md)

#### Carte

-   **Style de la carte** (`styleVisualisation`): Style de carte lors de l'édition
    -   *Défaut*: "mapbox://styles/mapbox/satellite-v9"
    -   [Documentation détaillée](docs/DocParametrage/Edition/Vue 3D/Carte/styleVisualisation.md)
-   **Coefficient de relief** (`exaggeration`): Facteur d'exagération du relief 3D. 1.0 pour un relief réel, >1.0 pour l'exagérer.
    -   *Défaut*: 1
    -   [Documentation détaillée](docs/DocParametrage/Edition/Vue 3D/Carte/exaggeration.md)

### Avancement dans les graphes

-   **Couleur de la zone d'avancement** (`couleurAvancementZone`): Couleur de la zone rectangulaire indiquant l'avancement de la caméra sur le graphe.
    -   *Défaut*: "yellow"
    -   [Documentation détaillée](docs/DocParametrage/Edition/Avancement dans les graphes/couleurAvancementZone.md)
-   **Opacité de la zone d'avancement** (`opaciteAvancementZone`): Opacité de la zone rectangulaire indiquant l'avancement de la caméra sur le graphe (0.0 à 1.0).
    -   *Défaut*: 0.1
    -   [Documentation détaillée](docs/DocParametrage/Edition/Avancement dans les graphes/opaciteAvancementZone.md)

### Camera

-   **Zoom** (`Zoom`): Niveau de zoom de la caméra par défaut.
    -   *Défaut*: 16
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/Zoom.md)

#### ZoomDépart

-   **Activer zoom départ** (`zoomDepart`): Active un dézoom automatique au début du parcours.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/ZoomDépart/zoomDepart.md)
-   **Valeur du zoom départ** (`zoomDepartValeur`): Niveau de zoom appliqué au point de départ (km 0).
    -   *Défaut*: 18
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/ZoomDépart/zoomDepartValeur.md)
-   **Distance du zoom départ** (`zoomDepartDistance`): Distance sur laquelle le dézoom s'applique, en segments de 100m.
    -   *Défaut*: 20
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/ZoomDépart/zoomDepartDistance.md)

#### ZoomArrivée

-   **Activer zoom arrivée** (`zoomArrivee`): Active un zoom automatique à l'approche de l'arrivée.
    -   *Défaut*: false
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/ZoomArrivée/zoomArrivee.md)
-   **Valeur du zoom arrivée** (`zoomArriveeValeur`): Niveau de zoom appliqué au point d'arrivée.
    -   *Défaut*: 18
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/ZoomArrivée/zoomArriveeValeur.md)
-   **Distance du zoom arrivée** (`distanceZoomArrivee`): Distance avant l'arrivée sur laquelle le zoom s'applique, en segments de 100m.
    -   *Défaut*: 20
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/ZoomArrivée/distanceZoomArrivee.md)

#### Graphe caméra

-   **undefined** (`couleurPointDeControle`): Couleur des indicateurs de points de contrôle sur le graphe.
    -   *Défaut*: "purple"
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/Graphe caméra/couleurPointDeControle.md)
-   **Epaisseur Point de Contrôle** (`epaisseurPointDeControle`): Epaisseur en pixels des indicateurs de points de contrôle.
    -   *Défaut*: 3
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/Graphe caméra/epaisseurPointDeControle.md)
-   **Longueur Point de Contrôle** (`longueurPointDeControle`): Longueur en pixels des indicateurs de points de contrôle sur le graphe.
    -   *Défaut*: 20
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/Graphe caméra/longueurPointDeControle.md)

##### Affichage courbes

-   **Afficher Delta Bearing (Calculé)** (`afficherBearingDeltaCalcule`): Affiche la courbe du delta de cap calculé (valeurs originales).
    -   *Défaut*: false
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/Graphe caméra/AffichageCourbes/afficherBearingDeltaCalcule.md)
-   **Afficher Delta Bearing (Édité)** (`afficherBearingDeltaEdite`): Affiche la courbe du delta de cap édité.
    -   *Défaut*: false
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/Graphe caméra/AffichageCourbes/afficherBearingDeltaEdite.md)
-   **Afficher Somme Delta Bearing (Calculé)** (`afficherBearingTotalDeltaCalcule`): Affiche la courbe de la somme des deltas de cap calculés.
    -   *Défaut*: false
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/Graphe caméra/AffichageCourbes/afficherBearingTotalDeltaCalcule.md)
-   **Afficher Somme Delta Bearing (Édité)** (`afficherBearingTotalDeltaEdite`): Affiche la courbe de la somme des deltas de cap édités.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/Graphe caméra/AffichageCourbes/afficherBearingTotalDeltaEdite.md)
-   **Afficher Zoom (Édité)** (`afficherZoomEdite`): Affiche la courbe du zoom édité.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/Graphe caméra/AffichageCourbes/afficherZoomEdite.md)
-   **Afficher Pitch (Édité)** (`afficherPitchEdite`): Affiche la courbe du pitch édité.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/Graphe caméra/AffichageCourbes/afficherPitchEdite.md)

##### Couleur courbes

-   **Couleur du Zoom** (`couleurZoom`): Couleur de la courbe de zoom sur le graphe.
    -   *Défaut*: "green"
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/Graphe caméra/CouleurCourbes/couleurZoom.md)
-   **Couleur Zoom Edité** (`couleurEditedZoom`): Couleur de la courbe du zoom édité sur le graphe.
    -   *Défaut*: "green-darken-4"
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/Graphe caméra/CouleurCourbes/couleurEditedZoom.md)
-   **Couleur du Pitch** (`couleurPitch`): Couleur de la courbe de pitch sur le graphe.
    -   *Défaut*: "light-blue"
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/Graphe caméra/CouleurCourbes/couleurPitch.md)
-   **Couleur Pitch Edité** (`couleurEditedPitch`): Couleur de la courbe du pitch édité sur le graphe.
    -   *Défaut*: "light-blue-darken-4"
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/Graphe caméra/CouleurCourbes/couleurEditedPitch.md)
-   **Couleur du Delta Bearing** (`couleurBearingDelta`): Couleur de la courbe du delta de cap sur le graphe.
    -   *Défaut*: "amber"
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/Graphe caméra/CouleurCourbes/couleurBearingDelta.md)
-   **Couleur Delta Bearing Edité** (`couleurEditedBearingDelta`): Couleur de la courbe du delta de cap édité sur le graphe.
    -   *Défaut*: "amber-darken-3"
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/Graphe caméra/CouleurCourbes/couleurEditedBearingDelta.md)
-   **Couleur du Somme Delta Bearing** (`couleurBearingTotalDelta`): Couleur de la courbe du delta de cap total sur le graphe.
    -   *Défaut*: "red"
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/Graphe caméra/CouleurCourbes/couleurBearingTotalDelta.md)
-   **Couleur Somme Delta Bearing Edité** (`couleurEditedBearingTotalDelta`): Couleur de la courbe du delta de cap total édité sur le graphe.
    -   *Défaut*: "red-darken-4"
    -   [Documentation détaillée](docs/DocParametrage/Edition/Camera/Graphe caméra/CouleurCourbes/couleurEditedBearingTotalDelta.md)

### Pause et Survol

-   **Durée du survol (sec)** (`duree`): Durée par défaut de l'animation de survol en secondes.
    -   *Défaut*: 2
    -   [Documentation détaillée](docs/DocParametrage/Edition/Pause_Flyto/dureeSurvol.md)
-   **Couleur de la croix centrale** (`couleurCroixCentraleEdition`): Définit la couleur de la croix centrale affichée en mode pause ou survol en édition.
    -   *Défaut*: "white"
    -   [Documentation détaillée](docs/DocParametrage/Edition/Pause_Flyto/couleurCroixCentraleEdition.md)

#### Graphe Pause et Survol

-   **Couleur des pauses** (`couleurPause`): Couleur des marqueurs de pause sur le graphe.
    -   *Défaut*: "purple"
    -   [Documentation détaillée](docs/DocParametrage/Edition/Pause_Flyto/Graphe Pause_Flyto/couleurPause.md)
-   **Couleur des survols** (`couleurFlyTo`): Couleur des marqueurs de survol sur le graphe.
    -   *Défaut*: "orange-darken-1"
    -   [Documentation détaillée](docs/DocParametrage/Edition/Pause_Flyto/Graphe Pause_Flyto/couleurFlyto.md)
-   **Longueur** (`longueur`): Longueur des marqueurs de pause et survol sur le graphe.
    -   *Défaut*: 12
    -   [Documentation détaillée](docs/DocParametrage/Edition/Pause_Flyto/Graphe Pause_Flyto/longueur.md)

### Messages

-   **Pré-affichage** (`preAffichage`): Nombre d'incréments avant le point de contrôle où le message commence à s'afficher.
    -   *Défaut*: 10
    -   [Documentation détaillée](docs/DocParametrage/Edition/Messages/preAffichage.md)
-   **Post-affichage** (`postAffichage`): Nombre d'incréments après le point de contrôle où le message cesse de s'afficher.
    -   *Défaut*: 20
    -   [Documentation détaillée](docs/DocParametrage/Edition/Messages/postAffichage.md)
-   **Durée de transition du Pitch** (`transitionDuree`): Durée de la transition du pitch de la caméra lors du déplacement de la carte avec la souris.
    -   *Défaut*: 50
    -   [Documentation détaillée](docs/DocParametrage/Edition/Messages/transitionDuree.md)
-   **URL Emoticon** (`urlEmoticon`): URL vers la page d'aide des emojis (ex: cheat sheet).
    -   *Défaut*: "https://github.com/ikatyang/emoji-cheat-sheet"
    -   [Documentation détaillée](docs/DocParametrage/Edition/Messages/urlEmoticon.md)

#### Distance

-   **Afficher les distances** (`ajouter`): Si activé, les bornes kilométriques sont automatiquement ajoutées pour chaque nouvelle importation de trace.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Edition/Messages/Distance/afficherDistances.md)
-   **Orientation à gauche** (`orientation`): Si activé, les messages s'affichent à gauche, sinon à droite.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Edition/Messages/Distance/orientationAGauche.md)
-   **Couleur des messages** (`couleur`): Couleur Material Design des messages de distance.
    -   *Défaut*: "red"
    -   [Documentation détaillée](docs/DocParametrage/Edition/Messages/Distance/couleurMessages.md)
-   **Intervalle** (`intervalle`): Intervalle en kilomètres entre chaque borne kilométrique affichée.
    -   *Défaut*: 10
    -   [Documentation détaillée](docs/DocParametrage/Edition/Messages/Distance/intervalle.md)
-   **Pre-affichage** (`preAffichage`): Nombre d'incréments avant la borne où le message commence à s'afficher.
    -   *Défaut*: 10
    -   [Documentation détaillée](docs/DocParametrage/Edition/Messages/Distance/preAffichage.md)
-   **Post-affichage** (`postAffichage`): Nombre d'incréments après la borne où le message cesse de s'afficher.
    -   *Défaut*: 10
    -   [Documentation détaillée](docs/DocParametrage/Edition/Messages/Distance/postAffichage.md)

#### Graphe messages

-   **Hauteur des messages** (`hauteurGraphique`): Hauteur des rectangles représentant les messages sur le graphe des événements.
    -   *Défaut*: 10
    -   [Documentation détaillée](docs/DocParametrage/Edition/Messages/Graphe messages/HauteurMessages.md)

### Commandes clavier

-   **Incrément Avancement** (`incrementAvancement`): Incrément de déplacement pour les flèches gauche/droite (points de tracking).
    -   *Défaut*: 1
    -   [Documentation détaillée](docs/DocParametrage/Edition/CommandesClavier/incrementAvancement.md)
-   **Incrément Avancement (Shift)** (`incrementAvancementShift`): Incrément de déplacement avec la touche Shift pour les flèches gauche/droite (points de tracking).
    -   *Défaut*: 10
    -   [Documentation détaillée](docs/DocParametrage/Edition/CommandesClavier/incrementAvancementShift.md)
-   **Incrément Pitch** (`incrementPitch`): Incrément pour l'angle de la caméra (pitch) avec les flèches haut/bas.
    -   *Défaut*: 1
    -   [Documentation détaillée](docs/DocParametrage/Edition/CommandesClavier/incrementPitch.md)
-   **Incrément Pitch (Shift)** (`incrementPitchShift`): Incrément pour l'angle de la caméra (pitch) avec les flèches haut/bas et la touche Shift.
    -   *Défaut*: 5
    -   [Documentation détaillée](docs/DocParametrage/Edition/CommandesClavier/incrementPitchShift.md)
-   **Touche Avancement Avant** (`toucheAvancementAvant`): Touche pour avancer (flèche droite).
    -   *Défaut*: "ArrowRight"
    -   [Documentation détaillée](docs/DocParametrage/Edition/CommandesClavier/toucheAvancementAvant.md)
-   **Touche Avancement Arrière** (`toucheAvancementArriere`): Touche pour reculer (flèche gauche).
    -   *Défaut*: "ArrowLeft"
    -   [Documentation détaillée](docs/DocParametrage/Edition/CommandesClavier/toucheAvancementArriere.md)
-   **Touche Pitch Haut** (`touchePitchHaut`): Touche pour augmenter le pitch (flèche haut).
    -   *Défaut*: "ArrowUp"
    -   [Documentation détaillée](docs/DocParametrage/Edition/CommandesClavier/touchePitchHaut.md)
-   **Touche Pitch Bas** (`touchePitchBas`): Touche pour diminuer le pitch (flèche bas).
    -   *Défaut*: "ArrowDown"
    -   [Documentation détaillée](docs/DocParametrage/Edition/CommandesClavier/touchePitchBas.md)

### Commandes souris

-   **Incrément Zoom** (`incrementZoom`): Incrément de zoom de la caméra avec la molette de la souris.
    -   *Défaut*: 0.1
    -   [Documentation détaillée](docs/DocParametrage/Edition/CommandesSouris/incrementZoom.md)
-   **Incrément Zoom (Shift)** (`incrementZoomShift`): Incrément de zoom de la caméra avec la molette de la souris et la touche Shift.
    -   *Défaut*: 1
    -   [Documentation détaillée](docs/DocParametrage/Edition/CommandesSouris/incrementZoomShift.md)
-   **Incrément Cap** (`incrementBearing`): Incrément de cap de la caméra avec la molette de la souris.
    -   *Défaut*: 1
    -   [Documentation détaillée](docs/DocParametrage/Edition/CommandesSouris/incrementBearing.md)
-   **Incrément Cap (Shift)** (`incrementBearingShift`): Incrément de cap de la caméra avec la molette de la souris et la touche Shift.
    -   *Défaut*: 5
    -   [Documentation détaillée](docs/DocParametrage/Edition/CommandesSouris/incrementBearingShift.md)

## Variante

-   **Couleur trace maîtresse** (`originalColor`): Couleur de la trace originale lors de l'édition d'une variante.
    -   *Défaut*: "#BDBDBD"
    -   [Documentation détaillée](docs/DocParametrage/Variante/originalColor.md)
-   **Couleur trace aperçu** (`previewColor`): Couleur du segment en cours de création.
    -   *Défaut*: "#651FFF"
    -   [Documentation détaillée](docs/DocParametrage/Variante/previewColor.md)
-   **Couleur des nœuds** (`nodeColor`): Couleur des points d'ancrage affichés sur la trace maîtresse.
    -   *Défaut*: "#FFFFFF"
    -   [Documentation détaillée](docs/DocParametrage/Variante/nodeColor.md)
-   **Service de routage** (`routingService`): Service utilisé pour calculer l'itinéraire entre les points.
    -   *Défaut*: "GraphHopper"
    -   [Documentation détaillée](docs/DocParametrage/Variante/routingService.md)
-   **Clé API Routage** (`routingApiKey`): Clé API pour le service de routage (GraphHopper ou OpenRouteService).
    -   *Défaut*: ""
    -   [Documentation détaillée](docs/DocParametrage/Variante/routingApiKey.md)
-   **Profil de routage** (`routingType`): Profil de véhicule utilisé par défaut pour le calcul d'itinéraire.
    -   *Défaut*: "Route uniquement"
    -   [Documentation détaillée](docs/DocParametrage/Variante/routingType.md)
-   **Style de carte** (`mapStyle`): Style de carte Mapbox utilisé pour l'édition des variantes (Standard, Satellite, Outdoors).
    -   *Défaut*: "mapbox://styles/mapbox/standard"
    -   [Documentation détaillée](docs/DocParametrage/Variante/mapStyle.md)
-   **Zoom visibilité nœuds** (`zoomVisuNode`): Niveau de zoom minimum pour afficher les points d'ancrage sur la trace maîtresse.
    -   *Défaut*: 13
    -   [Documentation détaillée](docs/DocParametrage/Variante/zoomVisuNode.md)

### Edition

#### Trace

-   **Couleur de la trace** (`couleur`): Couleur de la trace de la variante (si coloration par pente désactivée).
    -   *Défaut*: "light-blue"
    -   [Documentation détaillée](docs/DocParametrage/Variante/Edition/Trace/couleur.md)
-   **Largeur de la trace** (`largeur`): Largeur de la trace de la variante en pixels.
    -   *Défaut*: 4
    -   [Documentation détaillée](docs/DocParametrage/Variante/Edition/Trace/largeur.md)
-   **Colorer selon la pente** (`colorerSelonPente`): Si activé, la trace de la variante est colorée selon la pente. Sinon, la couleur unie est utilisée.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Variante/Edition/Trace/colorerSelonPente.md)

#### Trace Maîtresse

-   **Couleur de fond** (`couleur`): Couleur de la trace principale affichée en arrière-plan lors de l'édition d'une variante.
    -   *Défaut*: "grey-darken-3"
    -   [Documentation détaillée](docs/DocParametrage/Variante/Edition/TraceMaitresse/couleur.md)
-   **Largeur de fond** (`largeur`): Largeur de la trace principale en arrière-plan (px).
    -   *Défaut*: 4
    -   [Documentation détaillée](docs/DocParametrage/Variante/Edition/TraceMaitresse/largeur.md)
-   **Opacité de fond** (`opacite`): Opacité de la trace principale en arrière-plan (0.0 - 1.0).
    -   *Défaut*: 0.3
    -   [Documentation détaillée](docs/DocParametrage/Variante/Edition/TraceMaitresse/opacite.md)

### Visualisation

-   **Durée du flyto vers le segment** (`dureeFlytoSegment`): Durée en secondes du déplacement de la caméra vers le premier point du segment sélectionné.
    -   *Défaut*: 2
    -   [Documentation détaillée](docs/DocParametrage/Variante/Visualisation/dureeFlytoSegment.md)

### Parametres

-   **Clé API GraphHopper** (`routingApiKey`): Clé API pour le service de routage GraphHopper.
    -   *Défaut*: ""
    -   [Documentation détaillée](docs/DocParametrage/Variante/Parametres/routingApiKey.md)

## Visualisation

### Vue 3D

#### Carte

-   **Style de la carte.** (`styleVisualisation`): Style de carte lors de la visualisation
    -   *Défaut*: "mapbox://styles/mapbox/satellite-v9"
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Vue 3D/Carte/styleVisualisation.md)

#### Trace

-   **Colorer la trace selon la pente** (`colorerSelonPente`): Si activé, la couleur de la trace représente la pente. Sinon, une couleur unique est utilisée.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Vue 3D/Trace/colorerSelonPente.md)
-   **Couleur de la trace** (`couleurTrace`): Couleur de la trace GPX sur la carte (utilisée si la coloration par pente est désactivée).
    -   *Défaut*: "orange"
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Vue 3D/Trace/couleurTrace.md)
-   **Epaisseur de la trace** (`epaisseurTrace`): Epaisseur de la trace GPX sur la carte.
    -   *Défaut*: 8
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Vue 3D/Trace/epaisseurTrace.md)
-   **Opacité de la trace** (`opaciteTrace`): Opacité de la trace GPX sur la carte.
    -   *Défaut*: 0.8
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Vue 3D/Trace/opaciteTrace.md)
-   **Couleur de la comète** (`couleurComete`): Couleur de la comète qui suit l'avancement.
    -   *Défaut*: "white"
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Vue 3D/Trace/couleurComete.md)
-   **Epaisseur de la comète** (`epaisseurComete`): Epaisseur de la comète.
    -   *Défaut*: 8
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Vue 3D/Trace/epaisseurComete.md)
-   **Opacité de la comète** (`opaciteComete`): Opacité de la comète.
    -   *Défaut*: 1
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Vue 3D/Trace/opaciteComete.md)
-   **Longueur de la comète (m)** (`longueurComete`): Longueur de la comète en mètres.
    -   *Défaut*: 50
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Vue 3D/Trace/longueurComete.md)

### Widgets

-   **Distance** (`distance`): Afficher le widget de distance au démarrage.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Widgets/distance.md)
-   **Communes** (`communes`): Afficher le widget des communes au démarrage.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Widgets/communes.md)
-   **Altitude** (`altitude`): Afficher le profil d'altitude au démarrage.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Widgets/altitude.md)
-   **Commandes** (`commandes`): Afficher le panneau de commandes au démarrage.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Widgets/commandes.md)

### Lancement

-   **Style de la carte au lancement** (`styleLancement`): Style de carte utilisé lors de la phase d'initialisation (vue Europe et vue globale).
    -   *Défaut*: "mapbox://styles/mapbox/standard"
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Lancement/styleLancement.md)
-   **Centre initial** (`centerEurope`): Coordonnées (longitude, latitude) du centre de l'Europe pour la vue initiale.
    -   *Défaut*: 2.3522,48.8566
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Lancement/centerEurope.md)
-   **Zoom initial** (`zoomEurope`): Niveau de zoom initial pour la vue de l'Europe.
    -   *Défaut*: 5
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Lancement/zoomEurope.md)
-   **Durée de l'animation vers la trace au départ** (`durationEuropeToTrace`): Durée en secondes de l'animation de survol de l'Europe vers la vue globale de la trace.
    -   *Défaut*: 5
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Lancement/durationEuropeToTrace.md)
-   **Durée de la pause sur la trace** (`pauseBeforeStart`): Durée en secondes de la pause entre la vue globale de la trace et le début de l'animation.
    -   *Défaut*: 1
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Lancement/pauseBeforeStart.md)
-   **Durée de l'animation vers le départ** (`durationTraceToStart`): Durée en secondes de l'animation de survol de la vue globale de la trace vers le point de départ (km 0).
    -   *Défaut*: 2
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Lancement/durationTraceToStart.md)
-   **Durée de la pause au Km 0** (`pauseAuKm0`): Durée de la pause au point de départ (Km 0) avant le démarrage automatique de l'animation. S'applique à l'initialisation et à la réinitialisation.
    -   *Défaut*: 0.5
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Lancement/pauseAuKm0.md)

### Lecture

-   **Vitesse de l'animation (ms/km)** (`vitesse`): Nombre de millisecondes pour parcourir 1 kilomètre.
    -   *Défaut*: 3730
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Lecture/vitesse.md)
-   **Durée de la reprise après pause (sec)** (`timerReprisePause`): Durée en secondes du survol pour revenir à la position de la caméra après une pause.
    -   *Défaut*: 1
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Lecture/timerReprisePause.md)
-   **Délai avant de masquer le curseur (ms)** (`masquerCurseurDelai`): Délai d'inactivité en millisecondes avant de masquer le curseur de la souris.
    -   *Défaut*: 1000
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Lecture/masquerCurseurDelai.md)
-   **Afficher la croix centrale en pause** (`afficherCroixCentrale`): Affiche une croix au centre de l'écran lorsque l'animation est en pause.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Lecture/afficherCroixCentrale.md)
-   **Couleur de la croix centrale** (`couleurCroixCentrale`): Définit la couleur de la croix centrale affichée en pause.
    -   *Défaut*: "white"
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Lecture/couleurCroixCentrale.md)
-   **Zoom Minimum** (`zoomMinimum`): Niveau de zoom minimum autorisé lorsque l'animation est en pause (valeur réelle = valeur / 10).
    -   *Défaut*: 100
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Lecture/zoomMinimum.md)

#### Vitesse

-   **Vitesse minimale (x)** (`min_value`): Multiplicateur de vitesse minimal pour l'animation.
    -   *Défaut*: 0.1
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Lecture/Vitesse/min_value.md)
-   **Vitesse maximale (x)** (`max_value`): Multiplicateur de vitesse maximal pour l'animation.
    -   *Défaut*: 20
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Lecture/Vitesse/max_value.md)
-   **Vitesse par défaut (x)** (`default_value`): Multiplicateur de vitesse par défaut au démarrage et à la réinitialisation.
    -   *Défaut*: 1
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Lecture/Vitesse/default_value.md)
-   **Pas du slider (x)** (`slider_step`): Incrément/décrément du slider de vitesse.
    -   *Défaut*: 0.05
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Lecture/Vitesse/slider_step.md)

#### ZoomDynamique

-   **Intensité du zoom dynamique** (`intensite_zoom_dynamique`): Définit l'intensité de l'effet de zoom dynamique basé sur la vitesse. Une valeur plus élevée zoome davantage à basse vitesse.
    -   *Défaut*: 10
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Lecture/ZoomDynamique/intensite_zoom_dynamique.md)

### Finalisation

-   **Durée de la pause à l'arrivée** (`delayAfterAnimationEnd`): Délai en secondes avant de lancer la séquence de finalisation après la fin de l'animation.
    -   *Défaut*: 3
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Finalisation/delayAfterAnimationEnd.md)
-   **Durée de l'animation vers la trace à l'arrivée** (`flyToGlobalDuration`): Durée en secondes de l'animation de survol vers la vue globale de la trace.
    -   *Défaut*: 2
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Finalisation/flyToGlobalDuration.md)
-   **Durée de l'animation vers le départ** (`flyToKm0Duration`): Durée en secondes de l'animation de survol vers le point de départ (Km 0).
    -   *Défaut*: 1
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Finalisation/flyToKm0Duration.md)
-   **Reprise automatique** (`repriseAutomatique`): Si activé, l'animation redémarre automatiquement après la séquence de finalisation (vue globale).
    -   *Défaut*: false
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Finalisation/repriseAutomatique.md)
-   **Durée de la pause avant reprise auto** (`pauseAvantReprise`): En mode de reprise automatique, durée de la pause sur la vue globale avant de relancer l'animation.
    -   *Défaut*: 1
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Finalisation/pauseAvantReprise.md)

### Profil Altitude

#### Graphe

-   **Echelle des abscisses** (`Abscisse`): Définit l'échelle de l'axe des abscisses (distance). Nombre de pixels pour 100 mètres.
    -   *Défaut*: 2
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Profil Altitude/Graphe/Abscisse.md)
-   **Echelle des ordonnées** (`Ordonnee`): Définit l'échelle de l'axe des ordonnées (altitude). Nombre de pixels pour 10 mètres de dénivelé.
    -   *Défaut*: 1
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Profil Altitude/Graphe/Ordonnee.md)
-   **Aspect curseur lié à la comète** (`aspectCurseurLieComete`): Si activé, le curseur d'avancement reprend l'aspect de la comète (couleur, opacité, longueur).
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Profil Altitude/Graphe/aspectCurseurLieComete.md)
-   **Intervalle des repères distances** (`RepereDistance`): Intervalle en kilomètres pour les repères sur l'axe des distances.
    -   *Défaut*: 10
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Profil Altitude/Graphe/RepereDistance.md)
-   **Intervalle des repères altitude** (`RepereAltitude`): Intervalle en mètres pour les repères sur l'axe des altitudes.
    -   *Défaut*: 500
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Profil Altitude/Graphe/RepereAltitude.md)
-   **Position du curseur.** (`CurseurPositionKm`): Définit la position en km où le curseur se bloque lorsque le graphique défile.
    -   *Défaut*: 10
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Profil Altitude/Graphe/CurseurPositionKm.md)

#### Couleurs

-   **Pente <= 0%** (`TrancheNegative`): Couleur pour les pentes négatives ou nulles.
    -   *Défaut*: "light-blue"
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Profil Altitude/Couleurs/TrancheNegative.md)
-   **0% < Pente < 3%** (`Tranche1`): Couleur pour les pentes de 0% à 3%.
    -   *Défaut*: "green"
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Profil Altitude/Couleurs/Tranche1.md)
-   **3% <= Pente < 6%** (`Tranche2`): Couleur pour les pentes de 3% à 6%.
    -   *Défaut*: "yellow"
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Profil Altitude/Couleurs/Tranche2.md)
-   **6% <= Pente < 9%** (`Tranche3`): Couleur pour les pentes de 6% à 9%.
    -   *Défaut*: "orange"
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Profil Altitude/Couleurs/Tranche3.md)
-   **9% <= Pente < 12%** (`Tranche4`): Couleur pour les pentes de 9% à 12%.
    -   *Défaut*: "red"
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Profil Altitude/Couleurs/Tranche4.md)
-   **Pente >= 12%** (`Tranche5`): Couleur pour les pentes supérieures à 12%.
    -   *Défaut*: "purple"
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Profil Altitude/Couleurs/Tranche5.md)

### Taille des Messages

-   **Taille de police de base des messages (px)** (`baseFontSize`): Taille de police de base utilisée pour les messages affichés sur la carte. Les autres dimensions du message s'adapteront proportionnellement.
    -   *Défaut*: 30
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Taille des Messages/baseFontSize.md)
-   **Coefficient largeur message** (`coefLargeurMessage`): Coefficient multiplicateur appliqué à la taille de police pour estimer la largeur du texte du message. Ajuster si le cadre est trop large ou trop étroit.
    -   *Défaut*: 0.7
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Taille des Messages/coefLargeurMessage.md)
-   **Coefficient réducteur message** (`coefReducteurMessage`): Réduit le coefficient largeur pour les messages longs. Formule : Coef / (1 + Longueur * Reducteur).
    -   *Défaut*: 0.025
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Taille des Messages/coefReducteurMessage.md)

### Météo

-   **Activer la météo** (`meteoActif`): Active la récupération et l'affichage des données météo.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Meteo/meteoActif.md)
-   **Heure de début de journée** (`heureDebutJournee`): Heure de début pour la récupération des données météo (0-23).
    -   *Défaut*: 6
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Meteo/heureDebutJournee.md)
-   **Heure de fin de journée** (`heureFinJournee`): Heure de fin pour la récupération des données météo (0-23).
    -   *Défaut*: 20
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Meteo/heureFinJournee.md)
-   **Heure de départ par défaut** (`heureDepart`): Heure de départ par défaut pour le calcul des prévisions météo.
    -   *Défaut*: "09:30"
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Meteo/heureDepart.md)
-   **Vitesse moyenne par défaut (km/h)** (`vitesseMoyenne`): Vitesse moyenne par défaut pour estimer la progression sur le parcours.
    -   *Défaut*: 28
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Meteo/vitesseMoyenne.md)

#### Widgets

-   **Information Météo** (`informationMeteo`): Afficher le widget d'informations météorologiques.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Meteo/Widgets/informationMeteo.md)
-   **Boussole** (`boussole`): Afficher le widget boussole.
    -   *Défaut*: true
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Meteo/Widgets/boussole.md)
-   **Orientation de la boussole** (`orientationBoussole`): Référence pour le Nord de la boussole : aligné avec la trace ou avec la caméra.
    -   *Défaut*: "Trace"
    -   [Documentation détaillée](docs/DocParametrage/Visualisation/Meteo/Widgets/orientationBoussole.md)

## Système

-   **Taille fenêtre** (`tailleFenetre`): Dimensions de la fenêtre au démarrage.
    -   *Défaut*: "1920x1080"
    -   [Documentation détaillée](docs/DocParametrage/Systeme/tailleFenetre.md)
-   **Ecran** (`ecran`): Ecran sur lequel l'application doit s'ouvrir.
    -   *Défaut*: 1
    -   [Documentation détaillée](docs/DocParametrage/Systeme/ecran.md)

### Timers

-   **Interval de vérification du réseau** (`networkPolling`): Interval de temps entre chaque vérification de l'état du réseau (en ms)
    -   *Défaut*: 30000
    -   [Documentation détaillée](docs/DocAnnexe/polling_timer_documentation.md)

### Tokens

-   **Token Mapbox** (`mapbox`): Token d'accès pour les services Mapbox
    -   *Défaut*: ""
    -   [Documentation détaillée](docs/DocAnnexe/obtenir_token_mapbox.md)

### Télécommande

-   **Port du serveur de télécommande** (`Port`): Port utilisé par le serveur WebSocket pour la télécommande.
    -   *Défaut*: 9001
    -   [Documentation détaillée](docs/DocParametrage/Systeme/Telecommande/Port.md)
-   **Sensibilité du point de vue (X) (1-500)** (`sensibilitePointDeVueX`): Facteur de sensibilité pour le déplacement horizontal du point de vue. Une valeur de 100 correspond à la sensibilité par défaut.
    -   *Défaut*: 200
    -   [Documentation détaillée](docs/DocParametrage/Systeme/Telecommande/sensibilitePointDeVueX.md)
-   **Sensibilité du point de vue (Y) (1-500)** (`sensibilitePointDeVueY`): Facteur de sensibilité pour le déplacement vertical du point de vue. Une valeur de 100 correspond à la sensibilité par défaut.
    -   *Défaut*: 200
    -   [Documentation détaillée](docs/DocParametrage/Systeme/Telecommande/sensibilitePointDeVueY.md)
-   **Sensibilité du cap** (`sensibiliteCap`): Facteur de sensibilité pour la rotation du cap de la caméra.
    -   *Défaut*: 50
    -   [Documentation détaillée](docs/DocParametrage/Systeme/Telecommande/sensibiliteCap.md)
-   **Sensibilité du zoom (1-500)** (`sensibiliteZoom`): Facteur de sensibilité pour le zoom de la caméra. Une valeur de 100 correspond à la sensibilité par défaut.
    -   *Défaut*: 50
    -   [Documentation détaillée](docs/DocParametrage/Systeme/Telecommande/sensibiliteZoom.md)
-   **Sensibilité de l'inclinaison (tilt)** (`sensibiliteTilt`): Facteur de sensibilité pour l'inclinaison (tilt) de la caméra.
    -   *Défaut*: 50
    -   [Documentation détaillée](docs/DocParametrage/Systeme/Telecommande/sensibiliteTilt.md)

