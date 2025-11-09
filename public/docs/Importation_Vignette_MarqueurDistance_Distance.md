# 📏 Paramètre : Intervalle des marqueurs de distance (km)

Ce document détaille le paramètre `Distance`, qui contrôle l'intervalle en kilomètres entre les marqueurs de distance affichés sur la trace GPX dans la vignette 2D générée.

---

## 🎯 Rôle du Paramètre

Le paramètre `Distance` définit la fréquence à laquelle les marqueurs numériques (1km, 2km, etc.) apparaissent le long de la trace sur la vignette.

-   **Libellé**: Intervalle distance
-   **Type**: Entier
-   **Valeur par défaut**: 10 km
-   **Minimum**: 1 km
-   **Unité**: km (kilomètres)

## ⚖️ Justification : Pourquoi ajuster l'intervalle ?

L'ajustement de l'intervalle permet d'adapter la densité des marqueurs de distance à la longueur de la trace et à la taille de la vignette.

### 1. 👀 Lisibilité

-   **Traces courtes** : Pour des traces courtes, un intervalle plus petit (ex: 1 km ou 5 km) peut être pertinent pour avoir des repères plus fréquents.
-   **Traces longues** : Pour des traces très longues, un intervalle plus grand (ex: 20 km ou plus) évitera de surcharger la vignette avec trop de marqueurs, ce qui pourrait nuire à la lisibilité.

### 2. 🖼️ Esthétique

La densité des marqueurs influence l'esthétique générale de la vignette. Un bon équilibre rend la vignette informative sans être encombrée.

---

## ⚠️ Recommandations

-   **Valeur par défaut (10 km)** : C'est un bon compromis pour la plupart des traces.
-   **Adapter à la longueur de la trace** :
    -   Pour des traces de moins de 20 km, essayez 1 km ou 5 km.
    -   Pour des traces de plus de 50 km, un intervalle de 20 km ou 50 km pourrait être plus approprié.
