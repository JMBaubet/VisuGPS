# ⌨️ Paramètre : Incrément du pitch (flèches haut/bas)

Ce document détaille le paramètre `incrementPitch`, qui définit l'incrément en degrés pour l'angle d'inclinaison (pitch) de la caméra lorsque les flèches haut/bas du clavier sont utilisées.

---

## 🎯 Rôle du Paramètre

Le paramètre `incrementPitch` contrôle la granularité des ajustements manuels de l'angle de la caméra (pitch) lors de l'édition.

-   **Libellé**: Incrément Pitch
-   **Type**: Entier
-   **Valeur par défaut**: 1°
-   **Minimum**: 1°
-   **Maximum**: 10°
-   **Unité**: ° (degrés)

## ⚖️ Justification : Pourquoi ajuster l'incrément du pitch ?

L'ajustement de cet incrément permet de contrôler la précision et la rapidité des modifications manuelles de l'angle de la caméra.

### 1. 📐 Précision de l'Ajustement

-   **Valeur faible** (`1°`) : Permet des ajustements très fins du pitch, idéal pour des réglages précis de l'angle de vue.
-   **Valeur élevée** (`> 1°`) : Permet des ajustements plus rapides, utile pour des changements d'angle plus importants.

### 2. 🚀 Rapidité de Réglage

-   Une valeur plus élevée accélère le réglage manuel du pitch.

---

## ⚠️ Recommandations

-   **Valeur par défaut (1°)** : C'est un bon compromis pour un contrôle précis par défaut.
-   **Adapter aux besoins** :
    -   Pour des ajustements très fins, maintenez la valeur à 1°.
    -   Pour des réglages plus rapides, augmentez cette valeur.
