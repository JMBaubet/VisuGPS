# ⏱️ Paramètre : Durée de transition du pitch (ms)

Ce document détaille le paramètre `transitionDuree`, qui contrôle la fluidité de l'animation du pitch (inclinaison) de la caméra lors de la manipulation de la carte à la souris dans la vue d'édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `transitionDuree` définit la durée, en millisecondes, de l'animation qui incline la caméra lorsque l'utilisateur effectue un cliquer-glisser sur la carte.

-   **Libellé**: Durée de transition du pitch (ms)
-   **Type**: Entier
-   **Valeur par défaut**: 50 ms
-   **Plage**: 10 ms - 500 ms

**Note :** Bien que ce paramètre soit actuellement localisé sous `Edition/Evenements/Message` dans l'arborescence des paramètres, il affecte une interaction générale de la caméra et non les messages eux-mêmes.

## ⚖️ Justification : Pourquoi ajuster cette durée ?

Ce paramètre permet de personnaliser la réactivité de la caméra lors de la navigation manuelle sur la carte 3D.

### 1. ⚙️ Comportement de l'Interaction

-   Lorsque vous **maintenez le clic gauche** sur la carte pour la déplacer, la caméra s'anime pour se mettre à un pitch de 0° (vue de dessus).
-   Lorsque vous **relâchez le clic**, la caméra retourne en douceur à son pitch précédent.

`transitionDuree` contrôle la vitesse de ces deux animations.

### 2. ✨ Expérience Utilisateur

-   **Durée courte** (ex: `20 ms`): La transition est quasi-instantanée, rendant la caméra très réactive, ce qui peut être préféré pour des ajustements rapides.
-   **Durée longue** (ex: `300 ms`): La transition est plus douce et cinématique, ce qui peut donner une sensation de fluidité et de polissage à l'interaction.

---



## ⚠️ Recommandations

-   **Valeur par défaut (50 ms)** : Fournit un bon compromis entre réactivité et fluidité.
-   **Ajuster selon vos préférences** : Augmentez la valeur pour des transitions plus douces ou diminuez-la si vous souhaitez une réponse plus immédiate de la caméra.