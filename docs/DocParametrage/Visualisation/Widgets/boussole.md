# 🧭 Paramètre : Affichage Boussole (Visualisation)

Ce document détaille le paramètre `boussole`, qui permet de définir si la boussole doit être affichée ou masquée lors du lancement de la visualisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `boussole` contrôle la visibilité de la boussole dynamique. Elle indique non seulement le cap actuel de la caméra, mais peut également afficher la direction du vent associée aux données météo.

-   **Libellé**: Boussole
-   **Type**: Booléen
-   **Valeur par défaut**: true (Activé)

## ⚖️ Justification : Pourquoi afficher une boussole ?

L'orientation est fondamentale dans un environnement 3D où la caméra pivote constamment pour suivre la trace ou explorer les environs.

### 1. 🗺️ Orientation Spatiale

-   Permet de savoir instantanément dans quelle direction cardinale (Nord, Sud, Est, Ouest) la caméra regarde, facilitant la lecture du relief.

### 2. 💨 Analyse Environnementale

-   Couplée au widget météo, la boussole permet de visualiser la direction du vent par rapport au sens de progression.

---

## ⚠️ Recommandations

-   **Navigation libre** : Fortement recommandé lors de l'utilisation de la télécommande pour explorer la carte.
-   **Raccourci clavier** : L'affichage peut être basculé à tout moment avec la touche **B**.
-   **Pilotage à distance** : Contrôlable dynamiquement depuis l'interface mobile.
