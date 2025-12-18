# Mode Édition : Vue d'ensemble

Le mode Édition est l'espace de travail où vous configurez votre trace et préparez votre visalisation 3D. 
Cet espace est organisé autour de trois modes d'Édition complémentaires accessibles via des onglets : 
* Caméra, (voir [Introduction à la Caméra](./introduction_camera.md))
* Pause/FlyTo, (voir [Événements : Pauses et FlyTo](./edition_flyto_pause.md))
* Message. (voir [Gestion des Messages](./edition_messages.md))



## Interface du Mode Édition

L'écran est optimisé pour la manipulation visuelle :
1.  **le bouton flèche gauche** ![back](https://api.iconify.design/mdi/arrow-left.svg?width=20) (Haut à gauche)      : permet de revenir à la page d'accueil
2.  **La Vue 3D** (En fond) : Affiche votre trace sur le relief Mapbox. C'est votre zone de travail principale.
3.  **Indicateur de Progression** (Haut au centre) : Affiche la distance parcourue (km) et la progression globale sur la trace.
4.  **Paramètres de Caméra** (Haut à droite) : Affiche en temps réel le **Cap** (angle), le **Zoom** et le **Pitch** (inclinaison verticale).
5.  **Panneau de Contrôle** (Bas) : Regroupe les Graphes de données et les Onglets de réglage.

---

## Navigation Globale

Voici comment vous déplacer le long de votre trace :

### Au Clavier
*   **Flèches Gauche / Droite** : Avancer ou reculer sur la trace (Incrémentation de 100 m par défaut).
*   **Maj (Shift) + Flèches** : Déplacement rapide (Incrément de 1 km par défaut).

> [!NOTE]
> Si votre clavier ne possède pas de touches fléchées (ou si vous préférez d'autres touches), vous pouvez les redéfinir dans les [<span style="color: #FFC107">Paramètres</span>](./parametres.md).

### Via les Graphes
Sur les graphes vous avez sur l'axe des abscisses des indicateurs de progression (en km) permettant de vous positionner instantanément au kilomètre correspondant.
*   **Clic ou Glisser** sur le graphe présent pour vous positionner instantanément au kilomètre correspondant.

---

## Les Trois Modes d'Édition

### 1. Mode Caméra
C'est le mode par défaut pour définir le "vol" de la caméra. Vous placez des **points clés** pour dicter à la caméra où se situer à ces endroits. (voir [Introduction à la Caméra](./introduction_camera.md))

### 2. Mode Pause / FlyTo
Ce mode permet d'ajouter des événements ponctuels pour dynamiser l'animation. (voir [Événements : Pauses et FlyTo](./edition_flyto_pause.md))

### 3. Mode Message
Utilisez ce mode pour ajouter des annotations textuelles tout au long du parcours. (voir [Gestion des Messages](./edition_messages.md))

---

## Paramètres Liés

Certaines commandes de l'éditeur peuvent être personnalisées dans le menu [<span style="color: #FFC107">Paramètres</span>](./parametres.md) :

*   **Touches d'avancement** : Redéfinissez les touches clavier utilisées pour avancer ou reculer sur la trace (flèches par défaut).
*   **Incréments de déplacement** : Réglez la finesse du déplacement (nombre de points de tracking par appui touche).
*   **Commandes de Pitch** : Changez les touches ou l'incrément de l'inclinaison de la caméra.

---

