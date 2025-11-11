# 📊 Paramètre : Afficher Somme Delta Bearing (Calculé)

Ce document détaille le paramètre `afficherBearingTotalDeltaCalcule`, qui contrôle l'affichage de la courbe de la somme cumulée des deltas de cap calculés (valeurs originales) sur le graphe d'édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `afficherBearingTotalDeltaCalcule` est un interrupteur (booléen) qui, lorsqu'activé, affiche une courbe représentant la somme cumulée des changements de cap (bearing) bruts. Cette courbe donne une idée de la rotation totale de la caméra le long de la trace avant toute édition.

-   **Libellé**: Afficher Somme Delta Bearing (Calculé)
-   **Type**: Booléen
-   **Valeur par défaut**: `false`

## ⚖️ Justification : Pourquoi afficher la somme du delta de cap calculé ?

L'affichage de cette courbe est utile pour comprendre la dynamique globale des rotations de la caméra et pour la comparaison avec les données éditées.

### 1. 📈 Analyse de la Rotation Totale

-   Permet de visualiser la "quantité" de rotation que la caméra devrait effectuer le long de la trace, sans l'influence des lissages ou des modifications manuelles.
-   Utile pour identifier les sections de la trace où la caméra tourne le plus, ou où elle maintient un cap relativement stable.

### 2. 🔍 Comparaison

-   Peut être comparé à la courbe de la "Somme Delta Bearing (Éditée)" pour évaluer l'impact des modifications apportées sur la rotation globale de la caméra.

---

## ⚠️ Recommandations

-   **Désactivé par défaut** : Ce paramètre est désactivé par défaut car il représente des données brutes qui peuvent être complexes à interpréter sans un contexte d'édition.
-   **Activer pour l'analyse** : Activez ce paramètre si vous souhaitez analyser en détail la rotation cumulative de la caméra à partir des données originales.
