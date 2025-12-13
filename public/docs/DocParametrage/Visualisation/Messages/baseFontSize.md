# 📏 Paramètre : Taille de police de base des messages

Ce document détaille le paramètre `baseFontSize`, qui définit la taille de police en pixels utilisée pour le texte des messages affichés sur la carte.

---

## 🎯 Rôle du Paramètre

Le paramètre `baseFontSize` contrôle la taille du texte dans les bulles d'information. La taille de la bulle s'adapte proportionnellement à cette valeur.

-   **Libellé**: Taille de police de base des messages
-   **Type**: Entier
-   **Valeur par défaut**: 30 px
-   **Minimum**: 10 px
-   **Maximum**: 100 px

## ⚖️ Justification : Pourquoi ajuster la taille de la police ?

L'ajustement de la taille de police permet d'adapter la lisibilité des messages en fonction de la résolution de l'écran et de l'acuité visuelle de l'utilisateur.

### 1. 👀 Lisibilité

-   Une taille plus grande améliore la lecture, surtout sur les écrans haute résolution ou pour les présentations.
-   Une taille plus petite permet d'afficher plus de contenu sans masquer la carte.

### 2. 🗺️ Occupation de l'espace

-   Les messages plus gros occupent une part plus importante de la vue 3D, ce qui peut masquer le tracé ou le relief.

---

## ⚠️ Recommandations

-   **Valeur par défaut (30 px)** : Offre un bon équilibre pour la plupart des écrans Full HD (1080p).
-   **Ajuster selon la résolution** : Augmentez cette valeur pour les écrans 4K/Retina, ou diminuez-la si vous trouvez les messages trop envahissants.
