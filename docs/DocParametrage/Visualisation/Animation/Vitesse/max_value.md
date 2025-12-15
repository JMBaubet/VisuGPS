# ⏱️ Paramètre : Vitesse maximale de l'animation

Ce document détaille le paramètre `max_value`, qui définit le multiplicateur de vitesse maximal pour l'animation de la trace en mode visualisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `max_value` établit la limite supérieure de la plage de vitesse que l'utilisateur peut sélectionner pour l'animation. Il s'agit d'un multiplicateur appliqué à la vitesse de base définie par le paramètre `vitesse` (ms/km).

-   **Libellé**: Vitesse maximale (x)
-   **Type**: Réel (Float)
-   **Valeur par défaut**: 20.0
-   **Minimum**: 1.0
-   **Maximum**: 100.0
-   **Décimales**: 2

## ⚖️ Justification : Pourquoi définir une vitesse maximale ?

La définition d'une vitesse maximale est importante pour permettre à l'utilisateur d'accélérer l'animation pour des survols rapides, tout en évitant des vitesses qui rendraient l'animation inintelligible ou difficile à contrôler.

### 1. 🚀 Survol Rapide

-   Une vitesse maximale élevée permet de parcourir rapidement de longues traces ou de passer des sections moins intéressantes.

### 2. ⚙️ Contrôle Utilisateur

-   Fournit une limite haute raisonnable pour le contrôle de la vitesse via l'interface utilisateur (par exemple, un slider).
-   Évite que l'animation ne devienne trop rapide pour être suivie visuellement.

---

## ⚠️ Recommandations

-   **Valeur par défaut (20.0)** : Cette valeur offre une accélération significative, permettant de survoler la plupart des traces en un temps raisonnable.
-   **Ne pas monter trop haut** : Une valeur excessive pourrait rendre l'animation saccadée ou impossible à suivre.
-   **Cohérence avec `slider_step`** : Assurez-vous que le pas du slider (`slider_step`) est cohérent avec la plage définie par `min_value` et `max_value`.
