# Foire Aux Questions (FAQ)

Réponses aux questions les plus fréquentes sur l'utilisation de VisuGPS.

[< Retour à l'accueil](./index.md)

## Général

**Q : L'application est-elle gratuite ?**
R : VisuGPS est une application open-source sous licence Apache 2.0.

**Q : Sur quels systèmes fonctionne-t-elle ?**
R : VisuGPS est compatible avec Windows, macOS et Linux.

## Carte 3D et Mapbox

**Q : La carte ne s'affiche pas ou reste noire.**
R : Vérifiez votre connexion internet. Si vous êtes connecté, assurez-vous d'avoir saisi un **Token Mapbox** valide dans les [Paramètres](./parametres.md).

**Q : Pourquoi le relief semble plat ?**
R : Le chargement du relief (terrain DEM) nécessite une connexion internet et un Token Mapbox valide. Vérifiez également que le facteur d'exagération du relief n'est pas à 0 dans les options (si disponible).

## Animation

**Q : La caméra bouge de façon saccadée.**
R : En mode "Édition", assurez-vous d'avoir suffisamment de points clés (Keyframes) mais pas trop rapprochés. Une transition trop rapide entre deux angles très différents peut être désagréable.

**Q : Mon tracé semble passer sous terre.**
R : Cela arrive si les données d'altitude du GPX sont imprécises. L'application tente de corriger cela avec le "Map Matching" ou en récupérant les altitudes via Mapbox. Vous pouvez essayer de réimporter la trace.

## Télécommande

**Q : Le QR Code ne s'affiche pas.**
R : L'application n'arrive peut-être pas à déterminer votre adresse IP locale. Vérifiez votre connexion réseau.

**Q : "Impossible de se connecter" sur le mobile.**
R : C'est souvent un blocage pare-feu. Autorisez VisuGPS dans le pare-feu de votre ordinateur pour le port 9001 (TCP).

[< Retour à l'accueil](./index.md)
