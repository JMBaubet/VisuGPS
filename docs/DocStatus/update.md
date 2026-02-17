# Evolutions de VisuGPS

## Version 1.0.0
Version initiale.
Contient les fonctionnalités suivantes :
- Importation des fichiers GPX
- Edition de la visualisation 3D avec :
    - Poistionnement de la caméra,
    - Ajout de Pause et de flyto le long de la trace
    - Ajout de messages le long de la trace
- Intégration des communes traversées
- Intégration des données météo
- Gestion de groupes (horaire de départ et moyenne)
- Visualisation 3D de la trace avec les données suivantes :
    - Communes traversées
    - Distance parcourue / Distance totale
    - Données météo (vent température pluviosité)
    - Profil altimétrique
- Télécommande de contrôle de la visualisation 3D
- Documentation en ligne
- Exportation/Importation des circuits

## Version 1.1.0

Contient les fonctionnalités suivantes :
- Ajout de variantes
- Amélioration de la télécommande

## Version 1.1.1

Contient les fonctionnalités suivantes :
- Refonte complète de la vue Paramètres avec onglets verticaux imbriqués
- Ajout de l'accès à la vue DebugTracking en mode production

Corrections :

- Ajout de la restauration d'environnement et de l'édition des modes d'exécution
- Amélioration de la détection des segments aller-retour
- Résolution des accès concurrents sur tracking.json