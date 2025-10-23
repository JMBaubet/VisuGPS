  Documentation de l'Animation de Visualisation 3D

  Cette documentation décrit les différentes phases de l'animation "fly-by" lors de la visualisation d'un circuit, ainsi que les paramètres de configuration associés.

  1. Séquence d'Initialisation

  L'animation ne commence pas directement sur la trace. Elle suit une séquence cinématique pour une expérience plus immersive.

  Phase 1.1 : Vue Globale de l'Europe

   * Description : L'animation démarre sur une vue large de l'Europe.
   * Paramètres :
       * Visualisation/Initialisation/centerEurope : Coordonnées [longitude, latitude] du point de centrage initial.
       * Visualisation/Initialisation/zoomEurope : Niveau de zoom pour la vue de l'Europe.

  Phase 1.2 : Survol vers la Trace (Fly-to)

   * Description : La caméra effectue un survol rapide depuis la vue de l'Europe jusqu'à un aperçu global de la trace GPX à visualiser. La caméra se positionne pour que
     l'intégralité de la trace soit visible à l'écran.
   * Paramètres :
       * Visualisation/Initialisation/durationEuropeToTrace : Durée de cette animation de survol, en millisecondes.

  Phase 1.3 : Pause sur la Vue Globale

   * Description : Une fois arrivée sur la vue globale de la trace, la caméra marque une courte pause pour permettre à l'utilisateur d'appréhender le parcours.
   * Paramètres :
       * Visualisation/Initialisation/pauseBeforeStart : Durée de la pause, en millisecondes.

  Phase 1.4 : Survol vers le Point de Départ

   * Description : La caméra effectue un dernier survol pour se positionner au point de départ (kilomètre 0) de la trace, adoptant les paramètres de caméra (zoom, pitch,
     cap) définis pour ce point dans le fichier tracking.json.
   * Paramètres :
       * Visualisation/Initialisation/durationTraceToStart : Durée de cette animation de survol, en millisecondes.

  Phase 1.5 : Pause au Départ

   * Description : Avant que le suivi de la trace ne commence, une dernière pause est observée au point de départ. L'animation principale démarre automatiquement à la fin de
      cette pause.
   * Paramètres :
       * Visualisation/Initialisation/pauseAuKm0 : Durée de la pause au départ, en secondes.

  2. Animation Principale (Suivi de la Trace)

  C'est le cœur de l'animation, où la caméra suit la progression le long du parcours.

  Phase 2.1 : Déplacement de la Caméra

   * Description : La caméra se déplace le long de la trace. Son comportement est dicté par le fichier tracking.json :
       1. Entre les points de contrôle : La position, le zoom, le pitch et le cap de la caméra sont interpolés de manière linéaire entre deux points de contrôle consécutifs
          pour assurer une transition fluide.
       2. En dehors des segments de contrôle : Si la progression n'est pas entre deux points de contrôle (par exemple, après le dernier), la caméra suit les valeurs (zoom,
          pitch, cap) définies pour chaque point individuel du fichier tracking.json, avec une interpolation entre chaque point pour éviter les saccades.
   * Paramètres :
       * Visualisation/Animation/vitesse : Vitesse de l'animation, définie en millisecondes par kilomètre. Une valeur plus faible signifie une animation plus rapide.

  Phase 2.2 : Affichage de la Trace et de la "Comète"

   * Description : La trace GPX est affichée sur la carte. Pour matérialiser la progression, un effet de "comète" suit le point d'avancement. La trace déjà parcourue reste
     visible, tandis que la comète illumine le segment en cours.
   * Paramètres :
       * Visualisation/Mapbox/Traces/couleurTrace : Couleur de la trace principale.
       * Visualisation/Mapbox/Traces/epaisseurTrace : Épaisseur de la trace principale.
       * Visualisation/Mapbox/Traces/opaciteTrace : Opacité de la trace principale.
       * Visualisation/Mapbox/Traces/couleurComete : Couleur de la comète.
       * Visualisation/Mapbox/Traces/epaisseurComete : Épaisseur de la comète.
       * Visualisation/Mapbox/Traces/opaciteComete : Opacité de la comète.
       * Visualisation/Mapbox/Traces/longueurComete : Longueur de la comète, en mètres.

  Phase 2.3 : Gestion des Événements Programmés

   * Description : Pendant le parcours, des événements définis dans le fichier evt.json peuvent se déclencher.
       * Pauses : L'animation se met automatiquement en pause lorsqu'elle atteint un point de pause programmé.
       * Survols (Fly-to) : L'animation se met en pause, la caméra effectue un survol vers un point d'intérêt défini, attend que l'utilisateur relance manuellement, puis
         revient à sa position sur la trace pour continuer.
       * Messages : Des pop-ups d'information s'affichent à des coordonnées géographiques précises pour une durée déterminée.

  3. Séquence de Finalisation

  Une fois que la progression atteint la fin de la trace.

  Phase 3.1 : Pause à l'Arrivée

   * Description : L'animation marque une pause à la fin du parcours.
   * Paramètres :
       * Visualisation/Finalisation/delayAfterAnimationEnd : Durée de la pause à l'arrivée, en secondes.

  Phase 3.2 : Retour à la Vue Globale

   * Description : Après la pause, la caméra effectue un survol arrière pour revenir à une vue globale de l'ensemble de la trace.
   * Paramètres :
       * Visualisation/Finalisation/flyToGlobalDuration : Durée de cette animation de survol, en millisecondes.

  Phase 3.3 : Reprise Automatique (Optionnel)

   * Description : Si ce mode est activé, l'animation redémarre automatiquement depuis le début après une courte pause sur la vue globale.
   * Paramètres :
       * Visualisation/Finalisation/repriseAutomatique : Active ou désactive la reprise automatique (booléen).
       * Visualisation/Finalisation/pauseAvantReprise : Durée de la pause sur la vue globale avant de relancer l'animation, en secondes.

