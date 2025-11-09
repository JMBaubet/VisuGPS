# 🎨 Paramètre : Couleur des marqueurs de distance

Ce document détaille le paramètre `couleurPinDistance`, qui contrôle la couleur de base des marqueurs numériques de distance affichés sur la trace GPX dans la vignette 2D générée.

---

## 🎯 Rôle du Paramètre

Le paramètre `couleurPinDistance` définit la couleur de base utilisée pour les marqueurs de distance. L'application utilise cette couleur et y applique des nuances (lighten/darken) en fonction de la décade du marqueur (1-9, 10-19, etc.) pour créer une progression visuelle.

-   **Libellé**: Couleur des marqueurs de distance
-   **Type**: Couleur (Material Design, Strict)
-   **Valeur par défaut**: "red"

## ⚖️ Justification : Pourquoi choisir une couleur pour les marqueurs ?

Le choix de la couleur des marqueurs est important pour leur visibilité et pour la clarté de la progression visuelle.

### 1. 👀 Visibilité et Contraste

-   La couleur doit offrir un bon contraste avec le fond de carte et la couleur de la trace pour que les marqueurs soient facilement identifiables.
-   La valeur par défaut "red" est une couleur forte qui ressort bien.

### 2. 📊 Progression Visuelle

L'application utilise des nuances de cette couleur de base pour indiquer la progression par décade (par exemple, les marqueurs 1-9 seront plus clairs, 10-19 un peu moins, etc.). Choisir une couleur de base avec une bonne gamme de nuances dans la palette Material Design est donc important.

### 3. 🎨 Cohérence Visuelle

Vous pouvez choisir une couleur qui correspond à vos préférences personnelles ou qui s'harmonise avec d'autres éléments visuels de l'application.

---

## ⚠️ Recommandations

-   **Valeur par défaut ("red")** : C'est un bon choix pour la visibilité et la progression visuelle.
-   **Utiliser les couleurs Material Design** : Le paramètre accepte les noms de couleurs définis par Material Design (par exemple, "blue", "green", "purple").
-   **Éviter les couleurs trop claires ou trop sombres** : Une couleur de base au milieu de la gamme de nuances Material Design permettra une meilleure différenciation des décades.
