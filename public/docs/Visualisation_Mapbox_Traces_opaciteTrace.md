# 👻 Paramètre : Opacité de la trace (Visualisation)

Ce document détaille le paramètre `opaciteTrace`, qui définit le niveau de transparence de la trace GPX affichée sur la carte en mode visualisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `opaciteTrace` contrôle la transparence de la ligne représentant la trace GPX. Une valeur de 1.0 signifie une opacité totale (complètement visible), tandis qu'une valeur de 0.0 signifie une transparence totale (complètement invisible).

-   **Libellé**: Opacité de la trace
-   **Type**: Réel (Float)
-   **Valeur par défaut**: 0.8
-   **Minimum**: 0.0
-   **Maximum**: 1.0
-   **Décimales**: 2

## ⚖️ Justification : Pourquoi ajuster l'opacité de la trace en visualisation ?

L'ajustement de l'opacité de la trace permet de mieux l'intégrer au fond de carte et de gérer la superposition visuelle avec d'autres éléments.

### 1. 🏞️ Intégration au Fond de Carte

-   Une opacité légèrement inférieure à 1.0 (par exemple, 0.8) permet de voir les détails du fond de carte (routes, bâtiments, relief) à travers la trace, améliorant ainsi le contexte visuel.

### 2. 🎨 Esthétique et Clarté

-   Permet de personnaliser l'apparence de la trace pour qu'elle soit moins intrusive ou, au contraire, plus discrète, selon les préférences de l'utilisateur.
-   Peut aider à distinguer la trace principale d'autres éléments secondaires si plusieurs couches sont affichées.

---

## ⚠️ Recommandations

-   **Valeur par défaut (0.8)** : C'est un bon compromis qui rend la trace bien visible tout en permettant de percevoir le fond de carte.
-   **Adapter au fond de carte** : Si le fond de carte est très chargé, une opacité plus faible peut être utile. Si le fond de carte est très simple, une opacité plus élevée peut être préférée.
