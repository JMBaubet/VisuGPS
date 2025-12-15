# 🔢 Paramètre : Nombre de circuits par page

Ce document détaille le paramètre `circuitsPerPage`, qui contrôle le nombre de circuits affichés par page sur l'écran d'accueil de l'application.

---

## 🎯 Rôle du Paramètre

Le paramètre `circuitsPerPage` détermine combien de circuits sont visibles simultanément dans la liste de l'écran d'accueil.

-   **Libellé**: Nombre de circuits par page
-   **Type**: Entier
-   **Valeur par défaut**: 10
-   **Minimum**: 4
-   **Maximum**: 20

## ⚖️ Justification : Pourquoi ajuster ce nombre ?

Ce paramètre permet d'adapter l'affichage de la liste des circuits à vos préférences et à la taille de votre écran.

### 1. 👀 Lisibilité et Confort Visuel

-   **Écrans plus grands** : Sur un grand écran, vous pourriez préférer afficher plus de circuits pour réduire le besoin de pagination.
-   **Écrans plus petits** : Sur un écran plus petit ou si vous préférez une interface moins chargée, un nombre réduit de circuits par page peut améliorer la lisibilité.

### 2. 🚀 Performance de l'Interface

Bien que l'impact soit généralement minime, afficher un très grand nombre d'éléments complexes (comme les vignettes de circuit) peut légèrement affecter la fluidité de l'interface. Ajuster ce paramètre permet de trouver un équilibre.

---

## ⚠️ Recommandations

-   **Valeur par défaut (10)** : C'est un bon compromis pour la plupart des utilisateurs et des tailles d'écran.
-   **Personnalisation** : N'hésitez pas à expérimenter avec différentes valeurs pour trouver celle qui correspond le mieux à votre confort visuel et à votre flux de travail.
