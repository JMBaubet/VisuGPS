# 📍 Paramètre : Afficher la distance sur la vignette

Ce document détaille le paramètre `presenceDistance`, qui contrôle l'affichage des marqueurs de distance sur la trace GPX dans la vignette 2D générée.

---

## 🎯 Rôle du Paramètre

Le paramètre `presenceDistance` est un interrupteur (booléen) qui détermine si des marqueurs numériques (1km, 2km, etc.) doivent être affichés le long de la trace sur la vignette.

-   **Libellé**: Afficher la distance
-   **Type**: Booléen
-   **Valeur par défaut**: `true`

## ⚖️ Justification : Pourquoi afficher la distance ?

L'affichage des marqueurs de distance permet d'avoir une indication rapide de l'échelle de la trace directement sur la vignette.

### 1. 📏 Repère Spatial

Les marqueurs de distance fournissent un repère visuel immédiat sur la longueur de la trace et la progression le long du parcours.

### 2. 👀 Lisibilité

Sur des traces longues, ces marqueurs aident à mieux appréhender la géométrie du parcours.

### 3. 🖼️ Esthétique

Certains utilisateurs peuvent préférer une vignette plus épurée sans ces marqueurs.

---

## ⚠️ Recommandations

-   **Activé par défaut** : Il est généralement utile de laisser ce paramètre activé pour une meilleure compréhension de la trace.
-   **Désactiver pour une vignette épurée** : Si vous préférez une vignette plus minimaliste, vous pouvez désactiver ce paramètre.
