# Mode Visualisation

Cette section décrit comment exploiter le mode Visualisation, l'aboutissement de votre travail où l'animation 3D prend vie selon vos réglages de caméra et d'événements.

[< Retour à l'accueil](./index.md) | [< Précédent : Édition](./edition_messages.md)

## 1. Déroulement de l'Animation

L'animation suit une séquence automatisée fluide :

1.  **Lancement** : Transition (Flyto) depuis une vue d'ensemble de l'Europe vers votre trace.
2.  **Introduction** : Zoom progressif vers le point de départ (km 0).
3.  **Lecture** : Le marqueur (comète) parcourt la trace. La caméra suit fidèlement les mouvements définis en mode Édition.
4.  **Événements** :
    *   **Pauses** : L'animation s'arrête (voir [Panel de Contrôle] ci-dessous pour reprendre).
    *   **Survols** : La caméra s'échappe temporairement vers un point d'intérêt avant de revenir sur la trace.
    *   **Messages** : Apparition des bulles informatives et des marqueurs KM.
5.  **Final** : Une fois l'arrivée atteinte, la caméra effectue un zoom arrière pour montrer l'intégralité du parcours.

## 2. Interface et Contrôles

### Panel de Contrôle (Bas de l'écran)
Un menu escamotable permet de piloter la lecture :
*   **Play / Pause** <img src="https://api.iconify.design/mdi/play.svg?width=20" style="vertical-align: middle;"> <img src="https://api.iconify.design/mdi/pause.svg?width=20" style="vertical-align: middle;"> : Interrompt ou reprend le mouvement.
*   **Rembobinage** <img src="https://api.iconify.design/mdi/rewind.svg?width=20" style="vertical-align: middle;"> : Permet de reculer rapidement sur la trace.
*   **Vitesse (Slider)** : Ajustez la vitesse de progression en temps réel (ex: de 0.5x à 10x). Le bouton **1x** <img src="https://api.iconify.design/mdi/numeric-1-box-outline.svg?width=20" style="vertical-align: middle;"> permet de revenir instantanément à la vitesse normale.
*   **Reset** <img src="https://api.iconify.design/mdi/reload.svg?width=20" style="vertical-align: middle;"> : Permet de tout recommencer une fois l'arrivée atteinte.

### Indicateurs Temps Réel
*   **Widget Distance** : Affiche le kilomètre actuel par rapport au total.
*   **Widget Commune** : Affiche en haut de l'écran le nom de la commune traversée.
*   **Profil d'Altitude** : Un graphique dynamique au bas de l'écran montre votre position sur le relief.
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
| **R** | Recommencer depuis le début (Reset) |
| **H** | Afficher / Masquer le bouton de retour à l'accueil |
| **Flèche Gauche** | Rembobinage (Maintenir pour reculer) |
| **Flèche Haut/Bas** | Ajuster la vitesse de lecture |
| **Suppr** | Masquer tous les éléments d'interface (Mode épuré) |

---

## 4. Astuce : Interaction pendant la Pause
Si vous mettez l'animation en **Pause**, vous restez libre de manipuler la caméra (avec la souris ou la télécommande). Lors de la reprise (**Play**), l'application effectuera une transition fluide pour replacer la caméra sur sa trajectoire programmée.

---

## 5. Paramètres Liés

L'apparence et le comportement des widgets dépendent de certains paramètres globaux (accessibles dans le menu [<span style="color: #FFC107">Paramètres</span>](./parametres.md) > Visualisation > Widgets) :

*   **Distance** : Afficher ou masquer le widget de distance au démarrage. (voir [Distance](../DocParametrage/Visualisation/Widgets/distance.md))
*   **Communes** : Afficher ou masquer le widget des communes au démarrage. (voir [Communes](../DocParametrage/Visualisation/Widgets/communes.md))
*   **Altitude** : Afficher ou masquer le profil d'altitude au démarrage. (voir [Altitude](../DocParametrage/Visualisation/Widgets/altitude.md))
*   **Commandes** : Afficher ou masquer le panneau de commandes au démarrage. (voir [Commandes](../DocParametrage/Visualisation/Widgets/commandes.md))

---
[< Retour à l'accueil](./index.md) | [Suivant : <span style="color: #FFC107">Paramètres</span> >](./parametres.md)
