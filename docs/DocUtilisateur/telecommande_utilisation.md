# Guide d'Utilisation de l'Interface Télécommande

L'interface de la télécommande s'adapte automatiquement à ce qui est affiché sur l'écran principal de VisuGPS.
Voici une présentation graphique complète de chaque mode avec le détail des composants.

---

## 1. Vue Accueil (Mode Connexion)
Cette vue s'affiche au lancement de l'application ou lorsqu'aucune visualisation n'est active sur le PC.

### Vue Globale de l'IHM
<p align="center">
  <img src="../images/remote_accueil.svg" alt="Mockup Accueil">
</p>

### Composants détaillés

#### Barre d'entête
*   **Logo & Titre** : Rappel visuel de l'application VisuGPS.
*   **Sélecteur de Thème** : Icons Lune <img src="https://api.iconify.design/mdi/weather-night.svg?color=grey&width=20" style="vertical-align: middle;"> et Soleil <img src="https://api.iconify.design/mdi/white-balance-sunny.svg?color=orange&width=20" style="vertical-align: middle;"> pour basculer instantanément entre le mode sombre et le mode clair.

#### Traces Favorites
*   **Liste Dynamique** : Affiche uniquement les circuits favoris pour pouvoir les visualiser depuis la télécommande.
*   **Action Rapide** : Appuyez sur une trace pour déclencher son chargement et passer en mode Visualisation sur l'ordinateur.

#### Actions (Bas d'écran)
*   **Bouton Déconnexion** : Permet de couper proprement la liaison avec le PC.

---

## 2. Vue Animation (Lecture en cours)
Cette vue s'active dès que l'animation "Play" est lancée. Elle est optimisée pour le pilotage en direct.

### Vue Globale de l'IHM
<p align="center">
  <img src="../images/remote_animation.svg" alt="Mockup Animation">
</p>

### Composants détaillés

#### Barre de Lecture (Haut)
| Composant | Icône | Description |
| :--- | :---: | :--- |
| **Bouton Recul** | <img src="https://api.iconify.design/mdi/rewind.svg?color=black&width=24"> | Maintenez pour remonter le temps sur la trace. |
| **Bouton Pause** | <img src="https://api.iconify.design/mdi/pause.svg?color=black&width=24" style="border-radius:50%; padding:5px;"> | Arrête l'animation et bascule vers la **Vue Pause**. |
| **Reset x1** | **x1** | Réinitialise la vitesse de lecture à la valeur normale (1.0x). |

#### Réglette de Vitesse
*   **Fonction** : Ajuste la vitesse de déplacement sur une échelle logarithmique (de 0.1x à 10x).


#### Grille des Widgets
Chaque bouton active ou désactive un widget sur la visualisation 3D.
*   <img src="https://api.iconify.design/mdi/city.svg?color=green&width=20"> **Villes** : Nom des communes traversées.
*   <img src="https://api.iconify.design/mdi/counter.svg?color=green&width=20"> **Distance** : Compteur kilométrique.
*   <img src="https://api.iconify.design/mdi/sun-clock-outline.svg?color=green&width=20"> **Météo** : Conditions météorologiques le long du parcours.
*   <img src="https://api.iconify.design/mdi/chart-areaspline-variant.svg?color=green&width=20"> **Altitude** : Graphique de dénivelé.
*   <img src="https://api.iconify.design/mdi/movie-play-outline.svg?color=green&width=20"> **Commandes** : Aide visuelle des touches.
*   <img src="https://api.iconify.design/mdi/compass-outline.svg?color=green&width=20"> **Boussole** : Orientation et force du vent.

---

## 3. Vue Pause (Mode Exploration)
Cette vue s'active lors d'une pause. Elle permet :
*   de manipuler librement la caméra 3D.
*   de sélectionner les circuits alternatifs (Variante).

### Vue Globale de l'IHM
<p align="center">
  <img src="../images/remote_pause.svg" alt="Mockup Pause">
</p>

### Composants détaillés

#### Navigation (Haut)
| Composant | Icône | Description |
| :--- | :---: | :--- |
| **Bouton Recul** | <img src="https://api.iconify.design/mdi/rewind.svg?color=black&width=24"> | Permet de remonter le temps manuellement pendant la pause. |
| **Bouton Lecture** | <img src="https://api.iconify.design/mdi/play.svg?color=black&width=24"> | Reprend l'animation et revient à la **Vue Animation**. |
| **Vue Finale** | <img src="https://api.iconify.design/mdi/clock-end.svg?color=black&width=24"> | Déplace la caméra directement vers le point de vue final. |

