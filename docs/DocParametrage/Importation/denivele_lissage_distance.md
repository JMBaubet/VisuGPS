# ⛰️ Paramètre : Distance de lissage du dénivelé (m)

Ce document détaille le paramètre `denivele_lissage_distance`, qui définit la distance minimale en mètres entre deux points pour le calcul du dénivelé positif.

---

> [!IMPORTANT]
> **PARAMÈTRE CRITIQUE** : Ce réglage influe directement sur l'intégrité des statistiques (distance, dénivelé positif) enregistrées dans votre base de données circuits. Une modification peut altérer significativement l'interprétation de vos performances sportives.

---

## 🎯 Rôle du Paramètre

Le paramètre `denivele_lissage_distance` est utilisé pour filtrer les petites variations d'altitude entre des points très proches. Si la distance entre deux points consécutifs est inférieure à cette valeur, leur différence d'altitude ne sera pas prise en compte dans le calcul du dénivelé positif total.

-   **Libellé**: Distance de lissage du dénivelé
-   **Type**: Entier
-   **Valeur par défaut**: 10 m
-   **Minimum**: 0 m
-   **Maximum**: 100 m
-   **Unité**: m (mètres)

## ⚖️ Justification : Pourquoi lisser le dénivelé ?

Le lissage du dénivelé est essentiel pour obtenir des valeurs de dénivelé positif plus réalistes et moins sujettes aux imprécisions des données GPS.

### 1. 📊 Précision des Données GPS

-   Les données d'altitude des GPS peuvent être sujettes à des erreurs et des fluctuations mineures (bruit). Sans lissage, ces petites variations seraient cumulées, entraînant un dénivelé positif total artificiellement élevé.
-   En ignorant les variations sur de très courtes distances, on obtient une mesure plus fidèle du "vrai" dénivelé.

### 2. 📈 Pertinence du Calcul

-   Un dénivelé positif lissé est plus représentatif de l'effort réel fourni sur le terrain, car il ne prend pas en compte les micro-variations d'altitude qui ne sont pas significatives pour un cycliste ou un randonneur.

---

## ⚠️ Recommandations

-   **Valeur par défaut (10 m)** : C'est un bon compromis qui permet de filtrer la plupart des imprécisions sans masquer les montées réelles.
-   **Ajuster selon la qualité des données** :
    -   Si vos traces proviennent d'appareils GPS très précis, vous pouvez réduire cette valeur.
    -   Si vos traces sont issues d'appareils moins précis ou si vous souhaitez un lissage plus agressif, vous pouvez augmenter cette valeur.
-   **Valeur de 0 m** : Désactive complètement le lissage, ce qui peut entraîner un dénivelé positif très élevé et potentiellement irréaliste.
