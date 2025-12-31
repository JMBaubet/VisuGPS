# 🎨 Paramètre : Couleur de la croix centrale (Édition)

Ce document détaille le paramètre `couleurCroixCentraleEdition`, qui définit la couleur de la croix centrale affichée lorsque vous êtes sur les onglets **Pause/Survol** ou **Message** en mode édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `couleurCroixCentraleEdition` permet de personnaliser l'apparence visuelle de la croix centrale de visée en mode édition. Comme en mode visualisation, le choix de la couleur est important pour assurer sa visibilité sur différents fonds de carte.

-   **Libellé**: Couleur de la croix centrale
-   **Type**: Couleur (Material Design)
-   **Valeur par défaut**: "white"

## ⚖️ Justification : Pourquoi choisir la couleur de la croix centrale ?

La croix centrale est l'outil principal pour positionner avec précision les événements (Messages, Survols) sur la carte.

### 1. 👀 Visibilité et Précision
Une couleur contrastante assure que le centre de la vue (le point d'ancrage de l'événement) est parfaitement identifiable, quel que soit le style de carte utilisé (satellite, rues, etc.).

### 2. 🧩 Cohérence Visuelle
Permet d'harmoniser l'interface d'édition avec vos préférences visuelles ou de la distinguer de la croix de pause du mode visualisation si vous le souhaitez.

---

## ⚠️ Recommandations

-   **Valeur par défaut ("white")** : Le blanc est généralement le plus efficace sur les cartes satellites sombres.
-   **Utiliser les couleurs Material Design** : Le paramètre accepte les noms de couleurs standard du framework Vuetify.
-   **Contrôle du contraste** : Si vous travaillez sur des zones très claires (neige, sable), une couleur plus sombre ou vive (ex: "red" ou "blue") peut s'avérer plus confortable.
