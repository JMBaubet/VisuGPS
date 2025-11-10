# 🎨 Paramètre : Couleur 9% <= Pente < 12% (Profil Altimétrique)

Ce document détaille le paramètre `Tranche4`, qui définit la couleur utilisée pour représenter les pentes difficiles (entre 9% et 12%) sur le profil altimétrique.

---

## 🎯 Rôle du Paramètre

Le paramètre `Tranche4` permet de visualiser clairement les sections de montée difficile sur le profil altimétrique.

-   **Libellé**: 9% <= Pente < 12%
-   **Type**: Couleur (Material Design)
-   **Valeur par défaut**: "red"
-   **Surcharge**: `null` (indique que ce paramètre peut être surchargé par les paramètres spécifiques à une trace)

## ⚖️ Justification : Pourquoi choisir une couleur spécifique pour les pentes difficiles ?

L'utilisation de couleurs distinctes pour les différentes tranches de pente améliore considérablement la lisibilité et l'analyse du profil altimétrique.

### 1. 👀 Analyse Intuitive

-   Une couleur spécifique pour les pentes difficiles permet d'identifier rapidement les sections nécessitant un effort très important.

### 2. 🎨 Cohérence Visuelle

-   S'intègre dans un schéma de couleurs qui représente visuellement l'intensité de la pente, souvent avec une progression du vert (facile) au rouge/violet (difficile).

---

## ⚠️ Recommandations

-   **Valeur par défaut ("red")** : Le rouge est intuitivement associé à la difficulté ou à un effort intense, ce qui est approprié pour les pentes difficiles.
-   **Utiliser les couleurs Material Design** : Le paramètre accepte les noms de couleurs définis par Material Design.
-   **Assurer le contraste** : Choisissez une couleur qui contraste bien avec les autres tranches de pente pour une distinction claire.
