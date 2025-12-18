# Mode Édition : Vue d'ensemble

Le mode Édition est l'espace de travail où vous configurez votre trace et préparez votre visalisation 3D. Cet espace est organisé autour de trois modes de travail complémentaires accessibles via des onglets.

[< Retour à l'accueil](./index.md) | [< Précédent : Importation](./upload.md)

## Interface du Mode Édition

L'écran est optimisé pour la manipulation visuelle :

1.  **La Vue 3D** (En fond) : Affiche votre trace sur le relief Mapbox. C'est votre zone de travail principale.
2.  **Indicateur de Progression** (Haut au centre) : Affiche la distance parcourue (km) et la progression globale sur la trace.
3.  **Paramètres de Caméra** (Haut à droite) : Affiche en temps réel le **Cap** (angle), le **Zoom** et le **Pitch** (inclinaison).
4.  **Panneau de Contrôle** (Bas) : Regroupe les Graphes de données et les Onglets de réglage.

---

## Navigation Globale

Avant de configurer les détails, voici comment vous déplacer le long de votre trace :

### Au Clavier
*   **Flèches Gauche / Droite** : Avancer ou reculer sur la trace (Incrémentation de 1 point).
*   **Maj (Shift) + Flèches** : Déplacement rapide (Incrément de 10 points).

> [!NOTE]
> Si votre clavier ne possède pas de touches fléchées (ou si vous préférez d'autres touches), vous pouvez les redéfinir dans les [<span style="color: #FFC107">Paramètres</span>](./parametres.md).

### Via les Graphes
*   **Clic ou Glisser** sur le graphe présent pour vous positionner instantanément au kilomètre correspondant.

---

## Les Trois Modes d'Édition

### 1. Mode Caméra
C'est le mode par défaut pour définir le "vol" de la caméra. Vous placez des **points clés** pour dicter à la caméra où se situer à ces endroits.

(voir [Configuration de la Caméra](./edition_camera.md))

### 2. Mode Pause / FlyTo
Ce mode permet d'ajouter des événements ponctuels pour dynamiser l'animation.

(voir [Événements : Pauses et FlyTo](./edition_flyto_pause.md))

### 3. Mode Message
Utilisez ce mode pour ajouter des annotations textuelles tout au long du parcours.

(voir [Gestion des Messages](./edition_messages.md))

---

## Paramètres Liés

Certaines commandes de l'éditeur peuvent être personnalisées dans le menu [<span style="color: #FFC107">Paramètres</span>](./parametres.md) :

*   **Touches d'avancement** : Redéfinissez les touches clavier utilisées pour avancer ou reculer sur la trace (flèches par défaut).
*   **Incréments de déplacement** : Réglez la finesse du déplacement (nombre de points de tracking par appui touche).
*   **Commandes de Pitch** : Changez les touches ou l'incrément de l'inclinaison de la caméra.

---
[< Retour à l'accueil](./index.md)
