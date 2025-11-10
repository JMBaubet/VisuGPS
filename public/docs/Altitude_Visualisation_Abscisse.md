# 📏 Paramètre : Échelle Abscisse (Profil Altimétrique)

Ce document détaille le paramètre `Abscisse`, qui définit l'échelle de l'axe des abscisses (distance) du profil altimétrique, exprimée en nombre de pixels pour 100 mètres.

---

## 🎯 Rôle du Paramètre

Le paramètre `Abscisse` contrôle la compression ou l'étirement horizontal du profil altimétrique. Une valeur plus élevée étire le graphique horizontalement, rendant les détails de distance plus visibles, tandis qu'une valeur plus faible le comprime.

-   **Libellé**: Échelle Abscisse (px/100m)
-   **Type**: Entier
-   **Valeur par défaut**: 2 px/100m
-   **Minimum**: 1 px/100m
-   **Maximum**: 10 px/100m

## ⚖️ Justification : Pourquoi ajuster l'échelle des abscisses ?

L'ajustement de l'échelle des abscisses est important pour optimiser la lisibilité du profil altimétrique en fonction de la longueur de la trace et du niveau de détail souhaité.

### 1. 👀 Lisibilité des Distances

-   Pour des traces courtes ou pour une analyse détaillée de sections spécifiques, une valeur plus élevée peut être utile pour "étirer" le graphique et mieux visualiser les variations sur de courtes distances.
-   Pour des traces très longues, une valeur plus faible peut être nécessaire pour que l'ensemble du profil tienne dans l'espace disponible.

### 2. 📊 Représentation Visuelle

-   Permet d'adapter la représentation visuelle du profil pour qu'elle soit plus pertinente et informative.

---

## ⚠️ Recommandations

-   **Valeur par défaut (2 px/100m)** : Cette valeur offre un bon équilibre pour la plupart des traces, permettant de visualiser les détails sans trop étirer le graphique.
-   **Adapter à la longueur de la trace** :
    -   Pour des traces courtes, vous pouvez augmenter cette valeur.
    -   Pour des traces très longues, vous devrez peut-être la réduire.
