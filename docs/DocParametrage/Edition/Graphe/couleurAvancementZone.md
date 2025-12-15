# 🎨 Paramètre : Couleur de la zone d'avancement (Graphe)

Ce document détaille le paramètre `couleurAvancementZone`, qui définit la couleur de la zone rectangulaire indiquant l'avancement de la caméra sur le graphe d'édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `couleurAvancementZone` permet de personnaliser l'apparence visuelle de la zone qui met en évidence la portion de la trace actuellement visible ou en cours d'édition sur le graphe.

-   **Libellé**: Couleur de la zone d'avancement
-   **Type**: Couleur (Material Design)
-   **Valeur par défaut**: "yellow"

## ⚖️ Justification : Pourquoi choisir une couleur pour la zone d'avancement ?

Le choix de la couleur est important pour la clarté et la lisibilité du graphe, en particulier pour indiquer la position actuelle de la caméra.

### 1. 👀 Lisibilité et Repérage

-   Une couleur distincte aide à identifier rapidement la zone du graphe correspondant à la position actuelle de la caméra sur la carte.
-   Améliore la compréhension de la synchronisation entre le graphe et la carte.

### 2. 🎨 Cohérence Visuelle

-   Permet à l'utilisateur de choisir une couleur qui correspond à ses préférences ou à un schéma de couleurs qu'il utilise pour d'autres visualisations.

---

## ⚠️ Recommandations

-   **Valeur par défaut ("yellow")** : C'est un choix standard et intuitif pour une zone de mise en évidence.
-   **Utiliser les couleurs Material Design** : Le paramètre accepte les noms de couleurs définis par Material Design (par exemple, "blue", "red", "purple").
-   **Assurer le contraste** : Choisissez une couleur qui contraste bien avec le fond du graphe et les courbes affichées.
