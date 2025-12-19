# Introduction à la Caméra

Lors du chargement de votre fichier GPX, l'application effectue un premier calcul automatique pour positionner et orienter la caméra le long de la trace. Cependant, sur des parcours complexes — avec des changements de direction fréquents ou un relief marqué — l'automatisation seule peine à offrir une fluidité parfaite et un angle de vue toujours optimal.

Le but de l'édition de la caméra est de positionner, sur des points spécifiques, un point de vue qui va permettre d'obtenir une visualisation la plus fluide possible et de mettre en valeur les dénivelés et les courbes du relief, essentiels pour apprécier pleinement les parcours montagneux.

En plaçant manuellement des **points clés**, vous créez un véritable scénario de vol cinématographique adapté à votre trace.

---

## Présentation de l'Interface

L'onglet <span style="color: #FF9800">CAMÉRA</span> du panneau latéral (situé en bas à doite de votre écran) est organisé de haut en bas pour structurer votre travail :

### 1. Comportement de la Caméra (Haut)
Choisissez comment la caméra se comporte pendant vos réglages :
*   <span style="background-color: #2196F3; color: white; padding: 2px 10px; border-radius: 12px; font-size: 0.85em;">FIXE</span> : La caméra reste dans son état actuel (Zoom, Pitch, Cap). C'est le mode idéal pour préparer un nouveau point de vue sans être emporté par le mouvement existant. 
Préférez ce mode quand vous devez ajouter un point clé à un endroit stratégique.
*   <span style="background-color: #2196F3; color: white; padding: 2px 10px; border-radius: 12px; font-size: 0.85em;">ANIMÉE</span> : La caméra suit fidèlement la trajectoire déjà programmée. 
Préférez ce mode pour vérifier immédiatement le rendu de vos transitions.

### 2. Affichage des Courbes (Milieu)
Un panneau repliable vous permet d'activer ou de masquer individuellement les courbes sur le graphe (Cap, Δ Cap, Zoom, Pitch). Vous pouvez ainsi vous concentrer sur un seul paramètre à la fois.

### 3. Zooms de Départ et d'Arrivée (Bas)
Au point de départ et d'arrivée de votre parcours, vous pouvez mettre en place un zoom automatique afin de mieux visualiser ces deux endroits.
Deux onglets permettent de configurer des transitions automatiques de ces deux extrémités :
*  **Check box** : Active ou desactive le zoom automatique.
*   **Curseur Zoom** : Permettent de régler le niveau de zoom.
*   **Curseur Distance** : Permettent de régler la distance de transition (en mètres).

Pour prendre en compte ces paramètres, vous devez cliquer sur le bouton <span style="background-color: rgba(33, 150, 243, 0.1); color: #2196F3; padding: 2px 10px; border-radius: 4px; font-size: 0.85em;">METTRE À JOUR LE ZOOM</span>.

### 4. Actions de Modification (Pied du panneau)
*   <span style="color: #2196F3">AJOUTER POINT</span> <img src="https://api.iconify.design/mdi/plus.svg?width=20&color=%232196F3" style="vertical-align: middle; margin-bottom: 3px;"> : Enregistre le point de vue actuel à l'emplacement de la trace.
*   <span style="color: #F44336">SUPPRIMER POINT</span> <img src="https://api.iconify.design/mdi/delete.svg?width=20&color=%23F44336" style="vertical-align: middle; margin-bottom: 3px;"> : Retire le point clé si vous êtes positionné exactement dessus.

---

## Le Graphe de Caméra

Situé en bas de l'écran, le graphe est votre tableau de bord technique. Il affiche la progression de chaque paramètre le long du parcours.

### Les Types de Courbes
*   **Cap** (Orientation absolue) : Indique vers quel point cardinal la caméra regarde (0° Nord, 90° Est, etc.).
*   **Δ Cap** (Variations de direction) : C'est la courbe la plus importante pour la fluidité. Elle montre la "pression" sur le volant de la caméra. Des pics trop hauts indiquent des virages trop brusques de la vue.
*   **Zoom / Pitch** : Visualisent le niveau de zoom et l'inclinaison (Pitch) de la vue.

### Calculée vs Éditée
*   **Colonne "Calculée" (Données GPX)** : Affiche le cap et le Δ Cap calculés lors de l'importation de votre trace. Les variations brusques de ces deux courbes indiquent un manque de fluidité dans l'animation. C'est votre base de référence.
*   **Colonne "Éditée" (Vos réglages)** : Affiche en couleur les courbes résultant de vos points clés. Le but est d'ajuster ces courbes pour qu'elles soient les plus régulières possibles.

---
[< Retour aux généralités Édition](./edition_intro.md) | [Passer à la pratique : Éditer la Caméra >](./edition_camera.md)
