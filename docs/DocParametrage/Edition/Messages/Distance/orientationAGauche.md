# 🔎 Paramètre : Orientation des messages de distance

Ce document détaille le paramètre `orientation`, qui permet de choisir le côté (gauche ou droite) de la trace où les messages de bornes kilométriques sont affichés.

---

## 🎯 Rôle du Paramètre

Le paramètre `orientation` offre la flexibilité de positionner les bornes kilométriques par rapport à la trace. Ceci est utile pour optimiser la lisibilité en évitant les superpositions avec d'autres éléments visuels ou en s'adaptant à la disposition de la carte.

-   **Libellé**: Orientation à gauche
-   **Type**: Booléen
-   **Valeur par défaut**: `true` (Messages affichés à gauche)

## ⚖️ Justification : Importance de l'orientation des messages

Le positionnement des messages de distance peut grandement influencer la clarté de la visualisation et l'expérience utilisateur, surtout sur des traces complexes ou des fonds de carte chargés.

### 1. 📏 Prévention des chevauchements

Permet d'éviter que les messages de bornes kilométriques ne masquent des détails importants de la trace ou d'autres marqueurs.

### 2. 👁️ Amélioration de la lisibilité

En choisissant le côté le plus dégagé, on assure que les messages sont toujours facilement lisibles.

## ⚠️ Recommandations

-   **Dynamique** : Si d'autres informations sont affichées à un endroit précis, il peut être judicieux de changer l'orientation pour éviter les conflits visuels.
-   **Tester** : Il est recommandé de tester les deux options pour déterminer celle qui offre la meilleure clarté pour un parcours et un fond de carte donnés.