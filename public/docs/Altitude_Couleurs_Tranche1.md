# 🎨 Paramètre : Couleur 0% < Pente < 3% (Profil Altimétrique)

Ce document détaille le paramètre `Tranche1`, qui définit la couleur utilisée pour représenter les pentes douces (entre 0% et 3%) sur le profil altimétrique.

---

## 🎯 Rôle du Paramètre

Le paramètre `Tranche1` permet de visualiser clairement les sections de faible montée sur le profil altimétrique.

-   **Libellé**: 0% < Pente < 3%
-   **Type**: Couleur (Material Design)
-   **Valeur par défaut**: "green"
-   **Surcharge**: `null` (indique que ce paramètre peut être surchargé par les paramètres spécifiques à une trace)

## ⚖️ Justification : Pourquoi choisir une couleur spécifique pour les pentes douces ?

L'utilisation de couleurs distinctes pour les différentes tranches de pente améliore considérablement la lisibilité et l'analyse du profil altimétrique.

### 1. 👀 Analyse Intuitive

-   Une couleur spécifique pour les pentes douces permet d'identifier rapidement les sections de faible effort.

### 2. 🎨 Cohérence Visuelle

-   S'intègre dans un schéma de couleurs qui représente visuellement l'intensité de la pente, souvent avec une progression du vert (facile) au rouge/violet (difficile).

---

## ⚠️ Recommandations

-   **Valeur par défaut ("green")** : Le vert est intuitivement associé à la "facilité" ou à un effort modéré, ce qui est approprié pour les pentes douces.
-   **Utiliser les couleurs Material Design** : Le paramètre accepte les noms de couleurs définis par Material Design.
-   **Assurer le contraste** : Choisissez une couleur qui contraste bien avec les autres tranches de pente pour une distinction claire.
