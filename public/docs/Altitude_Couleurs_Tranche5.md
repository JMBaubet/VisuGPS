# 🎨 Paramètre : Couleur Pente >= 12% (Profil Altimétrique)

Ce document détaille le paramètre `Tranche5`, qui définit la couleur utilisée pour représenter les pentes très difficiles (supérieures ou égales à 12%) sur le profil altimétrique.

---

## 🎯 Rôle du Paramètre

Le paramètre `Tranche5` permet de visualiser clairement les sections de montée très difficile sur le profil altimétrique.

-   **Libellé**: Pente >= 12%
-   **Type**: Couleur (Material Design)
-   **Valeur par défaut**: "purple"
-   **Surcharge**: `null` (indique que ce paramètre peut être surchargé par les paramètres spécifiques à une trace)

## ⚖️ Justification : Pourquoi choisir une couleur spécifique pour les pentes très difficiles ?

L'utilisation de couleurs distinctes pour les différentes tranches de pente améliore considérablement la lisibilité et l'analyse du profil altimétrique.

### 1. 👀 Analyse Intuitive

-   Une couleur spécifique pour les pentes très difficiles permet d'identifier rapidement les sections nécessitant un effort extrême.

### 2. 🎨 Cohérence Visuelle

-   S'intègre dans un schéma de couleurs qui représente visuellement l'intensité de la pente, souvent avec une progression du vert (facile) au rouge/violet (difficile).

---

## ⚠️ Recommandations

-   **Valeur par défaut ("purple")** : Le violet est souvent associé à l'extrême ou à un effort maximal, ce qui est approprié pour les pentes très difficiles.
-   **Utiliser les couleurs Material Design** : Le paramètre accepte les noms de couleurs définis par Material Design.
-   **Assurer le contraste** : Choisissez une couleur qui contraste bien avec les autres tranches de pente pour une distinction claire.
