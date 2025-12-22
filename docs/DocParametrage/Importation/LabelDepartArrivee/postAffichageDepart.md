# ⏱️ Paramètre : Post affichage départ

Ce document détaille le paramètre `Post affichage départ`, qui gère la durée de visibilité du message de départ.

---

## 🎯 Rôle du Paramètre

Ce paramètre définit la distance (ou durée) pendant laquelle le message de départ reste affiché à l'écran après le début du mouvement. Il s'exprime en "incréments" de tracking.

-   **Type**: Entier
-   **Valeur par défaut**: 10
-   **Plage recommandée**: 1 - 20

---

## 💡 Utilisation

Un "incrément" correspond à un point de tracking généré.
- Une valeur plus élevée signifie que le message reste visible plus longtemps alors que la caméra commence à avancer.
- Une valeur faible le fait disparaître quasi immédiatement.

Cela permet de laisser au spectateur le temps de lire "Départ" avant que l'action ne commence vraiment.
