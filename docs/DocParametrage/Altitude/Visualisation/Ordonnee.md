# 📏 Paramètre : Échelle Ordonnée (Profil Altimétrique)

Ce document détaille le paramètre `Ordonnee`, qui définit l'échelle de l'axe des ordonnées (altitude) du profil altimétrique, exprimée en nombre de pixels pour 10 mètres de dénivelé.

---

## 🎯 Rôle du Paramètre

Le paramètre `Ordonnee` contrôle la compression ou l'étirement vertical du profil altimétrique. Une valeur plus élevée étire le graphique verticalement, rendant les variations d'altitude plus prononcées, tandis qu'une valeur plus faible les comprime.

-   **Libellé**: Échelle Ordonnée (px/10m)
-   **Type**: Entier
-   **Valeur par défaut**: 1 px/10m
-   **Minimum**: 1 px/10m
-   **Maximum**: 10 px/10m

## ⚖️ Justification : Pourquoi ajuster l'échelle des ordonnées ?

L'ajustement de l'échelle des ordonnées est important pour optimiser la lisibilité du profil altimétrique en fonction du dénivelé de la trace et du niveau de détail souhaité.

### 1. 👀 Lisibilité des Dénivelés

-   Pour des traces avec un faible dénivelé, une valeur plus élevée peut être utile pour "étirer" le graphique verticalement et mieux visualiser les petites variations d'altitude.
-   Pour des traces avec un fort dénivelé, une valeur plus faible peut être nécessaire pour que l'ensemble du profil tienne dans l'espace disponible.

### 2. 📊 Représentation Visuelle

-   Permet d'adapter la représentation visuelle du profil pour qu'elle soit plus pertinente et informative.

---

## ⚠️ Recommandations

-   **Valeur par défaut (1 px/10m)** : Cette valeur offre un bon équilibre pour la plupart des traces, permettant de visualiser les dénivelés sans trop les exagérer.
-   **Adapter au dénivelé de la trace** :
    -   Pour des traces avec peu de dénivelé, vous pouvez augmenter cette valeur.
    -   Pour des traces avec beaucoup de dénivelé, vous devrez peut-être la réduire.
