# 🖼️ Paramètre : Style de la carte au lancement (Mapbox)

Ce document détaille le paramètre `styleLancement`, qui définit le style de carte Mapbox utilisé lors de la phase d'initialisation de la visualisation (vue de l'Europe et vue globale de la trace).

---

## 🎯 Rôle du Paramètre

Le paramètre `styleLancement` permet de choisir un style de carte Mapbox spécifique pour les vues à grande échelle. Il est utilisé pour offrir une présentation claire et lisible lors des phases d'introduction et de conclusion de l'animation.

-   **Libellé**: Style de la carte au lancement
-   **Type**: Liste de sélection
-   **Valeur par défaut**: "mapbox://styles/mapbox/standard"

## ⚖️ Justification : Pourquoi utiliser un style différent au lancement ?

L'utilisation d'un style de carte dynamique permet d'adapter l'environnement visuel au niveau de zoom et au contexte géographique.

### 1. 🌍 Lisibilité Globale
Les styles de type "Standard" ou "Plan" sont optimisés pour la lecture à petite échelle (zoom faible). Ils offrent un rendu plus propre et professionnel pour l'introduction de l'animation, mettant en valeur les frontières et les grandes infrastructures sans surcharger l'image.

### 2. 🛰️ Détails de la Trace
Le basculement automatique vers le style détaillé (typiquement satellite) lors du zoom sur le point de départ permet de bénéficier d'une immersion maximale pendant la progression du parcours, là où les détails du terrain sont les plus pertinents.

### 3. 🌐 Styles Disponibles

Les styles suivants sont disponibles :
- `mapbox://styles/mapbox/standard`
- `mapbox://styles/mapbox/streets-v12`
- `mapbox://styles/mapbox/outdoors-v12`
- `mapbox://styles/mapbox/light-v11`
- `mapbox://styles/mapbox/dark-v11`
- `mapbox://styles/mapbox/satellite-v9`
- `mapbox://styles/mapbox/satellite-streets-v12`
- `mapbox://styles/mapbox/navigation-day-v1`
- `mapbox://styles/mapbox/navigation-night-v1`

---

## ⚠️ Recommandations

-   **Valeur par défaut ("mapbox://styles/mapbox/standard")** : Ce style est vivement recommandé pour le lancement car il offre la meilleure lisibilité pour les vues continentales et globales.
-   **Cohérence visuelle** : Sélectionnez un style de lancement qui contraste agréablement avec le style utilisé pendant la visualisation de la trace pour marquer visuellement la transition.
-   **Entièrement automatisé** : Notez que le passage d'un style à l'autre est géré par l'application lors des transitions entre la vue globale et la vue de suivi (Km 0).
