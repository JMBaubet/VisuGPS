# 🎨 Paramètre : Couleur 6% <= Pente < 9% (Profil Altimétrique)

Ce document détaille le paramètre `Tranche3`, qui définit la couleur utilisée pour représenter les pentes soutenues (entre 6% et 9%) sur le profil altimétrique.

---

## 🎯 Rôle du Paramètre

Le paramètre `Tranche3` permet de visualiser clairement les sections de montée soutenue sur le profil altimétrique.

-   **Libellé**: 6% <= Pente < 9%
-   **Type**: Couleur (Material Design)
-   **Valeur par défaut**: "orange"
-   **Surcharge**: `null` (indique que ce paramètre peut être surchargé par les paramètres spécifiques à une trace)

## ⚖️ Justification : Pourquoi choisir une couleur spécifique pour les pentes soutenues ?

L'utilisation de couleurs distinctes pour les différentes tranches de pente améliore considérablement la lisibilité et l'analyse du profil altimétrique.

### 1. 👀 Analyse Intuitive

-   Une couleur spécifique pour les pentes soutenues permet d'identifier rapidement les sections nécessitant un effort important.

### 2. 🎨 Cohérence Visuelle

-   S'intègre dans un schéma de couleurs qui représente visuellement l'intensité de la pente, souvent avec une progression du vert (facile) au rouge/violet (difficile).

---

## ⚠️ Recommandations

-   **Valeur par défaut ("orange")** : L'orange est intuitivement associé à un effort significatif, ce qui est approprié pour les pentes soutenues.
-   **Utiliser les couleurs Material Design** : Le paramètre accepte les noms de couleurs définis par Material Design.
-   **Assurer le contraste** : Choisissez une couleur qui contraste bien avec les autres tranches de pente pour une distinction claire.
