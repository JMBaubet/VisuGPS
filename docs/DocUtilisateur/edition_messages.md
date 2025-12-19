# Gestion des Messages

Les messages sont des bulles de texte informatives qui apparaissent dynamiquement sur la carte pendant la visualisation. Ils servent à identifier des cols, des points de ravitaillement, à donner des informations de distance, des instructions de direction, etc.

[< Retour aux généralités Édition](./edition_intro.md)

---

## L'onglet Message

Situé dans le panneau de contrôle inférieur, l'onglet **<span style="color: #FF9800">Message</span>** regroupe tout ce qui concerne la gestion des messages.

### 1. Sélection du Message
Cliquez sur le bouton <span style="background-color: rgba(33, 150, 243, 0.1); color: #2196F3; padding: 2px 10px; border-radius: 4px; font-size: 0.85em;">SÉLECTIONNER UN MESSAGE</span> pour ouvrir votre bibliothèque. (voir [Bibliothèque de messages](./library_messages.md))
*   Une fois sélectionné, un aperçu du message s'affiche avec son **texte** et sa **couleur de fond** réelle.
*   Si aucun message n'est sélectionné, les options de réglage restent grisées.

### 2. Durée d'affichage
Un double curseur permet de définir précisément quand le message apparaît et disparaît par rapport au passage du marqueur :
*   **Pré-affichage** (Côté gauche) : Distance avant que le marqueur n'atteigne le point d'ancrage.
*   **Post-affichage** (Côté droit) : Distance après le passage du marqueur.

> [!NOTE]
> Par défaut, une unité (incrément) correspond à **100 m**. Si vous réglez le pré-affichage sur 5, le message apparaîtra 500 m avant le point précis.

### 3. Mise en page
Un interrupteur (switch) **Gauche / Droite** permet de choisir de quel côté de la trace la bulle doit s'afficher. Cela permet d'éviter de masquer un élément important du relief ou de la route sur la carte.

---

## Créer et Gérer les Messages

### Ajouter un message
1.  **Position temporelle** : Placez le marqueur au kilomètre souhaité sur la trace.
2.  **Ciblage de l'ancrage** : Déplacez la carte (**clic gauche**) pour placer l'endroit exact où la flèche du message doit pointer (ex: un sommet, une auberge) au centre de votre écran.
    > [!TIP]
    > Comme pour les survols, le **clic gauche** fait basculer la vue en **2D** pour vous permettre de pointer le lieu avec une précision chirurgicale.
3.  Sélectionnez votre message dans la bibliothèque.
4.  Réglez l'orientation (**G/D**) et la durée d'affichage.
5.  Cliquez sur le bouton <span style="color: #2196F3">AJOUTER MESSAGE</span> <img src="https://api.iconify.design/mdi/plus.svg?width=20&color=%232196F3" style="vertical-align: middle; margin-bottom: 3px;">.

### Mettre à jour ou Supprimer
Si un message existe déjà à votre position actuelle :
*   Le bouton se transforme en <span style="color: #2196F3">METTRE À JOUR</span> <img src="https://api.iconify.design/mdi/check.svg?width=20&color=%232196F3" style="vertical-align: middle; margin-bottom: 3px;"> pour valider vos changements.
*   Un bouton <span style="color: #F44336">SUPPRIMER</span> <img src="https://api.iconify.design/mdi/delete.svg?width=20&color=%23F44336" style="vertical-align: middle; margin-bottom: 3px;"> permet de le retirer.

---

## Gestion des Marqueurs Kilométriques

Ces outils permettent de jalonner automatiquement votre parcours pour aider le spectateur à se situer.

*   **Ajout Global** : Cliquez sur <span style="color: #2196F3">AJOUTER LES DISTANCES</span> <img src="https://api.iconify.design/mdi/map-marker-distance.svg?width=20&color=%232196F3" style="vertical-align: middle; margin-bottom: 3px;"> pour générer des marqueurs à intervalle régulier sur toute la trace.
*   **Réglage de la fréquence** : La fréquence de ces marqueurs se définit dans les [<span style="color: #FFC107">Paramètres</span>](./parametres.md) (**Édition > Messages > Marqueurs KM**). Par exemple tous les 5 km ou 10 km.
*   **Nettoyage** : Le bouton <span style="color: #F44336">SUPPRIMER KM</span> <img src="https://api.iconify.design/mdi/delete.svg?width=20&color=%23F44336" style="vertical-align: middle; margin-bottom: 3px;"> retire instantanément tous les jalons de distance.

---
[< Retour aux généralités Édition](./edition_intro.md) | [Suivant : Mode Visualisation >](./visualisation.md)
