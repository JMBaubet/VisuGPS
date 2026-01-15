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

### 1.2. Paramètres d'Affichage
**Description** : Configuration de l'interface de la liste des circuits sur l'écran d'accueil.
*   [Nombre de circuits par page](../DocParametrage/Accueil/circuitsPerPage.md)
*   [Taille de la vignette](../DocParametrage/Accueil/TailleVignette.md)

---

## 2. 🔵 Importation
**Description** : Réglages appliqués automatiquement lors de l'ajout d'un nouveau fichier GPX à votre bibliothèque.

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
*   [Largeur de la trace sur la vignette](../DocParametrage/Importation/Vignette/Trace/largeurTrace.md)

#### 2.1.3. MarqueurDistance
**Description** : Affichage des jalons kilométriques sur la vignette.
*   [Afficher la distance sur la vignette](../DocParametrage/Importation/Vignette/MarqueurDistance/presenceDistance.md)
*   [Intervalle distance sur la vignette](../DocParametrage/Importation/Vignette/MarqueurDistance/Distance.md)
*   [Couleur des marqueurs de distance sur la vignette](../DocParametrage/Importation/Vignette/MarqueurDistance/couleurPinDistance.md)

#### 2.1.4. DepartArrivee
**Description** : Gestion de la visibilité et de la couleur des points de départ et d'arrivée sur la vignette.
*   [Afficher les marqueurs sur la vignette](../DocParametrage/Importation/Vignette/DepartArrivee/Vignettes.md)
*   [Couleur du marqueur de départ sur la vignette](../DocParametrage/Importation/Vignette/DepartArrivee/couleurDepart.md)
*   [Couleur du marqueur d'arrivée sur la vignette](../DocParametrage/Importation/Vignette/DepartArrivee/couleurArrivee.md)
*   [Distance max départ/arrivée (m)](../DocParametrage/Importation/Vignette/DepartArrivee/distanceMax.md)
*   [Couleur du marqueur départ/arrivée (proches) sur la vignette](../DocParametrage/Importation/Vignette/DepartArrivee/couleurDepartArrivee.md)

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

### 2.6. Autres paramètres d'importation
**Description** : Gestion des fichiers source, du calcul de dénivelé et du nettoyage de l'altitude.
*   [Suppression après importation](../DocParametrage/Importation/autoDelete.md)
*   [Dossier d'import](../DocParametrage/Importation/ImportDir.md)
*   ⚠️ [Distance de lissage du dénivelé](../DocParametrage/Importation/denivele_lissage_distance.md)
*   ⚠️ [Pente Maximum Autorisée (%)](../DocParametrage/Importation/max_gradient_percent.md)
*   [Fenêtre du Filtre Médian](../DocParametrage/Importation/altitude_smoothing_median_window.md)
*   [Fenêtre de Moyenne Glissante](../DocParametrage/Importation/altitude_smoothing_avg_window.md)

---

## 3. 🟠 Édition
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

#### 3.3.1. Zoom_Depart
**Description** : Configuration du décollage automatisé (éloignement progressif) au début du circuit.
*   [Activer zoom départ](../DocParametrage/Edition/Camera/Zoom_Depart/zoomDepart.md)
*   [Valeur du zoom départ](../DocParametrage/Edition/Camera/Zoom_Depart/zoomDepartValeur.md)
*   [Distance du zoom départ](../DocParametrage/Edition/Camera/Zoom_Depart/zoomDepartDistance.md)

#### 3.3.2. Zoom_Arrivee
**Description** : Configuration de l'atterrissage automatisé (zoom progressif) à la fin du circuit.
*   [Activer zoom arrivée](../DocParametrage/Edition/Camera/Zoom_Arrivee/zoomArrivee.md)
*   [Valeur du zoom arrivée](../DocParametrage/Edition/Camera/Zoom_Arrivee/zoomArriveeValeur.md)
*   [Distance du zoom arrivée](../DocParametrage/Edition/Camera/Zoom_Arrivee/distanceZoomArrivee.md)

#### 3.3.3. Graphe caméra
**Description** : Paramètres d'affichage détaillés pour le graphique technique (Cap, Zoom, Pitch).

##### 3.3.3.1. Affichage courbes
**Description** : Visibilité des données brutes (Calculé) et de vos modifications (Édité).
*   [Afficher Delta Bearing (Calculé)](../DocParametrage/Edition/Camera/Graphe_camera/AffichageCourbes/afficherBearingDeltaCalcule.md)
*   [Afficher Delta Bearing (Édité)](../DocParametrage/Edition/Camera/Graphe_camera/AffichageCourbes/afficherBearingDeltaEdite.md)
*   [Afficher Somme Delta Bearing (Calculé)](../DocParametrage/Edition/Camera/Graphe_camera/AffichageCourbes/afficherBearingTotalDeltaCalcule.md)
*   [Afficher Somme Delta Bearing (Édité)](../DocParametrage/Edition/Camera/Graphe_camera/AffichageCourbes/afficherBearingTotalDeltaEdite.md)
*   [Afficher Zoom (Édité)](../DocParametrage/Edition/Camera/Graphe_camera/AffichageCourbes/afficherZoomEdite.md)
*   [Afficher Pitch (Édité)](../DocParametrage/Edition/Camera/Graphe_camera/AffichageCourbes/afficherPitchEdite.md)

##### 3.3.3.2. Couleur courbes
**Description** : Codes couleurs pour distinguer chaque type de donnée sur le graphe.
*   [Couleur du Zoom](../DocParametrage/Edition/Camera/Graphe_camera/CouleurCourbes/couleurZoom.md)
*   [Couleur Zoom Edité](../DocParametrage/Edition/Camera/Graphe_camera/CouleurCourbes/couleurEditedZoom.md)
*   [Couleur du Pitch](../DocParametrage/Edition/Camera/Graphe_camera/CouleurCourbes/couleurPitch.md)
*   [Couleur Pitch Edité](../DocParametrage/Edition/Camera/Graphe_camera/CouleurCourbes/couleurEditedPitch.md)
*   [Couleur du Delta Bearing](../DocParametrage/Edition/Camera/Graphe_camera/CouleurCourbes/couleurBearingDelta.md)
*   [Couleur Delta Bearing Edité](../DocParametrage/Edition/Camera/Graphe_camera/CouleurCourbes/couleurEditedBearingDelta.md)
*   [Couleur du Somme Delta Bearing](../DocParametrage/Edition/Camera/Graphe_camera/CouleurCourbes/couleurBearingTotalDelta.md)
*   [Couleur Somme Delta Bearing Edité](../DocParametrage/Edition/Camera/Graphe_camera/CouleurCourbes/couleurEditedBearingTotalDelta.md)

##### 3.3.3.3. Points de contrôle
**Description** : Apparence visuelle des marqueurs "Points Clés" sur le graphe.
*   [Couleur des points de contrôle](../DocParametrage/Edition/Camera/Graphe_camera/couleurPointDeControle.md)
*   [Epaisseur Point de Contrôle](../DocParametrage/Edition/Camera/Graphe_camera/epaisseurPointDeControle.md)
*   [Longueur Point de Contrôle](../DocParametrage/Edition/Camera/Graphe_camera/longueurPointDeControle.md)

### 3.4. Pause et Survol
**Description** : Configuration des événements de transition (survol d'un point d'intérêt) et de pause.
*   [Durée du survol (ms)](../DocParametrage/Edition/Pause_Flyto/dureeSurvol.md)
*   [Couleur de la croix centrale](../DocParametrage/Edition/Pause_Flyto/couleurCroixCentraleEdition.md)

#### 3.4.1. Graphe Pause et Survol
**Description** : Couleurs et dimensions des symboles d'événements sur le graphe.
*   [Couleur des pauses](../DocParametrage/Edition/Pause_Flyto/Graphe%20Pause_Flyto/couleurPause.md)
*   [Couleur des survols](../DocParametrage/Edition/Pause_Flyto/Graphe%20Pause_Flyto/couleurFlyto.md)
*   [Longueur des marqueurs](../DocParametrage/Edition/Pause_Flyto/Graphe%20Pause_Flyto/longueur.md)

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
**Description** : Apparence visuelle de la trace de variante.
*   [Couleur de la trace](../DocParametrage/Variante/Edition/Trace/couleur.md)
*   [Largeur de la trace](../DocParametrage/Variante/Edition/Trace/largeur.md)
*   [Colorer selon la pente](../DocParametrage/Variante/Edition/Trace/colorerSelonPente.md)

### 4.2. Parametres
**Description** : Configuration des services externes pour les variantes.
*   [Clé API GraphHopper](../DocParametrage/Variante/Parametres/routingApiKey.md)

---

## 5. 🟡 Visualisation
**Description** : Réglages de l'expérience de vol finale, où l'animation est lancée en plein écran.

### 5.1. Vue 3D
**Description** : Rendu visuel global de la carte et du parcours pendant la lecture.

#### 5.1.1. Carte
**Description** : Style de carte (Satellite, Outdoor, etc.).
*   [Style de la carte](../DocParametrage/Visualisation/Vue%203D/Carte/styleVisualisation.md)

#### 5.1.2. Trace
**Description** : Personnalisation visuelle du parcours et de la "comète" qui guide l'œil.
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

### 5.3. Lancement
**Description** : Séquence d'introduction cinématographique (Plongée depuis l'espace vers le circuit).
*   [Style de la carte au lancement](../DocParametrage/Visualisation/Lancement/styleLancement.md)
*   [Centre initial](../DocParametrage/Visualisation/Lancement/centerEurope.md)
*   [Zoom initial](../DocParametrage/Visualisation/Lancement/zoomEurope.md)
*   [Durée vers la trace](../DocParametrage/Visualisation/Lancement/durationEuropeToTrace.md)
*   [Pause sur la trace](../DocParametrage/Visualisation/Lancement/pauseBeforeStart.md)
*   [Durée vers le départ](../DocParametrage/Visualisation/Lancement/durationTraceToStart.md)
*   [Durée de la pause au Km 0](../DocParametrage/Visualisation/Lancement/pauseAuKm0.md)

### 5.4. Lecture
**Description** : Paramètres de contrôle de la vitesse et comportement dynamique pendant le mouvement.

#### 5.4.1. Vitesse
**Description** : Bornes de vitesse et réglages du curseur de contrôle.
*   [Vitesse minimale (x)](../DocParametrage/Visualisation/Lecture/Vitesse/min_value.md)
*   [Vitesse maximale (x)](../DocParametrage/Visualisation/Lecture/Vitesse/max_value.md)
*   [Vitesse par défaut (x)](../DocParametrage/Visualisation/Lecture/Vitesse/default_value.md)
*   [Pas du slider (x)](../DocParametrage/Visualisation/Lecture/Vitesse/slider_step.md)

#### 5.4.2. ZoomDynamique
**Description** : Système de zoom intelligent qui s'adapte à la vitesse de vol.
*   [Intensité du zoom dynamique](../DocParametrage/Visualisation/Lecture/ZoomDynamique/intensite_zoom_dynamique.md)

#### 5.4.3. Comportement
**Description** : Réglages d'ergonomie et aides visuelles en cours de lecture.
*   [Vitesse de l'animation (ms/km)](../DocParametrage/Visualisation/Lecture/vitesse.md)
*   [Durée de la reprise après pause (ms)](../DocParametrage/Visualisation/Lecture/timerReprisePause.md)
*   [Délai avant de masquer le curseur (ms)](../DocParametrage/Visualisation/Lecture/masquerCurseurDelai.md)
*   [Afficher la croix centrale en pause](../DocParametrage/Visualisation/Lecture/afficherCroixCentrale.md)
*   [Couleur de la croix centrale](../DocParametrage/Visualisation/Lecture/couleurCroixCentrale.md)
*   [Zoom Minimum](../DocParametrage/Visualisation/Lecture/zoomMinimum.md)

### 5.5. Finalisation
**Description** : Actions automatiques déclenchées lorsque le marqueur atteint l'arrivée.
*   [Durée de la pause à l'arrivée](../DocParametrage/Visualisation/Finalisation/delayAfterAnimationEnd.md)
*   [Durée vers la trace à l'arrivée](../DocParametrage/Visualisation/Finalisation/flyToGlobalDuration.md)
*   [Durée vers le départ (reprise)](../DocParametrage/Visualisation/Finalisation/flyToKm0Duration.md)
*   [Reprise automatique](../DocParametrage/Visualisation/Finalisation/repriseAutomatique.md)
*   [Durée de la pause avant reprise auto](../DocParametrage/Visualisation/Finalisation/pauseAvantReprise.md)

### 5.6. Profil Altitude
**Description** : Configuration graphique de la vue en coupe du terrain.

#### 5.6.1. Graphe
**Description** : Échelles des axes et fréquence des annotations de distance/altitude.
*   [Echelle des abscisses](../DocParametrage/Visualisation/Profil%20Altitude/Graphe/Abscisse.md)
*   [Echelle des ordonnées](../DocParametrage/Visualisation/Profil%20Altitude/Graphe/Ordonnee.md)
*   [Intervalle des repères distances](../DocParametrage/Visualisation/Profil%20Altitude/Graphe/RepereDistance.md)
*   [Intervalle des repères altitude](../DocParametrage/Visualisation/Profil%20Altitude/Graphe/RepereAltitude.md)
*   [Position du curseur](../DocParametrage/Visualisation/Profil%20Altitude/Graphe/CurseurPositionKm.md)
*   [Aspect curseur lié à la comète](../DocParametrage/Visualisation/Profil%20Altitude/Graphe/aspectCurseurLieComete.md)

#### 5.6.2. Couleurs de pente
**Description** : Attribution des couleurs selon le pourcentage d'inclinaison.
*   [Pente <= 0%](../DocParametrage/Visualisation/Profil%20Altitude/Couleurs/TrancheNegative.md)
*   [0% < Pente < 3%](../DocParametrage/Visualisation/Profil%20Altitude/Couleurs/Tranche1.md)
*   [3% <= Pente < 6%](../DocParametrage/Visualisation/Profil%20Altitude/Couleurs/Tranche2.md)
*   [6% <= Pente < 9%](../DocParametrage/Visualisation/Profil%20Altitude/Couleurs/Tranche3.md)
*   [9% <= Pente < 12%](../DocParametrage/Visualisation/Profil%20Altitude/Couleurs/Tranche4.md)
*   [Pente >= 12%](../DocParametrage/Visualisation/Profil%20Altitude/Couleurs/Tranche5.md)

### 5.7. Taille des Messages
**Description** : Réglages de la lisibilité des bulles de texte sur la carte.
*   ⚠️ [Taille de police de base](../DocParametrage/Visualisation/Taille%20des%20Messages/baseFontSize.md)
*   ⚠️ [Coefficient largeur message](../DocParametrage/Visualisation/Taille%20des%20Messages/coefLargeurMessage.md)
*   ⚠️ [Coefficient réducteur message](../DocParametrage/Visualisation/Taille%20des%20Messages/coefReducteurMessage.md)

### 5.8. Météo
**Description** : Gestion des données météorologiques et de l'orientation sur le parcours.
*   [Activer la météo](../DocParametrage/Visualisation/Meteo/meteoActif.md)
*   [Heure de départ par défaut](../DocParametrage/Visualisation/Meteo/heureDepart.md)
*   [Heure de début de journée](../DocParametrage/Visualisation/Meteo/heureDebutJournee.md)
*   [Heure de fin de journée](../DocParametrage/Visualisation/Meteo/heureFinJournee.md)
*   [Vitesse moyenne par défaut (km/h)](../DocParametrage/Visualisation/Meteo/vitesseMoyenne.md)

#### 5.8.1. Widgets
**Description** : Gadgets visuels pour la météo et la navigation.
*   [Information Météo](../DocParametrage/Visualisation/Meteo/Widgets/informationMeteo.md)
*   [Boussole](../DocParametrage/Visualisation/Meteo/Widgets/boussole.md)
*   [Orientation de la boussole](../DocParametrage/Visualisation/Meteo/Widgets/orientationBoussole.md)

---

## 6. ⚙️ Système
**Description** : Paramètres internes, gestion du matériel et services de communication.

### 6.1. Fenêtre et Écran
**Description** : Configuration de l'affichage de l'application sur votre poste de travail.
*   [Taille fenêtre](../DocParametrage/Systeme/tailleFenetre.md)
*   [Ecran](../DocParametrage/Systeme/ecran.md)

### 6.2. Timers et Tokens
**Description** : Paramètres réseau et clés d'accès aux services cartographiques.
*   ⚠️ [Interval de vérification du réseau](../DocAnnexe/polling_timer_documentation.md)
*   ⚠️ [Token Mapbox](../DocAnnexe/obtenir_token_mapbox.md)

### 6.3. Télécommande
**Description** : Configuration du serveur de communication et sensibilité des contrôles mobiles.
*   ⚠️ [Port du serveur](../DocParametrage/Systeme/Telecommande/Port.md)
*   [Sensibilité du point de vue (X)](../DocParametrage/Systeme/Telecommande/sensibilitePointDeVueX.md)
*   [Sensibilité du point de vue (Y)](../DocParametrage/Systeme/Telecommande/sensibilitePointDeVueY.md)
*   [Sensibilité du cap](../DocParametrage/Systeme/Telecommande/sensibiliteCap.md)
*   [Sensibilité du zoom](../DocParametrage/Systeme/Telecommande/sensibiliteZoom.md)
*   [Sensibilité de l'inclinaison (tilt)](../DocParametrage/Systeme/Telecommande/sensibiliteTilt.md)

---

