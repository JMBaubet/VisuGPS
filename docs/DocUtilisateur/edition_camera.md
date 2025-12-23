# Éditer la Caméra

Ce guide explique comment utiliser l'interface pour créer, modifier et supprimer vos points de contrôle caméra afin d'obtenir une animation fluide.

[< Retour au mode Édition](./edition_intro.md#les-trois-modes-dédition) | [< Retour au guide d'exploitation](./exploitation.md)


## Pilotage de la Vue

La création d'un point de vue repose sur le réglage de trois paramètres. Pour garantir une stabilité visuelle, **privilégiez l'ajustement du Cap** et ne modifiez le Zoom et le Pitch que lorsque c'est réellement nécessaire.

> [!TIP]
>
>
> Dans l'indicateur situé en haut à droite de l'écran (Paramètres de Caméra), les valeurs de **Zoom** et de **Pitch** s'affichent en **<span style="color: #4CAF50">vert</span>** lorsqu'elles correspondent aux réglages par défaut du logiciel. 
Cela vous aide à repérer visuellement les moments où vous n'utilisez pas ces valeurs par défaut qui doivent être privilégiées.

### 1. Le Cap (Direction du regard)
*C'est l'outil principal pour suivre les virages de la trace.*
*   **Sur la Carte** : Maintenez le **Clic Droit** et glissez **Horizontalement**.
*   **Sur le Panneau d'Édition** : Cliquez simplement sur le panneau avec la souris et : 
**Molette seule** : permet un réglage d'une extrême précision de 1° par défaut. 
**Maj(Shift) + Molette** : permet d'avoir un pas de 5° par défaut.

### 2. Le Zoom (Rapprochement)
*   **Molette seule** : Zoomer / Dézoomer d'un pas fin (0.1).
*   **Maj (Shift) + Molette** : Zoomer / Dézoomer rapidement (pas de 1.0).

### 3. Le Pitch (Inclinaison verticale)
*   **Clavier uniquement** : Utilisez les **Flèches Haut / Bas** (1° par appui, 5° avec Maj). 



### Navigation sur la Trace
Pour vous déplacer le long du parcours tout en ajustant votre vue, utilisez les **Flèches Gauche / Droite** du clavier (100 m / 1 km avec Maj).

---

## Gestion des Points de Contrôle

Une fois que vous avez trouvé l'angle de vue idéal pour un endroit précis :

1.  **AJOUTER** : Cliquez sur le bouton <span style="color: #2196F3">AJOUTER POINT</span> <img src="https://api.iconify.design/mdi/plus.svg?width=20&color=%232196F3" style="vertical-align: middle; margin-bottom: 3px;">. 
Un nouveau point clé apparaît sur la partie supérieure du graphe.
2.  **MODIFIER** : Pour changer un point existant, 
Positionnez-vous dessus, 
Ajustez votre vue,
cliquez sur <span style="color: #2196F3">AJOUTER POINT</span> <img src="https://api.iconify.design/mdi/plus.svg?width=20&color=%232196F3" style="vertical-align: middle; margin-bottom: 3px;"> pour écraser l'ancienne position par la nouvelle. 
Une demande de confirmation vous sera demandée.
3.  **SUPPRIMER** : Si un point ne vous convient plus, positionnez-vous dessus et cliquez sur <span style="color: #F44336">SUPPRIMER POINT</span> <img src="https://api.iconify.design/mdi/delete.svg?width=20&color=%23F44336" style="vertical-align: middle; margin-bottom: 3px;">. 
Une demande de confirmation vous sera demandée. 
Le point clé disparait du graphe.

> [!TIP]
>
>
> **Priorité au Cap** : Dans la majorité des cas, vous n'aurez qu'à ajuster le **Cap** pour suivre la route. Ne modifiez le **Zoom** et le **Pitch** que pour des moments particuliers (survol d'un sommet, zoom sur un détail), afin de garder une stabilité visuelle globale.

---

## Méthode de travail recommandée

Pour travailler efficacement sur l'édition de votre caméra, voici la méthode pas à pas conseillée :

1.  **Sélecteur de la Caméra** : Choisissez le mode <span style="background-color: #2196F3; color: white; padding: 2px 10px; border-radius: 12px; font-size: 0.85em;">FIXE</span>. Cela vous permet de préparer vos points de vue sans que la caméra ne bouge toute seule.
2.  **Affichage des Courbes** : Dans le panneau dédié, affichez principalement le **Cap** et le **Δ Cap** édités. Ce sont vos deux indicateurs de fluidité indispensables.
3.  **Zooms de transition** : Ne configurez le **Zoom Départ** et le **Zoom Arrivée** qu'en toute fin de travail, une fois que toute votre trajectoire est déjà bien calée.
4.  **Contrôle Combiné** : 
    *   Gardez votre souris positionnée à l'intérieur de l'onglet **Caméra**.
    *   Utilisez votre main gauche sur les **Flèches Gauche / Droite** du clavier pour vous déplacer le long de la trace.
    *   Utilisez votre main droite pour actionner la **Molette de la souris** afin de modifier le **Cap** avec précision dès que vous voulez modifier le cap avant d'ajouter le point clé via le bouton <span style="color: #2196F3">AJOUTER POINT</span> <img src="https://api.iconify.design/mdi/plus.svg?width=20&color=%232196F3" style="vertical-align: middle; margin-bottom: 3px;">.

---

## Le secret de la fluidité

Pour obtenir une vidéo agréable à regarder :
*   Ayez une vision lointaine pour votre caméra, ne chercher pas à suivre la route avec une vision trop proche.
*   Sur les routes en lacets, placez plutôt la caméra perpendiculairement à la route.
*   Si vous constatez un mouvement brusque (Δ Cap elevé), entre deux points, supprimer un point d'extrémité de ce mouvement brusque pour adoucir le rendu final.

---

### 🛠️ Paramètres Liés
Retrouvez les réglages détaillés associés à cette fonctionnalité dans la section :
* [3.3. Camera](./parametres.md#33-camera)

---
[< Retour au mode Édition](./edition_intro.md#les-trois-modes-dédition) | [< Retour au guide d'exploitation](./exploitation.md)
