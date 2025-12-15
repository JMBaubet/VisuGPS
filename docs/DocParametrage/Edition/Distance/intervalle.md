# 🔎 Paramètre : Intervalle des bornes kilométriques

Ce document détaille le paramètre `intervalle`, qui définit la distance en kilomètres entre chaque borne kilométrique affichée le long de la trace GPX.

---

## 🎯 Rôle du Paramètre

Le paramètre `intervalle` contrôle la fréquence à laquelle les bornes kilométriques sont positionnées sur la trace. Cela permet d'adapter la granularité des repères en fonction de la longueur et du type de parcours.

-   **Libellé**: Intervalle (km)
-   **Type**: Entier
-   **Valeur par défaut**: 10 km
-   **Minimum**: 1 km
-   **Maximum**: 20 km

## ⚖️ Justification : Pourquoi ajuster l'intervalle ?

L'ajustement de l'intervalle des bornes kilométriques est important pour optimiser la lisibilité de la trace et la pertinence des informations affichées, sans surcharger visuellement la carte.

### 1. 📏 Lisibilité

Un intervalle approprié évite que les bornes ne se chevauchent ou ne soient trop espacées, améliorant ainsi la clarté visuelle.

### 2. 🗺️ Contexte du parcours

Pour les parcours courts, un intervalle plus petit peut être pertinent, tandis que pour les longs parcours, un intervalle plus grand réduit le nombre de repères affichés.

## ⚠️ Recommandations

-   **Traces courtes (< 30 km)** : Un intervalle de 5 km peut être envisagé.
-   **Traces moyennes (30-100 km)** : L'intervalle par défaut de 10 km est généralement un bon choix.
-   **Traces longues (> 100 km)** : Un intervalle de 15 ou 20 km peut être préférable pour une meilleure vue d'ensemble.