# 🎨 Paramètre : Couleur Somme Delta Bearing Edité (Graphe)

Ce document détaille le paramètre `couleurEditedBearingTotalDelta`, qui définit la couleur de la courbe représentant la somme cumulée des deltas de cap édités sur le graphe d'édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `couleurEditedBearingTotalDelta` permet de personnaliser l'apparence visuelle de la courbe de la somme cumulée des deltas de cap après qu'elle ait été modifiée manuellement ou lissée. Cela aide à distinguer la version éditée de la version originale et des autres courbes.

-   **Libellé**: Couleur Somme Delta Bearing Edité
-   **Type**: Couleur (Material Design)
-   **Valeur par défaut**: "red-darken-4"

## ⚖️ Justification : Pourquoi choisir une couleur pour la somme du delta de cap édité ?

Le choix de la couleur est important pour la clarté et la lisibilité du graphe, surtout lorsque plusieurs courbes sont affichées simultanément.

### 1. 👀 Lisibilité et Distinction

-   Une couleur distincte aide à différencier rapidement la courbe de la somme du delta de cap éditée de la courbe de la somme du delta de cap originale et des autres courbes.
-   La valeur par défaut "red-darken-4" est une nuance plus foncée de la couleur par défaut de la somme du delta de cap original, ce qui crée un lien visuel tout en marquant la différence.

### 2. 🎨 Cohérence Visuelle

-   Permet à l'utilisateur de choisir une couleur qui correspond à ses préférences ou à un schéma de couleurs qu'il utilise pour d'autres visualisations.

---

## ⚠️ Recommandations

-   **Valeur par défaut ("red-darken-4")** : C'est un bon choix pour maintenir une cohérence visuelle avec la courbe de la somme du delta de cap originale tout en la distinguant.
-   **Utiliser les couleurs Material Design** : Le paramètre accepte les noms de couleurs définis par Material Design (par exemple, "blue", "green", "purple").
-   **Assurer le contraste** : Choisissez une couleur qui contraste bien avec le fond du graphe et les autres courbes affichées.
