# 🖱️ Paramètre : Incrément de zoom (molette de la souris)

Ce document détaille le paramètre `incrementZoom`, qui définit l'incrément de zoom de la caméra lorsque la molette de la souris est utilisée en mode édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `incrementZoom` contrôle la granularité des ajustements manuels du niveau de zoom de la caméra, en utilisant la molette de la souris.

-   **Libellé**: Incrément Zoom
-   **Type**: Réel (Float)
-   **Valeur par défaut**: 0.1
-   **Minimum**: 0.01
-   **Maximum**: 1.0
-   **Décimales**: 2

## ⚖️ Justification : Pourquoi ajuster l'incrément de zoom ?

L'ajustement de cet incrément permet de contrôler la précision et la rapidité des modifications manuelles du niveau de zoom.

### 1. 🔎 Précision de l'Ajustement

-   **Valeur faible** (`0.01`) : Permet des ajustements très fins du zoom, idéal pour des réglages précis de la distance de la caméra.
-   **Valeur élevée** (`> 0.1`) : Permet des ajustements plus rapides, utile pour des changements de zoom plus importants.

### 2. 🚀 Rapidité de Réglage

-   Une valeur plus élevée accélère le réglage manuel du zoom.

---

## ⚠️ Recommandations

-   **Valeur par défaut (0.1)** : C'est un bon compromis pour un contrôle précis par défaut.
-   **Adapter aux besoins** :
    -   Pour des ajustements très fins, réduisez la valeur.
    -   Pour des réglages plus rapides, augmentez cette valeur.
