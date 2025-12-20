# 🏙️ Paramètre : Affichage Communes (Visualisation)

Ce document détaille le paramètre `communes`, qui permet de définir si le widget indiquant le nom de la commune traversée doit être affiché ou masqué lors du lancement de la visualisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `communes` contrôle la visibilité initiale de l'indicateur de localité. Ce widget affiche dynamiquement le nom de la commune survolée par la caméra.

-   **Libellé**: Communes
-   **Type**: Booléen
-   **Valeur par défaut**: true (Activé)

## ⚖️ Justification : Pourquoi contrôler l'affichage des communes ?

L'information géographique administrative apporte un contexte supplémentaire à la visualisation 3D.

### 1. 🗺️ Contexte Territorial

-   Il est souvent difficile d'identifier un lieu précis uniquement avec la vue satellite. Le nom de la commune aide à se repérer géographiquement.

### 2. 🏰 Intérêt Touristique

-   Pour les parcours de découverte ou touristiques, connaître les villes et villages traversés est une information essentielle.

---

## ⚠️ Recommandations

-   **Zones sauvages** : Si votre trace parcourt des zones inhabitées où le découpage communal a peu de sens visuel, vous pouvez désactiver ce widget pour alléger l'interface.
-   **Raccourci clavier** : Ce widget peut être activé ou désactivé à tout moment pendant la visualisation avec la touche **C**.
