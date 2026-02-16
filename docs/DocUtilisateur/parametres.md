# Configuration de VisuGPS

> [!IMPORTANT]
>
>
> 
> Avant de modifier vos réglages, nous vous recommandons de lire l'**[Introduction à la gestion des paramètres](./introduction_parametres.md)** pour comprendre la symbolique des icônes et les précautions à prendre avec les réglages critiques, identifiés par l'icône ⚠️.

Ce document répertorie l'ensemble des paramètres de configuration de l'application, organisés selon l'arborescence officielle. Chaque paramètre est lié à sa documentation détaillée.


---

## 1. 🟢 Accueil
**Description** : Gestion de l'écran principal, de la mise à jour des données géographiques et de l'apparence de la bibliothèque.
*   [Nombre de circuits par page](../DocParametrage/Accueil/circuitsPerPage.md)
*   [Taille de la vignette](../DocParametrage/Accueil/TailleVignette.md)
*   [Nombre de favoris](../DocParametrage/Accueil/NombreFavoris.md)

### 1.1. MajCommunes
**Description** : Configuration des services permettant d'identifier automatiquement les communes traversées par vos traces.

#### 1.1.1. Timers
**Description** : Réglages des délais entre les requêtes pour respecter les quotas des fournisseurs de données (IGN, Mapbox, OpenStreetMap).
*   [Timer IGN (ms)](../DocParametrage/Accueil/MajCommunes/Timers/TimerIGN.md)
*   [Timer Mapbox (ms)](../DocParametrage/Accueil/MajCommunes/Timers/TimerMapbox.md)
*   [Timer OpenStreetMap (ms)](../DocParametrage/Accueil/MajCommunes/Timers/TimerOSM.md)

