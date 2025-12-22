# 🎨 Paramètre : Couleur Somme Delta Bearing (Graphe)

Ce document détaille le paramètre `couleurBearingTotalDelta`, qui définit la couleur de la courbe représentant la somme cumulée des deltas de cap originaux (non édités) sur le graphe d'édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `couleurBearingTotalDelta` permet de personnaliser l'apparence visuelle de la courbe de la somme cumulée des deltas de cap sur le graphe, facilitant ainsi son identification et sa distinction des autres courbes.

-   **Libellé**: Couleur du Somme Delta Bearing
-   **Type**: Couleur (Material Design)
-   **Valeur par défaut**: "red"

## ⚖️ Justification : Pourquoi choisir une couleur pour la somme du delta de cap ?

Le choix de la couleur est important pour la clarté et la lisibilité du graphe, surtout lorsque plusieurs courbes sont affichées simultanément.

### 1. 👀 Lisibilité et Distinction

-   Une couleur distincte aide à différencier rapidement la courbe de la somme du delta de cap des autres courbes (zoom, pitch, etc.) et de sa version éditée.
-   Améliore la compréhension globale du graphe.

### 2. 🎨 Cohérence Visuelle

-   Permet à l'utilisateur de choisir une couleur qui correspond à ses préférences ou à un schéma de couleurs qu'il utilise pour d'autres visualisations.

---

## ⚠️ Recommandations

-   **Valeur par défaut ("red")** : C'est un choix standard et intuitif, souvent associé à un changement important ou à une alerte.
-   **Utiliser les couleurs Material Design** : Le paramètre accepte les noms de couleurs définis par Material Design (par exemple, "blue", "green", "purple").
-   **Assurer le contraste** : Choisissez une couleur qui contraste bien avec le fond du graphe et les autres courbes affichées.
