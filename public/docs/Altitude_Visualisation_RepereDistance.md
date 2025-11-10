# 📏 Paramètre : Intervalle des repères de distance (Profil Altimétrique)

Ce document détaille le paramètre `RepereDistance`, qui définit l'intervalle en kilomètres pour les repères affichés sur l'axe des distances (abscisses) du profil altimétrique.

---

## 🎯 Rôle du Paramètre

Le paramètre `RepereDistance` contrôle la fréquence des marqueurs de distance sur l'axe horizontal du profil. Une valeur plus petite affichera plus de repères, offrant une granularité plus fine, tandis qu'une valeur plus grande réduira le nombre de repères.

-   **Libellé**: Intervalle repère distance (km)
-   **Type**: Entier
-   **Valeur par défaut**: 10 km
-   **Minimum**: 1 km
-   **Maximum**: 100 km

## ⚖️ Justification : Pourquoi ajuster l'intervalle des repères de distance ?

L'ajustement de l'intervalle des repères de distance est important pour la lisibilité et la clarté du profil altimétrique, en évitant une surcharge d'informations ou, au contraire, un manque de repères.

### 1. 👀 Lisibilité

-   Pour des traces courtes, un intervalle plus petit (par exemple, 1 km ou 5 km) peut être utile pour mieux situer les points clés.
-   Pour des traces très longues, un intervalle plus grand (par exemple, 20 km ou 50 km) est nécessaire pour éviter que l'axe ne soit surchargé de chiffres.

### 2. 📊 Analyse

-   Aide à l'analyse en fournissant des points de référence réguliers le long de la trace.

---

## ⚠️ Recommandations

-   **Valeur par défaut (10 km)** : Cette valeur offre un bon équilibre pour la plupart des traces, fournissant des repères réguliers sans surcharger le graphique.
-   **Adapter à la longueur de la trace** : Choisissez un intervalle qui correspond à la longueur totale de la trace pour une lisibilité optimale.
