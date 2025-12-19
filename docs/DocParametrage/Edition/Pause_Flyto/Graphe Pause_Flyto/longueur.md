# 🔎 Paramètre : Longueur des marqueurs Pause/Survol
 
Ce document détaille le paramètre `longueur`, qui définit la longueur en pixels des marqueurs verticaux pour les événements de type "Pause" et "Survol" sur le graphe d'édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `longueur` permet d'ajuster la taille visuelle des indicateurs d'événements "Pause" et "Survol" sur le graphe. Une longueur appropriée assure que les marqueurs sont suffisamment visibles sans surcharger l'interface.

-   **Libellé**: Longueur (px)
-   **Type**: Entier
-   **Valeur par défaut**: 12 px
-   **Minimum**: 5 px
-   **Maximum**: 50 px

## ⚖️ Justification : Importance de la taille des marqueurs

La taille des marqueurs d'événements est un facteur clé pour leur perception et leur lisibilité sur le graphe. Des marqueurs trop petits peuvent être difficiles à distinguer, tandis que des marqueurs trop grands peuvent encombrer la visualisation.

### 1. 👀 Visibilité

Une longueur adéquate garantit que les marqueurs sont facilement repérables même sur des graphes complexes.

### 2. 📊 Clarté visuelle

Optimiser la longueur contribue à une meilleure clarté générale du graphe, en évitant les chevauchements ou les confusions.

## ⚠️ Recommandations

-   **Valeur par défaut (12 px)** : C'est un bon compromis pour la plupart des utilisations, offrant une visibilité claire sans être trop envahissant.
-   **Ajustement** : Vous pouvez augmenter la longueur pour les animations avec peu d'événements ou pour accentuer certains points. Diminuez-la si le graphe est très dense en événements.
