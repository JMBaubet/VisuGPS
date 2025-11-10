# ⛰️ Paramètre : Exagération du relief (Mapbox)

Ce document détaille le paramètre `exaggeration`, qui définit le facteur d'exagération du relief 3D affiché sur les cartes Mapbox en mode édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `exaggeration` permet d'ajuster la perception du dénivelé sur la carte 3D. Une valeur de 1.0 représente le relief réel, tandis qu'une valeur supérieure à 1.0 exagère les hauteurs et les profondeurs.

-   **Libellé**: Exagération du relief
-   **Type**: Réel (Float)
-   **Valeur par défaut**: 1.0
-   **Minimum**: 0.0 (relief plat)
-   **Maximum**: 2.0
-   **Décimales**: 2

## ⚖️ Justification : Pourquoi ajuster l'exagération du relief ?

L'ajustement de l'exagération du relief est utile pour améliorer la visualisation et l'analyse des variations d'altitude, en particulier sur des terrains peu accidentés.

### 1. 👀 Visibilité du Relief

-   Sur des terrains relativement plats, une légère exagération (`> 1.0`) peut rendre les petites variations d'altitude plus perceptibles, facilitant l'analyse du profil.
-   Sur des terrains très accidentés, une valeur plus faible (`< 1.0`) peut réduire la déformation visuelle, ou la valeur par défaut (1.0) peut être suffisante.

### 2. 🎨 Esthétique

-   Permet de personnaliser l'apparence de la carte 3D selon les préférences de l'utilisateur ou le type de trace visualisée.

---

## ⚠️ Recommandations

-   **Valeur par défaut (1.0)** : Représente le relief de manière fidèle, ce qui est un bon point de départ.
-   **Adapter au terrain** :
    -   Pour des traces en plaine ou avec un faible dénivelé, une valeur entre 1.2 et 1.5 peut aider à mieux percevoir les légères variations.
    -   Pour des traces en montagne avec un fort dénivelé, une valeur de 1.0 est souvent suffisante pour éviter une distorsion excessive.
