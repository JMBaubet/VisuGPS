# 🔍 Paramètre : Zoom Europe (Initialisation Visualisation)

Ce document détaille le paramètre `zoomEurope`, qui définit le niveau de zoom initial appliqué à la vue de l'Europe au début de la séquence d'initialisation de l'animation en mode visualisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `zoomEurope` contrôle l'étendue géographique visible lorsque la carte est centrée sur l'Europe. Un zoom plus faible (valeur numérique plus petite) montre une zone plus vaste, tandis qu'un zoom plus élevé (valeur numérique plus grande) se rapproche.

-   **Libellé**: Zoom Europe
-   **Type**: Entier
-   **Valeur par défaut**: 5
-   **Minimum**: 0
-   **Maximum**: 22

## ⚖️ Justification : Pourquoi ajuster le zoom initial sur l'Europe ?

L'ajustement du zoom initial sur l'Europe permet de contrôler la portée de la vue d'ensemble avant de se focaliser sur la trace, influençant l'effet cinématographique.

### 1. 🗺️ Contexte Géographique

-   Un zoom approprié permet de bien visualiser l'ensemble du continent européen, offrant un contexte géographique clair avant le survol vers la trace.

### 2. 🎥 Effet Cinématique

-   Un zoom trop élevé pourrait masquer l'effet de "fly-in" depuis une vue large.
-   Un zoom trop faible pourrait rendre l'Europe trop petite et moins reconnaissable.

---

## ⚠️ Recommandations

-   **Valeur par défaut (5)** : Cette valeur offre une bonne vue d'ensemble de l'Europe, permettant de reconnaître le continent.
-   **Expérimentation** : Vous pouvez ajuster cette valeur pour modifier l'effet d'introduction de l'animation.
