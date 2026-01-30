# ⭐ Paramètre : Nombre de favoris

Ce document détaille le paramètre `NombreFavoris`, qui définit le nombre maximum de circuits que l'utilisateur peut marquer comme favoris pour un accès rapide.

---

## 🎯 Rôle du Paramètre

Le paramètre `NombreFavoris` contrôle la limite imposée au système de mise en avant des circuits. Les circuits marqués comme favoris sont affichés en haut de la liste sur l'écran d'accueil, identifiés par une étoile jaune.

-   **Libellé**: Nombre de favoris
-   **Type**: Entier
-   **Valeur par défaut**: 6
-   **Minimum**: 1
-   **Maximum**: 20
-   **Unité**: nombre de circuits

## ⚖️ Justification : Pourquoi limiter le nombre de favoris ?

Ce paramètre permet de maintenir l'efficacité du système de favoris et l'ergonomie de l'écran d'accueil.

### 1. 🚀 Accès Rapide
Le but des favoris est de permettre un accès immédiat aux circuits les plus importants. Si le nombre de favoris est trop élevé, la liste perd de son utilité car elle devient elle-même difficile à parcourir.

### 2. 📱 Organisation de l'Écran d'Accueil
Les favoris étant affichés en haut de la liste, un nombre excessif pourrait repousser les autres circuits (ceux récemment ajoutés par exemple) trop loin vers le bas ou sur les pages suivantes.

### 3. 🎯 Focus
Limiter les favoris encourage l'utilisateur à ne conserver que les traces réellement pertinentes du moment, assurant une gestion plus ordonnée de sa bibliothèque.

---

## ⚠️ Recommandations

-   **Valeur par défaut (6)** : Offre un bon compromis pour une visibilité immédiate sans encombrer la première page.
-   **Gestion dynamique** : Si vous atteignez la limite, le système vous demandera de retirer un favori existant avant d'en ajouter un nouveau.
