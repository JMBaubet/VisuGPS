# 🌍 Paramètre : Centre de l'Europe (Initialisation Visualisation)

Ce document détaille le paramètre `centerEurope`, qui définit les coordonnées (longitude, latitude) du centre de l'Europe utilisées comme point de départ pour la vue initiale de l'animation en mode visualisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `centerEurope` spécifie le point géographique sur lequel la carte sera centrée au tout début de la séquence d'initialisation de l'animation. C'est la première étape du survol initial avant de se diriger vers la trace.

-   **Libellé**: Centre de l'Europe
-   **Type**: Coordonnées (Array de deux réels : [longitude, latitude])
-   **Valeur par défaut**: [2.3522, 48.8566] (Paris)

## ⚖️ Justification : Pourquoi définir un centre de l'Europe ?

La définition d'un centre de l'Europe comme point de départ initial offre une transition visuelle agréable et un contexte géographique clair avant de se focaliser sur la trace.

### 1. 🗺️ Contexte Géographique

-   Permet de situer la trace dans un contexte plus large, en partant d'une vue générale de l'Europe avant de zoomer sur la zone spécifique de la trace.

### 2. 🎥 Effet Cinématique

-   Crée un effet "fly-in" cinématographique, améliorant l'immersion de l'utilisateur dans l'animation.

---

## ⚠️ Recommandations

-   **Valeur par défaut ([2.3522, 48.8566])** : Ces coordonnées correspondent à Paris, un point central et reconnaissable en Europe.
-   **Ne pas modifier sans raison** : Il est généralement recommandé de conserver cette valeur par défaut, car elle sert de point de référence commun.
-   **Format des coordonnées** : Assurez-vous que les coordonnées sont au format [longitude, latitude].
