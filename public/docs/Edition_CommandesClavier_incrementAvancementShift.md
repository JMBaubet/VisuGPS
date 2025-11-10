# ⌨️ Paramètre : Incrément d'avancement (Shift + flèches gauche/droite)

Ce document détaille le paramètre `incrementAvancementShift`, qui définit le nombre de points de tracking par lesquels la caméra avance ou recule lorsque les flèches gauche/droite du clavier sont utilisées en combinaison avec la touche `Shift`.

---

## 🎯 Rôle du Paramètre

Le paramètre `incrementAvancementShift` offre un moyen d'accélérer le déplacement de la caméra le long de la trace. Il permet de parcourir la trace par pas plus importants que l'incrément standard.

-   **Libellé**: Incrément Avancement (Shift)
-   **Type**: Entier
-   **Valeur par défaut**: 10
-   **Minimum**: 1
-   **Maximum**: 50

## ⚖️ Justification : Pourquoi un incrément d'avancement avec Shift ?

L'utilisation de la touche `Shift` pour modifier l'incrément d'avancement est une convention ergonomique courante pour les actions "rapides".

### 1. 🚀 Rapidité de Navigation

-   Permet de se déplacer rapidement le long de la trace sans avoir à changer les paramètres, idéal pour sauter de longues sections ou naviguer rapidement.

### 2. ⚡ Gain de Temps

-   Réduit le nombre de pressions sur les touches nécessaires pour atteindre un point éloigné.

### 3. 🤔 Flexibilité

-   Offre une flexibilité à l'utilisateur, lui permettant de choisir entre un déplacement précis et un déplacement rapide selon le contexte.

---

## ⚠️ Recommandations

-   **Valeur par défaut (10)** : C'est un bon compromis qui offre un déplacement significativement plus rapide que l'incrément standard.
-   **Adapter aux besoins** :
    -   Si vous éditez de très longues traces et avez besoin de traverser rapidement de grandes distances, vous pouvez augmenter cette valeur.
    -   Si un déplacement légèrement plus rapide que l'incrément standard est suffisant, vous pouvez la réduire.