#### 1.1.2. APIs
**Description** : Activation individuelle des services de géocodage.
*   [Activer l'API IGN par défaut](../DocParametrage/Accueil/MajCommunes/APIs/IgnActif.md)
*   ⚠️ [Activer l'API Mapbox par défaut](../DocParametrage/Accueil/MajCommunes/APIs/MapboxActif.md)

---

## 2. 🔵 Importation
**Description** : Réglages appliqués automatiquement lors de l'ajout d'un nouveau fichier GPX à votre bibliothèque.
*   [Suppression après importation](../DocParametrage/Importation/autoDelete.md)
*   [Dossier d'import](../DocParametrage/Importation/ImportDir.md)
*   ⚠️ [Distance de lissage du dénivelé](../DocParametrage/Importation/denivele_lissage_distance.md)
*   ⚠️ [Filtre Médian Altitude](../DocParametrage/Importation/altitude_smoothing_median_window.md)
*   ⚠️ [Lissage Moyenne Altitude](../DocParametrage/Importation/altitude_smoothing_avg_window.md)
*   ⚠️ [Pente Max Autorisée (%)](../DocParametrage/Importation/max_gradient_percent.md)

### 2.1. Vignette
**Description** : Configuration de la miniature 2D générée pour illustrer le circuit dans la liste.
*   [Style de la vignette](../DocParametrage/Importation/Vignette/styleVignette.md)

#### 2.1.1. Dimensions
**Description** : Taille et rapport de forme de l'image générée.
*   [Largeur de la vignette](../DocParametrage/Importation/Vignette/Dimensions/largeur.md)
*   [Format de la vignette](../DocParametrage/Importation/Vignette/Dimensions/format.md)

#### 2.1.2. Trace
**Description** : Apparence visuelle de la ligne du parcours sur la vignette.
*   [Couleur de la trace sur la vignette](../DocParametrage/Importation/Vignette/Trace/colorGPXVignette.md)
*   [Largeur de la trace](../DocParametrage/Importation/Vignette/Trace/largeurTrace.md)

#### 2.1.3. MarqueurDistance
**Description** : Affichage des jalons kilométriques sur la vignette.
*   [Afficher la distance](../DocParametrage/Importation/Vignette/MarqueurDistance/presenceDistance.md)
*   [Intervalle distance](../DocParametrage/Importation/Vignette/MarqueurDistance/Distance.md)
*   [Couleur des marqueurs de distance](../DocParametrage/Importation/Vignette/MarqueurDistance/couleurPinDistance.md)

#### 2.1.4. DepartArrivee
**Description** : Gestion de la visibilité et de la couleur des points de départ et d'arrivée sur la vignette.
*   [Afficher les marqueurs](../DocParametrage/Importation/Vignette/DepartArrivee/Vignettes.md)
*   [Couleur du marqueur de départ](../DocParametrage/Importation/Vignette/DepartArrivee/couleurDepart.md)
*   [Couleur du marqueur d'arrivée](../DocParametrage/Importation/Vignette/DepartArrivee/couleurArrivee.md)
*   [Distance max départ/arrivée (m)](../DocParametrage/Importation/Vignette/DepartArrivee/distanceMax.md)
*   [Couleur du marqueur départ/arrivée (proches)](../DocParametrage/Importation/Vignette/DepartArrivee/couleurDepartArrivee.md)

### 2.2. Tracking
**Description** : Paramètres techniques d'analyse de la trace (segmentation et lissage du cap).
*   ⚠️ [Longueur du segment](../DocParametrage/Importation/Tracking/LongueurSegment.md)
*   ⚠️ [Lissage du cap](../DocParametrage/Importation/Tracking/LissageCap.md)
*   ⚠️ [Seuil de détection des superpositions](../DocParametrage/Importation/Tracking/seuilDetectionSuperposition.md)

### 2.3. Caméra
**Description** : Valeurs par défaut de la vue 3D lors du premier chargement d'un circuit.
*   [Zoom](../DocParametrage/Importation/Camera/Zoom.md)
*   [Pitch](../DocParametrage/Importation/Camera/Pitch.md)

### 2.4. QRCode
**Description** : Taille du QR Code permettant de retrouver l'origine de la trace.
*   [Taille du QR code](../DocParametrage/Importation/QRCode/taille.md)

### 2.5. Label Départ Arrivée
**Description** : Configuration des messages de texte ajoutés automatiquement aux extrémités de la trace.
*   [Afficher le départ](../DocParametrage/Importation/LabelDepartArrivee/afficherDepart.md)
*   [Message départ](../DocParametrage/Importation/LabelDepartArrivee/messageDepart.md)
*   [Orientation départ à droite](../DocParametrage/Importation/LabelDepartArrivee/orientationDepartDroite.md)
*   [Post affichage départ](../DocParametrage/Importation/LabelDepartArrivee/postAffichageDepart.md)
*   [Afficher l'arrivée](../DocParametrage/Importation/LabelDepartArrivee/afficherArrivee.md)
*   [Message arrivée](../DocParametrage/Importation/LabelDepartArrivee/messageArrivee.md)
*   [Orientation arrivée à droite](../DocParametrage/Importation/LabelDepartArrivee/orientationArriveeDroite.md)
*   [Pré affichage arrivée](../DocParametrage/Importation/LabelDepartArrivee/preAffichageArrivee.md)

---

## 3. 🟠 Edition
**Description** : Configuration de l'interface de travail permettant de réaliser la mise en scène 3D de vos parcours.

### 3.1. Vue 3D
**Description** : Réglages de l'affichage de la carte et de la trace dans la fenêtre d'édition principale.

#### 3.1.1. Trace
**Description** : Épaisseurs et couleurs de la ligne de suivi et d'avancement.
*   [Épaisseur de l'avancement](../DocParametrage/Edition/Vue%203D/Trace/epaisseurAvancement.md)
*   [Couleur de l'avancement](../DocParametrage/Edition/Vue%203D/Trace/couleurAvancement.md)
*   [Épaisseur de la trace](../DocParametrage/Edition/Vue%203D/Trace/epaisseur.md)
*   [Couleur de la trace](../DocParametrage/Edition/Vue%203D/Trace/couleur.md)
*   [Colorer la trace selon la pente](../DocParametrage/Edition/Vue%203D/Trace/colorerSelonPente.md)

#### 3.1.2. Carte
**Description** : Choix du style de fond de carte et accentuation du relief.
*   [Style de la carte](../DocParametrage/Edition/Vue%203D/Carte/styleVisualisation.md)
*   [Coefficient de relief](../DocParametrage/Edition/Vue%203D/Carte/exaggeration.md)

### 3.2. Avancement dans les graphes
**Description** : Personnalisation de l'indicateur visuel de position sur les graphiques techniques inférieurs.
*   [Couleur de la zone d'avancement](../DocParametrage/Edition/Avancement%20dans%20les%20graphes/couleurAvancementZone.md)
*   [Opacité de la zone d'avancement](../DocParametrage/Edition/Avancement%20dans%20les%20graphes/opaciteAvancementZone.md)

### 3.3. Camera
**Description** : Réglages des automatismes et des graphiques liés aux mouvements de caméra.
*   [Zoom](../DocParametrage/Edition/Camera/Zoom.md)

#### 3.3.1. ZoomDépart
**Description** : Configuration du décollage automatisé (éloignement progressif) au début du circuit.
*   [Activer zoom départ](../DocParametrage/Edition/Camera/ZoomDépart/zoomDepart.md)
*   [Valeur du zoom départ](../DocParametrage/Edition/Camera/ZoomDépart/zoomDepartValeur.md)
*   [Distance du zoom départ](../DocParametrage/Edition/Camera/ZoomDépart/zoomDepartDistance.md)

#### 3.3.2. ZoomArrivée
**Description** : Configuration de l'atterrissage automatisé (zoom progressif) à la fin du circuit.
*   [Activer zoom arrivée](../DocParametrage/Edition/Camera/ZoomArrivée/zoomArrivee.md)
*   [Valeur du zoom arrivée](../DocParametrage/Edition/Camera/ZoomArrivée/zoomArriveeValeur.md)
*   [Distance du zoom arrivée](../DocParametrage/Edition/Camera/ZoomArrivée/distanceZoomArrivee.md)

#### 3.3.3. Graphe caméra
**Description** : Paramètres d'affichage détaillés pour le graphique technique (Cap, Zoom, Pitch).
*   [Couleur des points de contrôle](../DocParametrage/Edition/Camera/Graphe%20caméra/couleurPointDeControle.md)
*   [Epaisseur Point de Contrôle](../DocParametrage/Edition/Camera/Graphe%20caméra/epaisseurPointDeControle.md)
*   [Longueur Point de Contrôle](../DocParametrage/Edition/Camera/Graphe%20caméra/longueurPointDeControle.md)

##### 3.3.3.1. Affichage courbes
**Description** : Visibilité des données brutes (Calculé) et de vos modifications (Édité).
*   [Afficher Delta Bearing (Calculé)](../DocParametrage/Edition/Camera/Graphe%20caméra/AffichageCourbes/afficherBearingDeltaCalcule.md)
*   [Afficher Delta Bearing (Édité)](../DocParametrage/Edition/Camera/Graphe%20caméra/AffichageCourbes/afficherBearingDeltaEdite.md)
*   [Afficher Somme Delta Bearing (Calculé)](../DocParametrage/Edition/Camera/Graphe%20caméra/AffichageCourbes/afficherBearingTotalDeltaCalcule.md)
*   [Afficher Somme Delta Bearing (Édité)](../DocParametrage/Edition/Camera/Graphe%20caméra/AffichageCourbes/afficherBearingTotalDeltaEdite.md)
*   [Afficher Zoom (Édité)](../DocParametrage/Edition/Camera/Graphe%20caméra/AffichageCourbes/afficherZoomEdite.md)
*   [Afficher Pitch (Édité)](../DocParametrage/Edition/Camera/Graphe%20caméra/AffichageCourbes/afficherPitchEdite.md)

##### 3.3.3.2. Couleur courbes
**Description** : Codes couleurs pour distinguer chaque type de donnée sur le graphe.
*   [Couleur du Zoom](../DocParametrage/Edition/Camera/Graphe%20caméra/CouleurCourbes/couleurZoom.md)
*   [Couleur Zoom Edité](../DocParametrage/Edition/Camera/Graphe%20caméra/CouleurCourbes/couleurEditedZoom.md)
*   [Couleur du Pitch](../DocParametrage/Edition/Camera/Graphe%20caméra/CouleurCourbes/couleurPitch.md)
*   [Couleur Pitch Edité](../DocParametrage/Edition/Camera/Graphe%20caméra/CouleurCourbes/couleurEditedPitch.md)
*   [Couleur du Delta Bearing](../DocParametrage/Edition/Camera/Graphe%20caméra/CouleurCourbes/couleurBearingDelta.md)
*   [Couleur Delta Bearing Edité](../DocParametrage/Edition/Camera/Graphe%20caméra/CouleurCourbes/couleurEditedBearingDelta.md)
*   [Couleur du Somme Delta Bearing](../DocParametrage/Edition/Camera/Graphe%20caméra/CouleurCourbes/couleurBearingTotalDelta.md)
*   [Couleur Somme Delta Bearing Edité](../DocParametrage/Edition/Camera/Graphe%20caméra/CouleurCourbes/couleurEditedBearingTotalDelta.md)

### 3.4. Pause et Survol
**Description** : Configuration des événements de transition (survol d'un point d'intérêt) et de pause.
*   [Durée du survol (sec)](../DocParametrage/Edition/Pause_Flyto/dureeSurvol.md)
*   [Couleur de la croix centrale](../DocParametrage/Edition/Pause_Flyto/couleurCroixCentraleEdition.md)

#### 3.4.1. Graphe Pause et Survol
**Description** : Couleurs et dimensions des symboles d'événements sur le graphe.
*   [Couleur des pauses](../DocParametrage/Edition/Pause_Flyto/Graphe%20Pause_Flyto/couleurPause.md)
*   [Couleur des survols](../DocParametrage/Edition/Pause_Flyto/Graphe%20Pause_Flyto/couleurFlyto.md)
*   [Longueur](../DocParametrage/Edition/Pause_Flyto/Graphe%20Pause_Flyto/longueur.md)

### 3.5. Messages
**Description** : Gestion des paramètres d'apparition et d'ergonomie pour les bulles de texte.
*   [Pré-affichage](../DocParametrage/Edition/Messages/preAffichage.md)
*   [Post-affichage](../DocParametrage/Edition/Messages/postAffichage.md)
*   [Durée de transition du Pitch](../DocParametrage/Edition/Messages/transitionDuree.md)
*   [URL Emoticon](../DocParametrage/Edition/Messages/urlEmoticon.md)

#### 3.5.1. Distance
**Description** : Configuration automatique des bornes kilométriques pour chaque circuit.
*   [Afficher les distances](../DocParametrage/Edition/Messages/Distance/afficherDistances.md)
*   [Orientation à gauche](../DocParametrage/Edition/Messages/Distance/orientationAGauche.md)
*   [Couleur des messages](../DocParametrage/Edition/Messages/Distance/couleurMessages.md)
*   [Intervalle](../DocParametrage/Edition/Messages/Distance/intervalle.md)
*   [Pre-affichage](../DocParametrage/Edition/Messages/Distance/preAffichage.md)
*   [Post-affichage](../DocParametrage/Edition/Messages/Distance/postAffichage.md)

#### 3.5.2. Graphe messages
**Description** : Apparence des blocs de messages sur le graphique des événements.
*   [Hauteur des messages](../DocParametrage/Edition/Messages/Graphe%20messages/HauteurMessages.md)

### 3.6. Commandes clavier
**Description** : Affectation des touches et sensibilité des déplacements pas-à-pas.
*   [Incrément Avancement](../DocParametrage/Edition/CommandesClavier/incrementAvancement.md)
*   [Incrément Avancement (Shift)](../DocParametrage/Edition/CommandesClavier/incrementAvancementShift.md)
*   [Incrément Pitch](../DocParametrage/Edition/CommandesClavier/incrementPitch.md)
*   [Incrément Pitch (Shift)](../DocParametrage/Edition/CommandesClavier/incrementPitchShift.md)
*   [Touche Avancement Avant](../DocParametrage/Edition/CommandesClavier/toucheAvancementAvant.md)
*   [Touche Avancement Arrière](../DocParametrage/Edition/CommandesClavier/toucheAvancementArriere.md)
*   [Touche Pitch Haut](../DocParametrage/Edition/CommandesClavier/touchePitchHaut.md)
*   [Touche Pitch Bas](../DocParametrage/Edition/CommandesClavier/touchePitchBas.md)

### 3.7. Commandes souris
**Description** : Sensibilité de la molette pour le zoom et le cap.
*   [Incrément Zoom](../DocParametrage/Edition/CommandesSouris/incrementZoom.md)
*   [Incrément Zoom (Shift)](../DocParametrage/Edition/CommandesSouris/incrementZoomShift.md)
*   [Incrément Cap](../DocParametrage/Edition/CommandesSouris/incrementBearing.md)
*   [Incrément Cap (Shift)](../DocParametrage/Edition/CommandesSouris/incrementBearingShift.md)

---

## 4. 🟣 Variante
**Description** : Configuration de l'interface de création de traces alternatives (déviations, nouveaux départs/arrivées).

### 4.1. Edition
**Description** : Réglages graphiques pour la trace de la variante en cours d'édition.

#### 4.1.1. Trace
*   [Couleur de la trace](../DocParametrage/Variante/Edition/Trace/couleur.md)
*   [Largeur de la trace](../DocParametrage/Variante/Edition/Trace/largeur.md)
*   [Colorer selon la pente](../DocParametrage/Variante/Edition/Trace/colorerSelonPente.md)

#### 4.1.2. Trace Maîtresse
*   [Couleur de fond](../DocParametrage/Variante/Edition/TraceMaitresse/couleur.md)
*   [Largeur de fond](../DocParametrage/Variante/Edition/TraceMaitresse/largeur.md)
*   [Opacité de fond](../DocParametrage/Variante/Edition/TraceMaitresse/opacite.md)

#### 4.1.3. Carte
*   [Style de carte](../DocParametrage/Variante/Edition/Carte/style.md)

#### 4.1.4. Noeuds
*   [Couleur des nœuds](../DocParametrage/Variante/Edition/Noeuds/couleur.md)
*   [Zoom visibilité nœuds](../DocParametrage/Variante/Edition/Noeuds/zoomVisu.md)

### 4.2. Visualisation
**Description** : Réglages du comportement de la caméra lors de visualisation de variantes.
*   [Durée du flyto vers le segment](../DocParametrage/Variante/Visualisation/dureeFlytoSegment.md)
*   [Afficher les segments](../DocParametrage/Variante/Visualisation/afficherSegments.md)
*   [Afficher la pente](../DocParametrage/Variante/Visualisation/afficherPente.md)
*   [Épaisseur des segments](../DocParametrage/Variante/Visualisation/epaisseurSegments.md)
*   [Opacité des segments](../DocParametrage/Variante/Visualisation/opaciteSegments.md)
*   [Épaisseur de la pente](../DocParametrage/Variante/Visualisation/epaisseurPente.md)
*   [Opacité de la pente](../DocParametrage/Variante/Visualisation/opacitePente.md)
*   [Couleur de la trace](../DocParametrage/Variante/Visualisation/couleurTrace.md)
*   [Couleur Nouveau tronçon](../DocParametrage/Variante/Visualisation/couleurNouveau.md)
*   [Couleur Tronçon commun](../DocParametrage/Variante/Visualisation/couleurCommun.md)
*   [Couleur Tronçon abandonné](../DocParametrage/Variante/Visualisation/couleurAbandonne.md)
*   [Afficher Segment Abandonné](../DocParametrage/Variante/Visualisation/afficherSegmentAbandonne.md)

#### 4.2.1. Widgets
**Description** : État initial (visible ou masqué) des indicateurs d'interface au lancement de l'animation d'une variante.
*   [Distance](../DocParametrage/Variante/Visualisation/Widgets/distance.md)
*   [Communes](../DocParametrage/Variante/Visualisation/Widgets/communes.md)
*   [Altitude](../DocParametrage/Variante/Visualisation/Widgets/altitude.md)
*   [Commandes](../DocParametrage/Variante/Visualisation/Widgets/commandes.md)
*   [Météo](../DocParametrage/Variante/Visualisation/Widgets/meteo.md)
*   [Boussole](../DocParametrage/Variante/Visualisation/Widgets/boussole.md)

### 4.3. Parametres
**Description** : Configuration des services de routage (GraphHopper, OpenRouteService).
*   [Clé API GraphHopper](../DocParametrage/Variante/Parametres/apiKeyGraphHopper.md)
*   [Clé API OpenRouteService](../DocParametrage/Variante/Parametres/apiKeyOpenRouteService.md)
*   [Service de routage](../DocParametrage/Variante/Parametres/routingService.md)
*   [Profil de routage](../DocParametrage/Variante/Parametres/routingType.md)

---

## 5. 🟡 Visualisation
**Description** : Réglages de l'expérience de vol finale, où l'animation est lancée en plein écran.

### 5.1. Vue 3D
**Description** : Rendu visuel global de la carte et du parcours pendant la lecture.

#### 5.1.1. Carte
**Description** : Style de carte (Satellite, Outdoor, etc.).
*   [Style de la carte.](../DocParametrage/Visualisation/Vue%203D/Carte/styleVisualisation.md)

#### 5.1.2. Trace
**Description** : Personnalisation visuelle du parcours et de la 'comète' qui guide l'œil.
*   [Colorer la trace selon la pente](../DocParametrage/Visualisation/Vue%203D/Trace/colorerSelonPente.md)
*   [Couleur de la trace](../DocParametrage/Visualisation/Vue%203D/Trace/couleurTrace.md)
*   [Epaisseur de la trace](../DocParametrage/Visualisation/Vue%203D/Trace/epaisseurTrace.md)
*   [Opacité de la trace](../DocParametrage/Visualisation/Vue%203D/Trace/opaciteTrace.md)
*   [Couleur de la comète](../DocParametrage/Visualisation/Vue%203D/Trace/couleurComete.md)
*   [Epaisseur de la comète](../DocParametrage/Visualisation/Vue%203D/Trace/epaisseurComete.md)
*   [Opacité de la comète](../DocParametrage/Visualisation/Vue%203D/Trace/opaciteComete.md)
*   [Longueur de la comète (m)](../DocParametrage/Visualisation/Vue%203D/Trace/longueurComete.md)

### 5.2. Widgets
**Description** : État initial (visible ou masqué) des indicateurs d'interface au lancement de l'animation.
*   [Distance](../DocParametrage/Visualisation/Widgets/distance.md)
*   [Communes](../DocParametrage/Visualisation/Widgets/communes.md)
*   [Altitude](../DocParametrage/Visualisation/Widgets/altitude.md)
*   [Commandes](../DocParametrage/Visualisation/Widgets/commandes.md)
*   [Météo](../DocParametrage/Visualisation/Widgets/meteo.md)
*   [Boussole](../DocParametrage/Visualisation/Widgets/boussole.md)

### 5.3. Lancement
**Description** : Séquence d'introduction cinématographique (Plongée depuis l'espace vers le circuit).
*   [Style de la carte au lancement](../DocParametrage/Visualisation/Lancement/styleLancement.md)
*   [Zoom initial](../DocParametrage/Visualisation/Lancement/zoomEurope.md)
*   [Durée de l'animation vers la trace au départ](../DocParametrage/Visualisation/Lancement/durationEuropeToTrace.md)
*   [Durée de la pause sur la trace](../DocParametrage/Visualisation/Lancement/pauseBeforeStart.md)
*   [Reprise automatique après vue trace](../DocParametrage/Visualisation/Lancement/repriseAutoVueTrace.md)
*   [Durée de l'animation vers le départ](../DocParametrage/Visualisation/Lancement/durationTraceToStart.md)
*   [Durée de la pause au Km 0](../DocParametrage/Visualisation/Lancement/pauseAuKm0.md)
*   [Reprise automatique au Km 0](../DocParametrage/Visualisation/Lancement/repriseAutoKm0.md)
*   [Marge autour de la trace (Début)](../DocParametrage/Visualisation/Lancement/margeLancement.md)

### 5.4. Lecture
**Description** : Paramètres de contrôle de la vitesse et comportement dynamique pendant le mouvement.
*   [Vitesse de l'animation (ms/km)](../DocParametrage/Visualisation/Lecture/vitesse.md)
*   [Durée déplacement](../DocParametrage/Visualisation/Lecture/jumpDuration.md)
*   [Durée de la reprise après pause (sec)](../DocParametrage/Visualisation/Lecture/timerReprisePause.md)
*   [Délai avant de masquer le curseur (ms)](../DocParametrage/Visualisation/Lecture/masquerCurseurDelai.md)
*   [Afficher la croix centrale en pause](../DocParametrage/Visualisation/Lecture/afficherCroixCentrale.md)
*   [Couleur de la croix centrale](../DocParametrage/Visualisation/Lecture/couleurCroixCentrale.md)
*   [Zoom Minimum](../DocParametrage/Visualisation/Lecture/zoomMinimum.md)

#### 5.4.1. Vitesse
**Description** : Bornes de vitesse et réglages du curseur de contrôle.
*   [Vitesse minimale (x)](../DocParametrage/Visualisation/Lecture/Vitesse/min_value.md)
*   [Vitesse maximale (x)](../DocParametrage/Visualisation/Lecture/Vitesse/max_value.md)
*   [Vitesse par défaut (x)](../DocParametrage/Visualisation/Lecture/Vitesse/default_value.md)

#### 5.4.2. ZoomDynamique
**Description** : Système de zoom intelligent qui s'adapte à la vitesse de vol.
*   [Intensité du zoom dynamique](../DocParametrage/Visualisation/Lecture/ZoomDynamique/intensite_zoom_dynamique.md)

### 5.5. Finalisation
**Description** : Actions automatiques déclenchées lorsque le marqueur atteint l'arrivée.
*   [Durée de la pause à l'arrivée](../DocParametrage/Visualisation/Finalisation/delayAfterAnimationEnd.md)
*   [Durée de l'animation vers la trace à l'arrivée](../DocParametrage/Visualisation/Finalisation/flyToGlobalDuration.md)
*   [Durée de l'animation vers le départ](../DocParametrage/Visualisation/Finalisation/flyToKm0Duration.md)
*   [Reprise automatique](../DocParametrage/Visualisation/Finalisation/repriseAutomatique.md)
*   [Durée de la pause avant reprise auto](../DocParametrage/Visualisation/Finalisation/pauseAvantReprise.md)
*   [Marge autour de la trace (Fin)](../DocParametrage/Visualisation/Finalisation/margeFinalisation.md)

### 5.6. Profil Altitude
**Description** : Configuration graphique de la vue en coupe du terrain.

#### 5.6.1. Graphe
**Description** : Échelles des axes et fréquence des annotations de distance/altitude.
*   [Echelle des abscisses](../DocParametrage/Visualisation/Profil%20Altitude/Graphe/Abscisse.md)
*   [Echelle des ordonnées](../DocParametrage/Visualisation/Profil%20Altitude/Graphe/Ordonnee.md)
*   [Aspect curseur lié à la comète](../DocParametrage/Visualisation/Profil%20Altitude/Graphe/aspectCurseurLieComete.md)
*   [Intervalle des repères distances](../DocParametrage/Visualisation/Profil%20Altitude/Graphe/RepereDistance.md)
*   [Intervalle des repères altitude](../DocParametrage/Visualisation/Profil%20Altitude/Graphe/RepereAltitude.md)
*   [Position du curseur.](../DocParametrage/Visualisation/Profil%20Altitude/Graphe/CurseurPositionKm.md)

#### 5.6.2. Couleurs
**Description** : Attribution des couleurs selon le pourcentage d'inclinaison.
*   [Pente <= 0%](../DocParametrage/Visualisation/Profil%20Altitude/Couleurs/TrancheNegative.md)
*   [0% < Pente < 3%](../DocParametrage/Visualisation/Profil%20Altitude/Couleurs/Tranche1.md)
*   [3% <= Pente < 6%](../DocParametrage/Visualisation/Profil%20Altitude/Couleurs/Tranche2.md)
*   [6% <= Pente < 9%](../DocParametrage/Visualisation/Profil%20Altitude/Couleurs/Tranche3.md)
*   [9% <= Pente < 12%](../DocParametrage/Visualisation/Profil%20Altitude/Couleurs/Tranche4.md)
*   [Pente >= 12%](../DocParametrage/Visualisation/Profil%20Altitude/Couleurs/Tranche5.md)

### 5.7. Taille des Messages
**Description** : Réglages de la lisibilité des bulles de texte sur la carte.
*   ⚠️ [Taille de police de base des messages (px)](../DocParametrage/Visualisation/Taille%20des%20Messages/baseFontSize.md)
*   ⚠️ [Coefficient largeur message](../DocParametrage/Visualisation/Taille%20des%20Messages/coefLargeurMessage.md)
*   ⚠️ [Coefficient réducteur message](../DocParametrage/Visualisation/Taille%20des%20Messages/coefReducteurMessage.md)

### 5.8. Météo
**Description** : Gestion des données météorologiques et de l'orientation sur le parcours.
*   [Activer la météo](../DocParametrage/Visualisation/Meteo/meteoActif.md)
*   [Heure de début de journée](../DocParametrage/Visualisation/Meteo/heureDebutJournee.md)
*   [Heure de fin de journée](../DocParametrage/Visualisation/Meteo/heureFinJournee.md)
*   [Heure de départ par défaut](../DocParametrage/Visualisation/Meteo/heureDepart.md)
*   [Vitesse moyenne par défaut (km/h)](../DocParametrage/Visualisation/Meteo/vitesseMoyenne.md)

#### 5.8.1. Widgets
**Description** : Gadgets visuels pour la météo et la navigation.
*   [Information Météo](../DocParametrage/Visualisation/Meteo/Widgets/informationMeteo.md)
*   [Boussole](../DocParametrage/Visualisation/Meteo/Widgets/boussole.md)
*   [Orientation de la boussole](../DocParametrage/Visualisation/Meteo/Widgets/orientationBoussole.md)

---

## 6. ⚙️ Système
**Description** : Paramètres internes, gestion du matériel et services de communication.
*   [Taille fenêtre](../DocParametrage/Systeme/tailleFenetre.md)
*   [Ecran](../DocParametrage/Systeme/ecran.md)
*   [Mode Debug](../DocParametrage/Systeme/Debug.md)

### 6.1. Timers
**Description** : Réglages réseau (Vérification connexion).
*   ⚠️ [Interval de vérification du réseau](../DocAnnexe/polling_timer_documentation.md)

### 6.2. Tokens
**Description** : Clés d'accès aux services Mapbox.
*   ⚠️ [Token Mapbox](../DocAnnexe/obtenir_token_mapbox.md)

### 6.3. Télécommande
**Description** : Configuration du serveur de communication et sensibilité des contrôles mobiles.
*   ⚠️ [Port du serveur de télécommande](../DocParametrage/Systeme/Telecommande/Port.md)
*   [Sensibilité du point de vue (X) (1-500)](../DocParametrage/Systeme/Telecommande/sensibilitePointDeVueX.md)
*   [Sensibilité du point de vue (Y) (1-500)](../DocParametrage/Systeme/Telecommande/sensibilitePointDeVueY.md)
*   [Sensibilité du cap](../DocParametrage/Systeme/Telecommande/sensibiliteCap.md)
*   [Sensibilité du zoom (1-500)](../DocParametrage/Systeme/Telecommande/sensibiliteZoom.md)
*   [Sensibilité de l'inclinaison (tilt)](../DocParametrage/Systeme/Telecommande/sensibiliteTilt.md)

#### 6.3.1. Rétention
**Description** : Gestion de la conservation des données de connexion des terminaux mobiles.
*   [Rétention des télécommandes autorisées](../DocParametrage/Systeme/Telecommande/Retention/autorisees.md)
*   [Rétention des télécommandes interdites](../DocParametrage/Systeme/Telecommande/Retention/interdites.md)

---
