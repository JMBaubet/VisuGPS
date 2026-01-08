# Mode Visualisation

Cette section décrit comment exploiter le mode Visualisation, l'aboutissement de votre travail où l'animation 3D prend vie selon vos réglages de caméra et d'événements.

[< Retour au guide d'exploitation](./exploitation.md)


## 1. Déroulement de l'Animation

L'animation suit une séquence automatisée fluide :

1.  **Lancement** : Transition (Flyto) depuis une vue d'ensemble de l'Europe vers votre trace. La carte utilise alors le **style de lancement** (ex: Mapbox Standard) pour une clarté maximale à grande échelle.
2.  **Introduction** : Zoom progressif vers le point de départ (km 0). Pendant cette phase, le style de carte bascule automatiquement vers le **style détaillé** (ex: Satellite) défini pour la trace.
3.  **Lecture** : Le marqueur (comète) parcourt la trace. La caméra suit fidèlement les mouvements définis en mode Édition.
4.  **Événements** :
    *   **Pauses** : L'animation s'arrête (voir [Panel de Contrôle] ci-dessous pour reprendre).
    *   **Survols** : La caméra s'échappe temporairement vers un point d'intérêt avant de revenir sur la trace.
    *   **Messages** : Apparition des bulles informatives et des marqueurs KM.
5.  **Final** : Une fois l'arrivée atteinte, la caméra effectue un zoom arrière pour montrer l'intégralité du parcours. Le style de carte repasse alors automatiquement en **style de lancement** pour une vue globale épurée.

## 2. Interface et Contrôles

### Panel de Contrôle (Bas de l'écran)
Un menu escamotable permet de piloter la lecture :
*   **Play / Pause** <img src="https://api.iconify.design/mdi/play.svg?width=20" style="vertical-align: middle;"> <img src="https://api.iconify.design/mdi/pause.svg?width=20" style="vertical-align: middle;"> : Interrompt ou reprend le mouvement.
*   **Rembobinage** <img src="https://api.iconify.design/mdi/rewind.svg?width=20" style="vertical-align: middle;"> : Permet de reculer rapidement sur la trace.
*   **Vitesse (Slider)** : Ajustez la vitesse de progression en temps réel (ex: de 0.5x à 10x). Le bouton **1x** <img src="https://api.iconify.design/mdi/numeric-1-box-outline.svg?width=20" style="vertical-align: middle;"> permet de revenir instantanément à la vitesse normale.
*   **Reset** <img src="https://api.iconify.design/mdi/reload.svg?width=20" style="vertical-align: middle;"> : Permet de tout recommencer une fois l'arrivée atteinte.

### Indicateurs Temps Réel
*   **Widget Distance** : 
<span style="display: inline-flex; align-items: center; justify-content: center; height: 32px; background-color: white; border-radius: 4px; box-shadow: 0 2px 4px rgba(0,0,0,0.2); color: black; font-weight: bold; font-family: sans-serif; padding: 0 12px; vertical-align: middle; margin: 0 5px;">Distance : 12.56 / 83.20 km</span> 
Affiche le kilomètre actuel par rapport au total.
####
*   **Widget Commune** : 
<span style="display: inline-flex; align-items: center; justify-content: center; min-width: 100px; height: 28px; background-color: white; border: 3px solid #c62828; border-radius: 4px; color: black; font-weight: bold; font-family: sans-serif; padding: 0 10px; vertical-align: middle; margin: 0 5px;">Fréjus</span> 
Affiche en haut à gauche de l'écran le nom de la commune traversée.
####
*   **Profil d'Altitude** : Un graphique dynamique au bas de l'écran montre votre position sur le relief. 
    >
    > Le curseur de progression peut être personnalisé via le paramètre [Aspect curseur lié à la comète](./parametres.md#461-graphe) pour reprendre l'apparence de la comète.
####
*   **Information Météo** : 
<span style="display: inline-flex; flex-direction: column; background: white; border-radius: 4px; padding: 6px; box-shadow: 0 2px 5px rgba(0,0,0,0.2); font-family: sans-serif; min-width: 150px; vertical-align: middle; margin: 0 5px; color: black;"><span style="display: block; font-size: 0.65em; font-weight: bold; text-align: left; margin-bottom: 4px; padding-left: 2px;">Lun 01 Jan 2024</span><span style="display: flex; align-items: center; justify-content: space-between;"><span style="display: flex; flex-direction: column; margin-right: 8px;"><span style="display: flex; align-items: center; font-size: 0.7em; font-weight: bold;"><span>Gr. 1</span><svg viewBox="0 0 24 24" width="10" height="10" style="margin-left: 2px; display: block;"><path d="M12,17.27L18.18,21L16.54,13.97L22,9.24L14.81,8.62L12,2L9.19,8.62L2,9.24L7.45,13.97L5.82,21L12,17.27Z" fill="#1976D2" /></svg></span><span style="font-size: 0.6em; color: #666;">09:30</span></span><span style="display: flex; align-items: center; font-size: 0.85em;"><img src="https://api.iconify.design/mdi/weather-sunny.svg?color=orange&width=16" style="margin-right: 4px;"><span style="font-weight: bold; margin-right: 4px; color: #2E7D32;">20°</span><span style="display: flex; align-items: center; font-size: 0.8em; color: #455A64; margin-right: 4px;"><img src="https://api.iconify.design/mdi/water.svg?color=blue&width=12"> 0% (0mm)</span><span style="display: flex; align-items: center; font-size: 0.8em; color: #666;"><img src="https://api.iconify.design/mdi/navigation.svg?color=grey&width=12" style="transform: rotate(135deg);"> 12</span></span></span></span> 
Widget dynamique affichant les conditions météo au point de passage (Température, Vent, Pluie) selon le scénario choisi. Nous pouvons avoir l'affichage de plusieurs groupes (voir [gestionaire météo](./meteo_manager.md)).

*   **Boussole** : 
<span style="display: inline-flex; flex-direction: column; align-items: center; width: 60px; vertical-align: middle; margin: 0 5px;"><span style="display: block; width: 60px; height: 60px;"><svg viewBox="0 -12 100 112" width="100%" height="100%"><path d="M50 -12 L44 -2 L56 -2 Z" fill="#2196F3" /><g transform="rotate(-20, 50, 50)"><circle cx="50" cy="50" r="48" fill="black" stroke="white" stroke-width="2" /><text x="50" y="16" fill="#F44336" font-family="Arial" font-size="14" font-weight="bold" text-anchor="middle" dominant-baseline="central">N</text><text x="50" y="86" fill="white" font-family="Arial" font-size="12" font-weight="bold" text-anchor="middle" dominant-baseline="central" transform="rotate(180, 50, 86)">S</text><text x="86" y="50" fill="white" font-family="Arial" font-size="12" font-weight="bold" text-anchor="middle" dominant-baseline="central" transform="rotate(90, 86, 50)">E</text><text x="14" y="50" fill="white" font-family="Arial" font-size="12" font-weight="bold" text-anchor="middle" dominant-baseline="central" transform="rotate(-90, 14, 50)">O</text><g transform="rotate(10, 50, 50)"><path d="M50 2 L44 12 L56 12 Z" fill="white" /></g><g transform="rotate(160, 50, 50)"><path d="M 46 2 L 54 2 L 50 35 Z" fill="#4CAF50" /></g></g></svg></span><span style="display: block; color: #4CAF50; font-size: 0.8em; font-weight: bold; font-family: sans-serif; text-align: center; line-height: 1.1; margin-top: -5px;">12 <span style="font-size: 0.8em; opacity: 0.8;">(18)</span><span style="font-size: 0.7em;"> km/h</span></span></span> 
Indique l'orientation de la caméra ou de la trace du vent ainsi que sa vitesse. 
La longueur de l'indicateur de vent dépend de sa vitesse. Sa couleur dépend de son orientation par rapport à la trace.
    Le triangle bleu représente le cap de la trace. 
    Le triangle blanc représente le cap de la caméra. 
    Les données visualisées correspondent aux données du groupe sur lequel nous avons le symbole <span style="color: #1976D2; font-weight: bold; font-size: 1.2em;">*</span>. 

*   **Croix Centrale** : En mode Pause, une croix peut s'afficher pour indiquer le centre de rotation de la caméra.

### Télécommande Mobile
Vous pouvez piloter intégralement votre visualisation depuis votre smartphone (Lecture, Vitesse, Affichage des widgets, etc.).
*   **[Connexion et Couplage](./telecommande_couplage.md)** : Comment appairer votre mobile.
*   **[Utilisation de la Télécommande](./telecommande_utilisation.md)** : Liste des commandes disponibles sur mobile.

## 3. Raccourcis Clavier

| Touche | Action |
| :--- | :--- |
| **P** | Met en pause / reprend l'animation |
| **Espace** | Afficher / Masquer le panel de contrôle |
| **A** | Afficher / Masquer le profil d'Altitude |
| **D** | Afficher / Masquer le Widget Distance |
| **C** | Afficher / Masquer le Widget Commune |
| **M** | Afficher / Masquer l'Information Météo |
| **Ctrl + M** | Afficher / Masquer le Tableau Météo |
| **B** | Afficher / Masquer la Boussole |
| **Shift + B** | Changer le mode d'orientation de la boussole |
| **R** | Recommencer depuis le début (Reset) |
| **H** | Afficher / Masquer le bouton de retour à l'accueil |
| **Flèche Gauche** | Rembobinage (Maintenir pour reculer) |
| **Flèche Haut/Bas** | Ajuster la vitesse de lecture |
| **Suppr** | Masquer tous les éléments d'interface (Mode épuré) |

---

## 4. Astuce : Interaction pendant la Pause
Si vous mettez l'animation en **Pause**, vous restez libre de manipuler la caméra (avec la souris ou la télécommande). Lors de la reprise (**Play**), l'application effectuera une transition fluide pour replacer la caméra sur sa trajectoire programmée.

---

### 🛠️ Paramètres Liés
Retrouvez les réglages détaillés associés à cette fonctionnalité dans la section :
* [4. 🟡 Visualisation](./parametres.md#4--visualisation)

---
[< Retour au guide d'exploitation](./exploitation.md)