#### Contrôles de Précision
*   <img src="https://api.iconify.design/mdi/cursor-move.svg?color=black&width=24"> **Déplacement (Trackpad central)** : Glissez votre doigt dans le grand rectangle central pour déplacer la caméra horizontalement (Nord, Sud, Est, Ouest).
*   <img src="https://api.iconify.design/mdi/magnify-plus-outline.svg?color=black&width=24"> **Zoom (Gauche)** : Glissez de haut en bas pour changer l'altitude de caméra.
*   <img src="https://api.iconify.design/mdi/compass-outline.svg?color=black&width=24"> **Rotation (Centre)** : Glissez horizontalement pour faire pivoter la vue (Cap).
*   <img src="https://api.iconify.design/mdi/angle-acute.svg?color=black&width=24"> **Inclinaison (Droite)** : Glissez verticalement pour changer l'angle de plongée (Tilt).

#### Actions Fondamentales
*   <img src="https://api.iconify.design/mdi/home.svg?color=black&width=24"> **Maison** : Quitte la trace en cours pour revenir au menu principal de l'application.
*   <img src="https://api.iconify.design/mdi/map-marker-distance.svg?color=black&width=24"> **Variante** : Ouvre le menu de sélection des parcours alternatifs ou revient à la trace principale.

---

## 4. Navigation par Segments (Mode Variante)
Ce bandeau s'affiche en bas de l'écran (sur les vues Animation et Pause) uniquement lors de la visualisation d'une **Variante**. Il permet de visualiser le découpage du parcours et de naviguer instantanément entre les sections.

### Les Icônes de Segments
| Composant | Icône | Signification |
| :--- | :---: | :--- |
| **Départ** | <img src="https://api.iconify.design/mdi/ray-start-arrow.svg?color=green&width=24"> | Point de départ du circuit. |
| **Commun** | <img src="https://api.iconify.design/mdi/link-variant.svg?color=black&width=24"> | Tronçon commun partagé entre plusieurs tracés. |
| **Segment** | <img src="https://api.iconify.design/mdi/map-marker-path.svg?color=blue&width=24"> | Portion de tracé spécifique à la variante sélectionnée. |
| **Arrivée** | <img src="https://api.iconify.design/mdi/ray-end-arrow.svg?color=red&width=24"> | Point d'arrivée final du circuit. |

### Fonctionnement
*   **Indication visuelle** : L'étape actuelle est mise en avant (icône en surbrillance).
*   **Saut Rapide** : Appuyez sur n'importe quelle icône du bandeau pour déplacer instantanément la caméra au début du segment sélectionné (déclenche un repositionnement automatique sur la vue 3D). Les segments communs ne sont pas sélectionnables.
*   **Progression** : Le bandeau se met à jour en temps réel au fur et à mesure que l'animation avance.

---

## 5. Paramètres associés

Le comportement de la télécommande et l'affichage des circuits peuvent être personnalisés dans les réglages de l'application VisuGPS (sur le PC).

### Configuration de la Télécommande
Ces paramètres se trouvent dans **Système > Télécommande** :

| Paramètre | Description | Défaut |
| :--- | :--- | :---: |
| **Port du serveur** | Port réseau utilisé pour la communication (WebSocket/SSE). | 9001 |
| **Sensibilité (X / Y)** | Ajuste la vitesse de déplacement sur le trackpad central. | 200 |
| **Sensibilité du Zoom** | Influence la vitesse de changement d'altitude. | 150 |
| **Sensibilité du Cap** | Ajuste la vitesse de rotation (pivotement). | 50 |
| **Sensibilité Tilt** | Ajuste la vitesse d'inclinaison de la vue. | 25 |

### Affichage de l'Accueil
Puisque la télécommande affiche uniquement les favoris, vous pouvez ajuster leur nombre dans **Accueil** :

| Paramètre | Description | Défaut |
| :--- | :--- | :---: |
| **Nombre de favoris** | Définit la quantité maximale de circuits affichés sur la télécommande. | 6 |
