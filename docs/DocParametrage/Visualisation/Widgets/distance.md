# 📏 Paramètre : Affichage Distance (Visualisation)

Ce document détaille le paramètre `distance`, qui permet de définir si le widget de distance doit être affiché ou masqué lors du lancement de la visualisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `distance` contrôle la visibilité initiale du compteur kilométrique affiché à l'écran. Ce widget indique la distance parcourue par rapport à la distance totale du parcours.

-   **Libellé**: Distance
-   **Type**: Booléen
-   **Valeur par défaut**: true (Activé)

## ⚖️ Justification : Pourquoi contrôler l'affichage de la distance ?

L'affichage de la distance est une information clé, mais il peut être souhaitable de le masquer dans certains contextes.

### 1. 📍 Suivi de la progression

-   Permet de savoir instantanément où l'on se situe sur le parcours et quelle distance il reste à parcourir.

### 2. 🎥 Immersion visuelle

-   Masquer ce widget permet d'obtenir une vue plus "cinématographique" et moins chargée, idéale pour des enregistrements vidéo ou des présentations purement esthétiques.

---

## ⚠️ Recommandations

-   **Valeur par défaut (Activé)** : Recommandé pour une utilisation standard afin de fournir des repères à l'utilisateur.
-   **Raccourci clavier** : Rappelez-vous que même masqué au démarrage, l'utilisateur peut toujours basculer l'affichage de ce widget avec la touche **D**.
