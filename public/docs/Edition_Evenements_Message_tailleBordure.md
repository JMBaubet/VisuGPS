# 📏 Paramètre : Épaisseur de la bordure du message (Graphe)

Ce document détaille le paramètre `tailleBordure`, qui définit l'épaisseur en pixels de la bordure du message affiché lors d'un événement de message sur le graphe d'édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `tailleBordure` contrôle l'épaisseur de la bordure de la boîte de message, influençant sa visibilité et son impact visuel.

-   **Libellé**: Taille de la bordure
-   **Type**: Entier
-   **Valeur par défaut**: 4 px
-   **Minimum**: 0 px (pas de bordure)
-   **Maximum**: 10 px
-   **Pas (Step)**: 1 px
-   **Unité**: px (pixels)

## ⚖️ Justification : Pourquoi ajuster l'épaisseur de la bordure du message ?

L'ajustement de l'épaisseur de la bordure permet de trouver un équilibre entre la visibilité du message et son encombrement visuel.

### 1. 👀 Visibilité et Distinction

-   Une bordure plus épaisse rend le message plus visible et le fait ressortir davantage.
-   Une bordure plus fine (ou absente) rend le message plus discret.

### 2. 🎨 Esthétique

-   Permet de personnaliser l'apparence du message selon les préférences de l'utilisateur.

---

## ⚠️ Recommandations

-   **Valeur par défaut (4 px)** : C'est un bon compromis qui offre une bordure visible sans être trop imposante.
-   **Adapter aux préférences** : Vous pouvez augmenter l'épaisseur si vous souhaitez une bordure plus prononcée, ou la réduire (jusqu'à 0 pour la supprimer) si vous préférez un style plus minimaliste.
