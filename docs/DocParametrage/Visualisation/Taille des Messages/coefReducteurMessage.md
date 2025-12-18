# 📏 Paramètre : Coefficient réducteur message

Ce document détaille le paramètre `coefReducteurMessage`, qui permet de réduire progressivement la largeur estimée des messages au fur et à mesure que leur contenu s'allonge.

---

## 🎯 Rôle du Paramètre

Le paramètre `coefReducteurMessage` sert à amortir la croissance de la largeur du cadre pour les messages longs. Il divise le coefficient de largeur de base par un facteur dépendant de la longueur du texte.

-   **Libellé**: Coefficient réducteur message
-   **Type**: Réel
-   **Valeur par défaut**: 0.025
-   **Minimum**: 0.0
-   **Maximum**: 0.1

## ⚖️ Justification : Pourquoi utiliser un coefficient réducteur ?

Une estimation purement linéaire de la largeur (NbCaractères * LargeurMoyenne) tend à surestimer la place nécessaire pour les textes longs, car l'œil humain tolère (et l'affichage gère) une densité de caractère légèrement plus élevée sur les longs blocs.

### 1. 🤏 Optimisation de l'espace

-   Pour les messages courts, une largeur "confortable" est nécessaire pour inclure les marges.
-   Pour les messages longs, conserver cette même proportion créerait des cadres immenses et disgracieux couvrant trop de carte.

### 2. 🧮 Formule mathématique

La formule utilisée est :
`Largeur = (NbCaractères * CoefLargeur) / (1 + NbCaractères * CoefRéducteur)`

-   Si `CoefRéducteur` est 0, la largeur est proportionnelle à la longueur.
-   Plus `CoefRéducteur` augmente, plus la courbe de largeur "s'aplatit" pour les longs textes.

---

## ⚠️ Recommandations

-   **Valeur par défaut (0.025)** : Offre un compromis idéal, gardant les messages courts lisibles tout en empêchant les messages longs de devenir démesurés.
-   **Désactivation** : Mettez ce paramètre à **0** pour revenir à un comportement parfaitement linéaire (largeur proportionnelle au nombre de caractères).

