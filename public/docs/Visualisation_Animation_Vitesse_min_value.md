# ⏱️ Paramètre : Vitesse minimale de l'animation

Ce document détaille le paramètre `min_value`, qui définit le multiplicateur de vitesse minimal pour l'animation de la trace en mode visualisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `min_value` établit la limite inférieure de la plage de vitesse que l'utilisateur peut sélectionner pour l'animation. Il s'agit d'un multiplicateur appliqué à la vitesse de base définie par le paramètre `vitesse` (ms/km).

-   **Libellé**: Vitesse minimale (x)
-   **Type**: Réel (Float)
-   **Valeur par défaut**: 0.1
-   **Minimum**: 0.05
-   **Maximum**: 1.0
-   **Décimales**: 2

## ⚖️ Justification : Pourquoi définir une vitesse minimale ?

La définition d'une vitesse minimale est importante pour garantir que l'animation reste perceptible et utile, même à son réglage le plus lent.

### 1. 👀 Visibilité et Analyse

-   Une vitesse trop faible pourrait rendre l'animation imperceptible ou extrêmement longue, ce qui nuirait à l'expérience utilisateur et à la capacité d'analyser la trace.
-   La valeur minimale permet de s'assurer que l'animation progresse toujours à un rythme qui permet de suivre le parcours.

### 2. ⚙️ Contrôle Utilisateur

-   Fournit une limite basse raisonnable pour le contrôle de la vitesse via l'interface utilisateur (par exemple, un slider).

---

## ⚠️ Recommandations

-   **Valeur par défaut (0.1)** : Cette valeur permet une animation très lente, utile pour l'observation détaillée de sections spécifiques de la trace.
-   **Ne pas descendre trop bas** : Évitez de définir une valeur minimale trop proche de zéro, car cela pourrait rendre l'animation quasi immobile.
-   **Cohérence avec `slider_step`** : Assurez-vous que le pas du slider (`slider_step`) est cohérent avec la plage définie par `min_value` et `max_value`.
