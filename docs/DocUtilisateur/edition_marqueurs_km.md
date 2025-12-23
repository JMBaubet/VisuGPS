# Marqueurs Kilométriques

Les marqueurs kilométriques sont des jalons visuels automatiques qui permettent au spectateur de se situer facilement sur la trace pendant la visualisation 3D.

[< Retour à la gestion des Messages](./edition_messages.md)

---

## Fonctionnement

Ces outils permettent de jalonner automatiquement votre parcours sans avoir à créer manuellement chaque message de distance.


### 1. Ouverture de la Configuration
Cliquez sur le bouton <span style="color: #2196F3">AJOUTER LES DISTANCES</span> <img src="https://api.iconify.design/mdi/map-marker-distance.svg?width=20&color=%232196F3" style="vertical-align: middle; margin-bottom: 3px;"> pour ouvrir la fenêtre de configuration. Si des marqueurs existent déjà, le bouton devient <span style="color: #607D8B">MODIFIER KM</span> <img src="https://api.iconify.design/mdi/map-marker-distance.svg?width=20&color=%23607D8B" style="vertical-align: middle; margin-bottom: 3px;"> pour vous permettre d'ajuster les réglages existants.

### 2. Options de Configuration
La fenêtre **Configuration des bornes kilométriques** vous permet de personnaliser l'affichage global :

*   **Intervalle** : Réglez la fréquence d'apparition (de 1 à 20 km). Par exemple, un marqueur tous les 5 km ou 10 km.
*   **Pré-affichage** : Nombre d'incréments (unités de 100 m) avant le point kilométrique où la bulle doit apparaître.
*   **Post-affichage** : Nombre d'incréments après le point kilométrique où la bulle doit disparaître.
*   **Orientation** : Choisissez de quel côté de la trace (**Gauche** ou **Droite**) les bulles doivent s'afficher.
*   **Couleur des messages** : Sélectionnez une couleur dans la palette pour les bulles de distance.

### 3. Application et Génération
Une fois vos réglages terminés, cliquez sur **Appliquer**. L'application va alors :
1. Calculer les positions exactes sur toute la longueur de la trace.
2. Générer instantanément tous les messages correspondants.
3. Ces marqueurs utilisent un style de bulle compact pour ne pas surcharger la vue.

### 4. Suppression
Si vous souhaitez retirer tous les jalons de distance d'un seul coup, utilisez le bouton <span style="color: #F44336">SUPPRIMER KM</span> <img src="https://api.iconify.design/mdi/delete.svg?width=20&color=%23F44336" style="vertical-align: middle; margin-bottom: 3px;">.

---

### 🛠️ Paramètres Liés
Retrouvez les réglages détaillés associés à cette fonctionnalité dans la section :
* [3.5.1. Distance](./parametres.md#351-distance)

---
[< Retour à la gestion des Messages](./edition_messages.md)
