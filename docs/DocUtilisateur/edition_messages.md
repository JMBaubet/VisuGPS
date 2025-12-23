# Gestion des Messages

Les messages sont des bulles de texte informatives qui apparaissent dynamiquement sur la carte pendant la visualisation. Ils servent à identifier des cols, des points de ravitaillement, à donner des informations de distance, des instructions de direction, (voir [Marqueurs kilométriques automatiques](./edition_marqueurs_km.md)), etc.

[< Retour aux généralités Édition](./edition_intro.md)

---

## L'onglet Message

Situé dans le panneau de contrôle inférieur, l'onglet **<span style="color: #FF9800">Message</span>** regroupe tout ce qui concerne la gestion des messages manuels et des marqueurs kilométriques. (voir [Marqueurs kilométriques](./edition_marqueurs_km.md))

### 1. Sélection du Message
Cliquez sur le bouton <span style="background-color: rgba(33, 150, 243, 0.1); color: #2196F3; padding: 2px 10px; border-radius: 4px; font-size: 0.85em;">SÉLECTIONNER UN MESSAGE</span> pour ouvrir votre bibliothèque. (voir [Bibliothèque de messages](./library_messages.md))
*   Une fois sélectionné, un aperçu du message s'affiche avec son **texte** et sa **couleur de fond** réelle.


### 2. Durée d'affichage
Un double curseur permet de définir précisément quand le message apparaît et disparaît par rapport au passage du marqueur :
*   **Pré-affichage** (Côté gauche) : Distance avant que le marqueur n'atteigne le point d'ancrage.
*   **Post-affichage** (Côté droit) : Distance après le passage du marqueur.

> [!NOTE]
> Par défaut, une unité (incrément) correspond à **100 m**. Si vous réglez le pré-affichage sur 5, le message apparaîtra 500 m avant le point précis.

### 3. Mise en page
Un interrupteur (switch) **Gauche / Droite** permet de choisir de quel côté de la trace la bulle doit s'afficher. Cela permet d'éviter le chevauchement de deux messages proches l'un de l'autre.

---

## Créer et Gérer les Messages

> [!NOTE]
> Les messages peuvent être ancrés n'importe où sur la carte, que ce soit précisément sur votre trace ou sur un point d'intérêt situé à l'écart du parcours.

### Ajouter un message
1.  **Position temporelle** : Placez le marqueur au kilomètre souhaité sur la trace.
2.  **Ciblage de l'ancrage** : Déplacez la carte (**clic gauche**) pour placer l'endroit exact où la flèche du message doit pointer (ex: un sommet, une auberge) au centre de votre écran.
    > [!TIP]
    > Comme pour les survols, le **clic gauche** fait basculer la vue en **2D** pour vous permettre de pointer le lieu avec une précision chirurgicale.
3.  Sélectionnez votre message dans la bibliothèque.
4.  Réglez l'orientation (**Gauche/Droite**) et la durée d'affichage.
5.  Cliquez sur le bouton <span style="color: #2196F3">AJOUTER MESSAGE</span> <img src="https://api.iconify.design/mdi/plus.svg?width=20&color=%232196F3" style="vertical-align: middle; margin-bottom: 3px;">.

### Mettre à jour ou Supprimer
Si un message existe déjà à votre position actuelle :
*   Le bouton se transforme en <span style="color: #2196F3">METTRE À JOUR</span> <img src="https://api.iconify.design/mdi/check.svg?width=20&color=%232196F3" style="vertical-align: middle; margin-bottom: 3px;"> pour valider vos changements.
*   Un bouton <span style="color: #F44336">SUPPRIMER</span> <img src="https://api.iconify.design/mdi/delete.svg?width=20&color=%23F44336" style="vertical-align: middle; margin-bottom: 3px;"> permet de le retirer.

---

## Visualisation sur le Graphe

Le graphe des messages offre une vue temporelle et spatiale de toutes vos annotations :

*   **Rectangles de couleur** : Chaque message est représenté par un bloc horizontal. Sa largeur correspond à sa durée de visibilité (pré-affichage + post-affichage). La couleur du bloc est identique à celle choisie pour le message.
*   **Ligne verticale (Ancrage)** : Au sein de chaque rectangle, un trait vertical plus épais indique le point d'ancrage exact (le kilomètre précis) du message.
*   **Superposition (Lignes)** : Pour éviter que les messages ne se chevauchent, ils sont répartis sur plusieurs hauteurs (lignes). 
    *   Les **marqueurs kilométriques** automatiques occupent généralement les lignes du bas.
    *   Vos **messages personnels** occupent les lignes supérieures.
*   **Interaction** : 
    *   **Survol** : Passez la souris sur un rectangle pour voir le texte du message dans une bulle d'aide.
    *   **Déplacement** : Cliquez n'importe où sur le graphe pour positionner instantanément le marqueur de l'application à cet endroit.

---

En complément des messages manuels, vous pouvez également générer automatiquement des jalons de distance sur l'ensemble de votre parcours. (voir [Gestion des Marqueurs Kilométriques](./edition_marqueurs_km.md))

---
[< Retour aux généralités Édition](./edition_intro.md) | [Suivant : Mode Visualisation >](./visualisation.md)
