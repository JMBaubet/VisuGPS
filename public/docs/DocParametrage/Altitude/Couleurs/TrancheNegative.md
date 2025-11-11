# 🎨 Paramètre : Couleur Pente <= 0% (Profil Altimétrique)

Ce document détaille le paramètre `TrancheNegative`, qui définit la couleur utilisée pour représenter les pentes négatives ou nulles sur le profil altimétrique.

---

## 🎯 Rôle du Paramètre

Le paramètre `TrancheNegative` permet de visualiser clairement les sections descendantes ou plates de la trace sur le profil altimétrique.

-   **Libellé**: Pente <= 0%
-   **Type**: Couleur (Material Design)
-   **Valeur par défaut**: "light-blue"
-   **Surcharge**: `null` (indique que ce paramètre peut être surchargé par les paramètres spécifiques à une trace)

## ⚖️ Justification : Pourquoi choisir une couleur spécifique pour les pentes négatives/nulles ?

L'utilisation de couleurs distinctes pour les différentes tranches de pente améliore considérablement la lisibilité et l'analyse du profil altimétrique.

### 1. 👀 Analyse Intuitive

-   Une couleur spécifique pour les pentes négatives/nulles permet d'identifier rapidement les sections de descente ou de plat.

### 2. 🎨 Cohérence Visuelle

-   S'intègre dans un schéma de couleurs qui représente visuellement l'intensité de la pente.

---

## ⚠️ Recommandations

-   **Valeur par défaut ("light-blue")** : Le bleu clair est souvent associé à la "facilité" ou au "plat", ce qui est intuitif pour les pentes négatives ou nulles.
-   **Utiliser les couleurs Material Design** : Le paramètre accepte les noms de couleurs définis par Material Design.
-   **Assurer le contraste** : Choisissez une couleur qui contraste bien avec les autres tranches de pente pour une distinction claire.
