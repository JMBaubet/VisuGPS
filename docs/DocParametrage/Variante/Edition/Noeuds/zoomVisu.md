# 🔍 Paramètre : Zoom Visibilité Nœuds

Ce document détaille le paramètre `zoomVisuNode`, contrôlant l'affichage des points d'ancrage.

---

## 🎯 Rôle du Paramètre

Le paramètre `zoomVisuNode` définit le niveau de zoom minimal à partir duquel les points d'ancrage (nœuds) de la trace maîtresse deviennent visibles.

-   **Libellé**: Zoom visibilité nœuds
-   **Type**: Entier
-   **Valeur par défaut**: 14
-   **Minimum**: 10
-   **Maximum**: 20

## ⚖️ Justification : Pourquoi régler ce seuil ?

L'affichage de milliers de points sur une carte peut nuire à la lisibilité si l'on est trop dézoomé.

### 1. 🧹 Clarté de la carte (LOD)

-   **Vue d'ensemble** : En vue dézoomée (< 14), on veut voir la trace globale sans "bruit".
-   **Vue de détail** : En vue zoomée (>= 14), on a besoin de voir les points précis pour cliquer dessus et faire des raccordements exacts.

---

## ⚠️ Recommandations

-   **Ajustement** : Augmentez cette valeur (ex: 15 ou 16) si vous trouvez que les points apparaissent trop tôt et encombrent l'écran.
-   **Performance** : Afficher trop de points simultanément peut ralentir l'affichage, ce seuil sert aussi d'optimisation.
