# 🌊 Paramètre : Fenêtre de Moyenne Glissante

Ce document détaille le paramètre `altitude_smoothing_avg_window`, qui définit l'intensité du lissage final de la courbe d'altitude.

---

> [!TIP]
>
>
> **USAGE** : Ce filtre intervient *après* le filtre médian. Son but n'est pas de supprimer les erreurs, mais d'adoucir les transitions pour rendre l'animation "soyeuse".

---

## 🎯 Rôle du Paramètre

Le filtre de moyenne glissante calcule, pour chaque point, la moyenne des altitudes de ses `N` voisins. Cela a pour effet de gommer les petites aspérités ("bruit") du signal GPS et de transformer une courbe en escalier ou tremblotante en une courbe fluide.

-   **Libellé**: Fenêtre Moyenne (Lissage)
-   **Type**: Entier (Nombre de points)
-   **Valeur par défaut**: 5
-   **Minimum**: 1 (Inactif)
-   **Unité**: Points

## ⚖️ Justification : Pourquoi lisser par moyenne ?

Même après avoir retiré les pics aberrants (via le filtre médian), un enregistrement GPS brut reste souvent "nerveux" (petites variations de +/- 1m constantes).
Ce lissage est purement esthétique et ergonomique :
1.  Il évite que la caméra ne tremble en suivant ces micro-variations.
2.  Il rend les graphiques de profil d'altitude plus lisibles et professionnels.

---

## ⚠️ Recommandations

-   **Valeur par défaut (5)** : Offre un bon équilibre. La courbe reste fidèle au terrain mais perd son aspect "dent de scie".
-   **Augmenter la valeur** : Si vous voulez un effet très "cinématographique" et fluide, au risque d'arrondir un peu trop les changements de pente brusques (pied d'un col).
