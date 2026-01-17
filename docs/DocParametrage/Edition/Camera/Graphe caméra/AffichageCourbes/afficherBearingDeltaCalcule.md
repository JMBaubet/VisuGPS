# 📈 Paramètre : Afficher Delta Bearing (Calculé)

Ce document détaille le paramètre `Afficher Delta Bearing (Calculé)`.

---

## 🎯 Rôle du Paramètre

Ce paramètre **booléen** contrôle l'affichage de la courbe représentant le "Delta Bearing Calculé" sur le graphe d'édition de la caméra.

-   **Type**: Booléen
-   **Valeur par défaut**: `false`

## 💡 Signification

Le "Delta Bearing" correspond à la variation de cap (direction) entre deux points successifs de la trace. La version "Calculée" est celle issue directement de l'analyse brute de la trace GPX, avant tout lissage ou modification manuelle.

---

## ⚠️ Recommandations

-   Utile principalement pour le **débogage** ou pour comprendre les accoups de la caméra.
-   Laisser désactivé pour une vue plus claire si vous n'analysez pas les micro-variations de direction.
