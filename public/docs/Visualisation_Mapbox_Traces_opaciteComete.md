# 👻 Paramètre : Opacité de la comète (Visualisation)

Ce document détaille le paramètre `opaciteComete`, qui définit le niveau de transparence de la "comète" (la trace lumineuse) qui suit l'avancement de la caméra le long de la trace GPX en mode visualisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `opaciteComete` contrôle la transparence de la comète. Une opacité appropriée permet de la rendre visible et distincte sans masquer excessivement le fond de carte ou la trace principale.

-   **Libellé**: Opacité de la comète
-   **Type**: Réel (Float)
-   **Valeur par défaut**: 1.0
-   **Minimum**: 0.0
-   **Maximum**: 1.0
-   **Décimales**: 2

## ⚖️ Justification : Pourquoi ajuster l'opacité de la comète ?

L'ajustement de l'opacité de la comète est essentiel pour son intégration visuelle et sa fonction d'indicateur de progression.

### 1. 👀 Visibilité et Contraste

-   Une opacité élevée (proche de 1.0) assure que la comète est bien visible et se détache du fond de carte, ce qui est important pour suivre l'animation.
-   Une opacité plus faible peut être utilisée pour un effet plus subtil ou si la comète est très épaisse.

### 2. 🎨 Esthétique

-   Permet de personnaliser l'apparence de l'animation pour qu'elle corresponde mieux aux préférences de l'utilisateur.

---

## ⚠️ Recommandations

-   **Valeur par défaut (1.0)** : Une opacité totale est souvent préférable pour la comète afin de maximiser sa visibilité en tant qu'indicateur de progression.
-   **Adapter au fond de carte et à la trace** : Si la comète est très lumineuse ou épaisse, une légère réduction de l'opacité peut améliorer l'équilibre visuel.
