# 🔨 Paramètre : Fenêtre du Filtre Médian

Ce document détaille le paramètre `altitude_smoothing_median_window`, qui contrôle la puissance du "rabotage" des pics d'altitude isolés.

---

> [!TIP]
>
>
> **USAGE** : Ce filtre est la première étape du nettoyage. Il est particulièrement efficace pour supprimer les "pointes" uniques (un seul point aberrant) sans altérer la forme globale du relief.

---

## 🎯 Rôle du Paramètre

Le filtre médian regarde une fenêtre glissante de `N` points autour de chaque position. Il remplace la valeur centrale par la valeur *médiane* de cette fenêtre.
Concrètement, si votre GPS enregistre une série d'altitudes comme `[100, 101, 150 (erreur), 102, 103]`, le filtre médian va ignorer le `150` et le remplacer par une valeur voisine cohérente.

-   **Libellé**: Fenêtre Médiane (Dégrossissage)
-   **Type**: Entier (Nombre de points)
-   **Valeur par défaut**: 5
-   **Minimum**: 1 (Inactif)
-   **Unité**: Points

## ⚖️ Justification : Pourquoi un filtre médian ?

Contrairement à une moyenne classique qui serait influencée par une erreur énorme (la moyenne monterait un peu), la médiane **ignore totalement** les valeurs extrêmes si elles sont minoritaires dans la fenêtre. C'est l'outil idéal pour supprimer le "bruit impulsionnel" typique des capteurs électroniques.

---

## ⚠️ Recommandations

-   **Valeur par défaut (5)** : Regarde 2 points avant et 2 points après. Suffisant pour éliminer un pic isolé de 1 ou 2 points.
-   **Augmenter la valeur (ex: 9 ou 15)** : Si vous avez des séries d'erreurs plus longues (plusieurs secondes de données fausses). Attention, une valeur trop grande peut commencer à "ariser" les sommets pointus de votre parcours.
