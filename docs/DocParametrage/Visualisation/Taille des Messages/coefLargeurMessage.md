# 📏 Paramètre : Coefficient largeur message

Ce document détaille le paramètre `coefLargeurMessage`, qui permet d'ajuster la largeur du cadre du message en fonction de la longueur du texte.

---

## 🎯 Rôle du Paramètre

Le paramètre `coefLargeurMessage` est un multiplicateur appliqué à la taille de la police pour estimer la largeur moyenne d'un caractère. Il détermine la largeur de base du cadre avant réduction éventuelle pour les messages longs.

-   **Libellé**: Coefficient largeur message
-   **Type**: Réel
-   **Valeur par défaut**: 0.7
-   **Minimum**: 0.3
-   **Maximum**: 1.5

## ⚖️ Justification : Pourquoi ajuster ce coefficient ?

La largeur réelle d'un texte dépend fortement de la police utilisée et de la nature des caractères (lettres larges vs étroites). Ce paramètre permet de calibrer le cadre pour qu'il soit ni trop lâche, ni trop serré.

### 1. 📐 Ajustement fin

-   Une valeur plus élevée élargit le cadre, ce qui "aère" le texte.
-   Une valeur plus faible resserre le cadre autour du texte.

### 2. 🔠 Adaptation à la police

-   Si vous changez la police de caractères, ce coefficient devra probablement être ajusté pour correspondre à la chasse (largeur) moyenne de la nouvelle police.

---

## ⚠️ Recommandations

-   **Valeur par défaut (0.7)** : Calibrée pour la police standard (Roboto) avec une bonne marge de sécurité.
-   **Ajustement** : Si vos messages semblent systématiquement tronqués ou si le cadre semble vide à droite, ajustez ce coefficient par pas de 0.05.

