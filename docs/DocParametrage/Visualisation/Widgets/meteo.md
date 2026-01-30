# 🌦️ Paramètre : Affichage Météo (Visualisation)

Ce document détaille le paramètre `meteo`, qui permet de définir si le widget météo dynamqiue doit être affiché ou masqué lors du lancement de la visualisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `meteo` contrôle la visibilité initiale des informations météorologiques (température, conditions, vent) superposées à la carte. Ces données sont calculées dynamiquement en fonction de la position sur le parcours et de l'heure de passage simulée.

-   **Libellé**: Météo
-   **Type**: Booléen
-   **Valeur par défaut**: true (Activé)

## ⚖️ Justification : Pourquoi contrôler l'affichage de la météo ?

La météo ajoute une couche de réalisme et d'immersion à la visualisation en projetant les conditions atmosphériques prévues sur le terrain 3D.

### 1. 🌡️ Immersion et Contexte

-   Permet de visualiser les conditions que rencontrera l'utilisateur (ou a rencontrées) aux différents points du tracé.

### 2. 🧩 Clarté de l'interface

-   Bien que compact, le widget peut être masqué pour libérer de l'espace visuel, notamment lors de l'analyse purement géographique ou technique du parcours.

---

## ⚠️ Recommandations

-   **Vidéos et Présentations** : Activé pour un rendu visuel plus riche et professionnel.
-   **Raccourci clavier** : Le widget peut être affiché ou masqué à tout moment avec la touche **M**.
-   **Pilotage à distance** : La visibilité peut également être contrôlée via la télécommande mobile.
