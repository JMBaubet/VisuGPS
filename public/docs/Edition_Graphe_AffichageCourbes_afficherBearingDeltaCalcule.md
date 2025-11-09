# 📊 Paramètre : Afficher Delta Bearing (Calculé)

Ce document détaille le paramètre `afficherBearingDeltaCalcule`, qui contrôle l'affichage de la courbe du delta de cap calculé (valeurs originales) sur le graphe d'édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `afficherBearingDeltaCalcule` est un interrupteur (booléen) qui, lorsqu'activé, affiche une courbe représentant les changements de cap (bearing) bruts, tels que calculés directement à partir des données de la trace GPX.

-   **Libellé**: Afficher Delta Bearing (Calculé)
-   **Type**: Booléen
-   **Valeur par défaut**: `false`

## ⚖️ Justification : Pourquoi afficher le delta de cap calculé ?

L'affichage de cette courbe est utile pour l'analyse des données brutes et la compréhension des variations de direction de la trace avant toute édition.

### 1. 📈 Analyse des Données Brutes

-   Permet de visualiser les changements de direction réels de la trace, sans l'influence des lissages ou des modifications manuelles.
-   Utile pour identifier les zones où la trace tourne brusquement ou change de direction de manière significative.

### 2. 🔍 Comparaison

-   Peut être comparé à la courbe du "Delta Bearing (Édité)" pour évaluer l'impact des modifications apportées.

---

## ⚠️ Recommandations

-   **Désactivé par défaut** : Ce paramètre est désactivé par défaut car il représente des données brutes qui peuvent être "bruyantes" et moins pertinentes pour une visualisation générale.
-   **Activer pour l'analyse** : Activez ce paramètre si vous souhaitez analyser en détail les variations de cap originales de la trace.
