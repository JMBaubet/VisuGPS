# 📍 Paramètre : Position du curseur d'avancement (Profil Altimétrique)

Ce document détaille le paramètre `CurseurPositionKm`, qui définit la position en kilomètres où le curseur d'avancement se bloque lorsque le graphique du profil altimétrique défile.

---

## 🎯 Rôle du Paramètre

Le paramètre `CurseurPositionKm` contrôle le comportement du curseur d'avancement sur le profil altimétrique. Lorsque la trace est suffisamment longue pour que le profil défile, le curseur se "bloque" à cette position (en km depuis le début de la trace), et c'est le graphique qui défile derrière lui.

-   **Libellé**: Position du curseur (km)
-   **Type**: Entier
-   **Valeur par défaut**: 10 km
-   **Minimum**: 1 km

## ⚖️ Justification : Pourquoi définir la position du curseur d'avancement ?

La définition de la position du curseur d'avancement est importante pour maintenir le focus de l'utilisateur sur la progression de l'animation, même sur de très longues traces.

### 1. 👀 Maintien du Focus

-   En bloquant le curseur à une position fixe, l'utilisateur peut toujours voir la progression de l'animation par rapport à un point de référence constant, même si le graphique défile.
-   Évite que le curseur ne sorte de l'écran sur de très longues traces.

### 2. 🖐️ Expérience Utilisateur

-   Améliore la lisibilité et le suivi de l'animation sur le profil altimétrique.

---

## ⚠️ Recommandations

-   **Valeur par défaut (10 km)** : Cette valeur permet de voir les 10 premiers kilomètres de la trace avant que le défilement ne commence, offrant un bon équilibre entre la vue initiale et le suivi continu.
-   **Adapter aux préférences** : Vous pouvez ajuster cette valeur si vous préférez que le défilement commence plus tôt ou plus tard.
