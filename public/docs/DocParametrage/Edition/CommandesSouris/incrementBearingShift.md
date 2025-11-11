# 🖱️ Paramètre : Incrément de cap (molette + Shift)

Ce document détaille le paramètre `incrementBearingShift`, qui définit l'incrément de rotation du cap de la caméra lorsque la molette de la souris est utilisée en combinaison avec la touche `Shift`.

---

## 🎯 Rôle du Paramètre

Le paramètre `incrementBearingShift` agit comme un multiplicateur de vitesse pour la rotation du cap. Il permet d'effectuer des changements d'angle rapides et importants sans avoir à modifier la valeur de base (`incrementBearing`).

Cette fonctionnalité est active uniquement lorsque les deux conditions suivantes sont remplies :
1. Le curseur de la souris se trouve sur le widget des onglets de contrôle (Caméra, Pause/Zooms, Message).
2. L'onglet "Caméra" est sélectionné.

-   **Libellé**: Incrément Cap (Shift)
-   **Type**: Réel (Float)
-   **Valeur par défaut**: 5.0
-   **Minimum**: 1.0
-   **Maximum**: 20.0
-   **Décimales**: 1

## ⚖️ Justification : Pourquoi utiliser un incrément "Shift" ?

L'utilisation d'un incrément alternatif avec la touche `Shift` offre une double vitesse de contrôle, ce qui améliore l'efficacité du flux de travail en édition.

### 1. ⚡ Contrôle à deux vitesses

-   **Sans Shift** (avec `incrementBearing`) : Permet un contrôle fin et précis pour les ajustements détaillés.
-   **Avec Shift** (avec `incrementBearingShift`) : Permet un contrôle rapide pour les grands changements d'orientation, comme faire un demi-tour ou s'orienter rapidement dans une nouvelle direction.

### 2. ⚙️ Efficacité

-   Évite d'avoir à changer constamment la valeur de l'incrément de base dans les paramètres pour passer d'un ajustement fin à un ajustement rapide.

---

## ⚠️ Recommandations

-   **Valeur par défaut (5.0)** : Fournit un multiplicateur de vitesse notable par rapport à la valeur par défaut de `incrementBearing` (1.0), ce qui est efficace pour la plupart des situations.
-   **Maintenir un ratio significatif** : Il est recommandé de conserver une valeur pour `incrementBearingShift` qui soit au moins 3 à 5 fois supérieure à celle de `incrementBearing` pour que la différence de vitesse soit perceptible et utile.
