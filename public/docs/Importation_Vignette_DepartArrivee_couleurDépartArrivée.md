# 🎨 Paramètre : Couleur du marqueur départ/arrivée (proches) sur la vignette

Ce document détaille le paramètre `couleurDépartArrivée`, qui contrôle la couleur du marqueur unique utilisé lorsque les points de départ et d'arrivée de la trace GPX sont considérés comme proches sur la vignette 2D générée.

---

## 🎯 Rôle du Paramètre

Le paramètre `couleurDépartArrivée` définit la couleur du marqueur visuel combiné qui représente à la fois le début et la fin de la trace, lorsque leur distance est inférieure ou égale à `distanceMax`.

-   **Libellé**: Couleur du marqueur départ/arrivée (proches)
-   **Type**: Couleur (Material Design)
-   **Valeur par défaut**: "blue-darken-2"

## ⚖️ Justification : Pourquoi une couleur combinée ?

L'utilisation d'une couleur spécifique pour le marqueur combiné améliore la clarté visuelle.

### 1. 👀 Clarté Visuelle

-   Indique clairement que le point marqué représente à la fois le départ et l'arrivée, évitant toute confusion.
-   Fournit une distinction visuelle par rapport aux marqueurs de départ et d'arrivée individuels s'ils étaient affichés séparément.

### 2. 🎨 Cohérence Visuelle

Vous pouvez choisir une couleur qui correspond à vos préférences personnelles ou qui s'harmonise avec d'autres éléments visuels de l'application.

---

## ⚠️ Recommandations

-   **Valeur par défaut ("blue-darken-2")** : C'est un choix standard et visuellement distinct.
-   **Utiliser les couleurs Material Design** : Le paramètre accepte les noms de couleurs définis par Material Design (par exemple, "blue", "green", "purple").
-   **Assurer le contraste** : Évitez d'utiliser une couleur trop proche de celle du fond de carte ou de la trace pour garantir une bonne visibilité.
