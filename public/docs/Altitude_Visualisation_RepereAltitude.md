# ⛰️ Paramètre : Intervalle des repères d'altitude (Profil Altimétrique)

Ce document détaille le paramètre `RepereAltitude`, qui définit l'intervalle en mètres pour les repères affichés sur l'axe des altitudes (ordonnées) du profil altimétrique.

---

## 🎯 Rôle du Paramètre

Le paramètre `RepereAltitude` contrôle la fréquence des marqueurs d'altitude sur l'axe vertical du profil. Une valeur plus petite affichera plus de repères, offrant une granularité plus fine, tandis qu'une valeur plus grande réduira le nombre de repères.

-   **Libellé**: Intervalle repère altitude (m)
-   **Type**: Entier
-   **Valeur par défaut**: 500 m
-   **Minimum**: 50 m
-   **Maximum**: 2000 m

## ⚖️ Justification : Pourquoi ajuster l'intervalle des repères d'altitude ?

L'ajustement de l'intervalle des repères d'altitude est important pour la lisibilité et la clarté du profil altimétrique, en évitant une surcharge d'informations ou, au contraire, un manque de repères.

### 1. 👀 Lisibilité

-   Pour des traces avec un faible dénivelé, un intervalle plus petit (par exemple, 50 m ou 100 m) peut être utile pour mieux visualiser les variations d'altitude.
-   Pour des traces avec un fort dénivelé, un intervalle plus grand (par exemple, 1000 m ou 2000 m) est nécessaire pour éviter que l'axe ne soit surchargé de chiffres.

### 2. 📊 Analyse

-   Aide à l'analyse en fournissant des points de référence réguliers pour les niveaux d'altitude.

---

## ⚠️ Recommandations

-   **Valeur par défaut (500 m)** : Cette valeur offre un bon équilibre pour la plupart des traces, fournissant des repères réguliers sans surcharger le graphique.
-   **Adapter au dénivelé de la trace** : Choisissez un intervalle qui correspond au dénivelé total de la trace pour une lisibilité optimale.
