# 🔍 Paramètre : Marge autour de la trace (Fin)

Ce document détaille le paramètre `margeFinalisation`, qui définit l'espace (padding) en pixels conservé autour de la trace lors du dé-zoom final, après l'arrivée.

---

## 🎯 Rôle du Paramètre

Le paramètre `margeFinalisation` contrôle le niveau de recul de la caméra lors de la vue d'ensemble du parcours à la fin de l'animation. Une marge plus élevée (valeur numérique plus grande) éloigne la caméra, créant plus d'espace vide autour de la trace, tandis qu'une marge plus faible (valeur numérique plus petite) rapproche la caméra, remplissant l'écran avec la trace.

-   **Libellé**: Marge autour de la trace (Fin)
-   **Type**: Entier
-   **Unité**: px
-   **Valeur par défaut**: 80
-   **Minimum**: 20
-   **Maximum**: 140

## ⚖️ Justification : Pourquoi ajuster la marge finale ?

L'ajustement de la marge permet d'optimiser la présentation globale du parcours une fois l'animation terminée, pour récapitulatif ou conclusion.

### 1. 🖼️ Cadrage Récapitulatif

-   Offre une vue d'ensemble claire et stable de l'intégralité du trajet parcouru.
-   Permet de voir les messages de fin de parcours dans leur contexte géographique complet.

### 2. 🛡️ Gestion de l'Interface de Fin

-   À la fin de l'animation, certains éléments d'interface peuvent disparaître ou changer. La marge assure que la trace reste parfaitement cadrée et visible.
-   Facilite la capture d'écran ou l'analyse globale post-parcours.

---

## ⚠️ Recommandations

-   **Valeur par défaut (80 px)** : Recommandée pour la plupart des usages, elle assure que la trace complète est bien centrée et visible.
-   **Messages volants** : Si vous utilisez de grands messages de fin (popups), une marge plus importante peut aider à éviter qu'ils ne soient coupés ou masquent la trace.
-   **Capture d'écran finale** : Ajustez cette marge si vous souhaitez réaliser une capture d'écran "clean" pour illustrer votre parcours.
