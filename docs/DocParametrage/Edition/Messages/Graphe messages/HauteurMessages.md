# 📏 Paramètre : Hauteur des messages sur le graphe (px)

Ce document détaille le paramètre `hauteurGraphique`, qui contrôle la hauteur visuelle des événements de type "Message" sur le graphique temporel de la vue d'édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `hauteurGraphique` définit la hauteur en pixels de chaque rectangle représentant un message sur le graphe.

-   **Libellé**: Hauteur des messages sur le graphe (px)
-   **Type**: Entier
-   **Valeur par défaut**: 10 px
-   **Minimum**: 3 px
-   **Maximum**: 20 px
-   **Unité**: px (pixels)

## ⚖️ Justification : Pourquoi ajuster cette hauteur ?

Ce paramètre permet de personnaliser la densité et la lisibilité du graphique des événements.

### 1. 👀 Lisibilité Visuelle

-   **Augmenter la hauteur** (ex: `15px`) rend chaque événement de message plus visible et plus facile à sélectionner, surtout sur des écrans à haute résolution ou si vous avez des difficultés à cliquer sur de petites zones.
-   **Réduire la hauteur** (ex: `5px`) permet d'afficher plus de messages sans qu'ils se chevauchent verticalement, ce qui est utile si vous avez de nombreux messages regroupés sur une courte portion de la trace.

### 2.  densità d'information

Le graphique des messages organise les événements sur plusieurs "couloirs" (lanes) pour éviter les superpositions. Une hauteur plus faible permet de loger plus de couloirs dans le même espace vertical, affichant ainsi plus d'informations sans avoir besoin de faire défiler.

---


## ⚠️ Recommandations

-   **Valeur par défaut (10 px)** : Offre un bon équilibre entre la visibilité et la densité d'information pour la plupart des usages.
-   **Ajuster selon le besoin** : Si vous travaillez avec un grand nombre de messages qui se chevauchent, une valeur plus faible peut être préférable. Si la sélection des messages est difficile, augmentez la hauteur.
