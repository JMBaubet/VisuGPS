# 📏 Paramètre : Distance maximale départ/arrivée (m)

Ce document détaille le paramètre `distanceMax`, qui définit la distance maximale en mètres entre le point de départ et le point d'arrivée d'une trace GPX pour qu'ils soient considérés comme "proches" sur la vignette 2D générée.

---

## 🎯 Rôle du Paramètre

Le paramètre `distanceMax` est utilisé pour déterminer si le départ et l'arrivée d'une trace sont suffisamment proches pour être représentés par un seul marqueur sur la vignette, plutôt que par deux marqueurs distincts.

-   **Libellé**: Distance max départ/arrivée (m)
-   **Type**: Entier
-   **Valeur par défaut**: 250 m
-   **Minimum**: 0 m
-   **Unité**: m (mètres)

## ⚖️ Justification : Pourquoi regrouper les marqueurs ?

Le regroupement des marqueurs de départ et d'arrivée est une question de clarté visuelle et de lisibilité de la vignette, en particulier pour les traces en boucle ou très courtes.

### 1. 👀 Clarté Visuelle

-   Pour les traces en boucle (où le départ et l'arrivée sont très proches), afficher deux marqueurs distincts pourrait les faire se chevaucher, rendant la vignette confuse.
-   Un seul marqueur combiné (`couleurDépartArrivée`) est plus clair dans ce scénario.

### 2. 🖼️ Esthétique

Éviter le chevauchement des marqueurs améliore l'esthétique générale de la vignette.

---

## ⚠️ Recommandations

-   **Valeur par défaut (250 m)** : C'est une valeur raisonnable pour considérer le départ et l'arrivée comme proches.
-   **Ajuster selon la précision des traces** : Si vos traces sont très précises et que vous souhaitez une distinction même pour de très courtes distances, vous pouvez réduire cette valeur.
-   **Augmenter pour les boucles larges** : Pour des traces en boucle avec un point de départ/arrivée légèrement décalé mais que vous souhaitez tout de même regrouper, vous pouvez augmenter cette valeur.
