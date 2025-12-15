# 📐 Paramètre : Angle de la caméra (pitch) par défaut pour le tracking

Ce document détaille le paramètre `Pitch`, qui définit l'angle d'inclinaison (pitch) par défaut de la caméra lors de la visualisation d'une trace GPX en mode tracking.

---

## 🎯 Rôle du Paramètre

Le paramètre `Pitch` contrôle l'angle vertical de la caméra par rapport à la surface de la carte. Une valeur de 0° signifie que la caméra regarde directement vers le bas (vue zénithale), tandis qu'une valeur de 90° signifie que la caméra regarde horizontalement (vue à l'horizon).

-   **Libellé**: Pitch
-   **Type**: Entier
-   **Valeur par défaut**: 60°
-   **Minimum**: 0°
-   **Maximum**: 90°
-   **Unité**: ° (degrés)

## ⚖️ Justification : Pourquoi ajuster l'angle de la caméra ?

L'angle de la caméra affecte la perception du relief et la perspective de la trace.

### 1. ⛰️ Perception du Relief

-   **Pitch élevé** (`> 60°`) : Offre une vue plus "plate" ou aérienne, réduisant la perception du relief. Utile pour des vues d'ensemble ou des terrains peu accidentés.
-   **Pitch faible** (`< 60°`) : Accentue la perception du relief, donnant une impression de profondeur et de dénivelé. Idéal pour les zones montagneuses ou vallonnées.

### 2. 🎥 Expérience de Visualisation

Le choix du pitch peut influencer l'immersion et la dynamique de l'animation.

-   Un pitch plus faible donne une sensation de "vol" plus proche du sol.
-   Un pitch plus élevé donne une sensation de "survol" plus lointain.

---

## ⚠️ Recommandations

-   **Valeur par défaut (60°)** : C'est un bon compromis qui offre une vue équilibrée, permettant de percevoir le relief sans être trop proche du sol.
-   **Adapter au terrain** :
    -   Pour les terrains plats, un pitch plus élevé (ex: 70-80°) peut être suffisant.
    -   Pour les terrains montagneux, un pitch plus faible (ex: 40-50°) mettra mieux en valeur le dénivelé.
