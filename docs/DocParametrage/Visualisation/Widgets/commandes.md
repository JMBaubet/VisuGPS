# 🎮 Paramètre : Affichage Commandes (Visualisation)

Ce document détaille le paramètre `commandes`, qui permet de définir si le panneau de contrôle de lecture doit être affiché ou masqué lors du lancement de la visualisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `commandes` contrôle la visibilité initiale de la barre d'outils flottante contenant les boutons de lecture (Play, Pause, Reset) et le curseur de vitesse.

-   **Libellé**: Commandes
-   **Type**: Booléen
-   **Valeur par défaut**: false (Masqué au lancement)

## ⚖️ Justification : Pourquoi contrôler l'affichage des commandes ?

L'interface de contrôle est nécessaire pour interagir avec l'animation, mais peut être masquée pour un rendu plus épuré.

### 1. 🕹️ Interactivité

-   Donne un accès immédiat aux fonctions principales pour mettre en pause, changer la vitesse ou revenir en arrière.

### 2. 📺 Mode "Kiosque" ou Démonstration

-   Si l'animation est destinée à tourner en boucle sur un écran (avec reprise automatique), masquer les commandes offre un rendu plus professionnel et évite les distractions visuelles.

---

## ⚠️ Recommandations

-   **Interactivité** : Laissez ce paramètre activé pour une utilisation standard sur ordinateur de bureau.
-   **Raccourci clavier** : Le panneau peut être affiché ou masqué à la volée en appuyant sur la touche **Espace**.
