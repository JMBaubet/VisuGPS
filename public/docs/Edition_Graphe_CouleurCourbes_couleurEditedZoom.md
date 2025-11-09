# 🎨 Paramètre : Couleur Zoom Edité (Graphe)

Ce document détaille le paramètre `couleurEditedZoom`, qui définit la couleur de la courbe représentant le niveau de zoom édité sur le graphe d'édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `couleurEditedZoom` permet de personnaliser l'apparence visuelle de la courbe de zoom après qu'elle ait été modifiée manuellement ou lissée. Cela aide à distinguer la version éditée de la version originale et des autres courbes.

-   **Libellé**: Couleur Zoom Edité
-   **Type**: Couleur (Material Design)
-   **Valeur par défaut**: "green-darken-4"

## ⚖️ Justification : Pourquoi choisir une couleur pour le zoom édité ?

Le choix de la couleur est important pour la clarté et la lisibilité du graphe, surtout lorsque plusieurs courbes sont affichées simultanément.

### 1. 👀 Lisibilité et Distinction

-   Une couleur distincte aide à différencier rapidement la courbe de zoom éditée de la courbe de zoom originale et des autres courbes.
-   La valeur par défaut "green-darken-4" est une nuance plus foncée de la couleur par défaut du zoom original, ce qui crée un lien visuel tout en marquant la différence.

### 2. 🎨 Cohérence Visuelle

-   Permet à l'utilisateur de choisir une couleur qui correspond à ses préférences ou à un schéma de couleurs qu'il utilise pour d'autres visualisations.

---

## ⚠️ Recommandations

-   **Valeur par défaut ("green-darken-4")** : C'est un bon choix pour maintenir une cohérence visuelle avec la courbe de zoom originale tout en la distinguant.
-   **Utiliser les couleurs Material Design** : Le paramètre accepte les noms de couleurs définis par Material Design (par exemple, "blue", "red", "purple").
-   **Assurer le contraste** : Choisissez une couleur qui contraste bien avec le fond du graphe et les autres courbes affichées.
