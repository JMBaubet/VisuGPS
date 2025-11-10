# 🎨 Paramètre : Couleur 3% <= Pente < 6% (Profil Altimétrique)

Ce document détaille le paramètre `Tranche2`, qui définit la couleur utilisée pour représenter les pentes modérées (entre 3% et 6%) sur le profil altimétrique.

---

## 🎯 Rôle du Paramètre

Le paramètre `Tranche2` permet de visualiser clairement les sections de montée modérée sur le profil altimétrique.

-   **Libellé**: 3% <= Pente < 6%
-   **Type**: Couleur (Material Design)
-   **Valeur par défaut**: "yellow"
-   **Surcharge**: `null` (indique que ce paramètre peut être surchargé par les paramètres spécifiques à une trace)

## ⚖️ Justification : Pourquoi choisir une couleur spécifique pour les pentes modérées ?

L'utilisation de couleurs distinctes pour les différentes tranches de pente améliore considérablement la lisibilité et l'analyse du profil altimétrique.

### 1. 👀 Analyse Intuitive

-   Une couleur spécifique pour les pentes modérées permet d'identifier rapidement les sections nécessitant un effort moyen.

### 2. 🎨 Cohérence Visuelle

-   S'intègre dans un schéma de couleurs qui représente visuellement l'intensité de la pente, souvent avec une progression du vert (facile) au rouge/violet (difficile).

---

## ⚠️ Recommandations

-   **Valeur par défaut ("yellow")** : Le jaune est intuitivement associé à un avertissement ou un effort croissant, ce qui est approprié pour les pentes modérées.
-   **Utiliser les couleurs Material Design** : Le paramètre accepte les noms de couleurs définis par Material Design.
-   **Assurer le contraste** : Choisissez une couleur qui contraste bien avec les autres tranches de pente pour une distinction claire.
