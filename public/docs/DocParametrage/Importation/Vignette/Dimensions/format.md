# 📐 Paramètre : Format de la vignette

Ce document détaille le paramètre `format`, qui contrôle le ratio largeur/hauteur de la vignette 2D générée lors de l'importation d'une trace GPX.

---

## 🎯 Rôle du Paramètre

Le paramètre `format` définit les proportions de la miniature de la trace GPX. Il s'exprime sous forme de ratio (par exemple, "1/1" pour un carré, "16/9" pour un format écran large).

-   **Libellé**: Format de la vignette
-   **Type**: Chaîne de caractères (String)
-   **Valeur par défaut**: "1/1"
-   **Options disponibles**: "1/1", "4/3", "16/9"

## ⚖️ Justification : Pourquoi choisir un format de vignette ?

Le choix du format influence l'esthétique de la vignette et la manière dont la trace est présentée.

### 1. 🖼️ Esthétique et Présentation

-   **"1/1" (Carré)** : Idéal pour une présentation équilibrée, souvent utilisé pour les icônes ou les aperçus compacts.
-   **"4/3" (Classique)** : Un format traditionnel, proche de celui des écrans d'ordinateur plus anciens ou de certaines photos.
-   **"16/9" (Écran Large)** : Adapté aux écrans modernes et aux vidéos, il peut être pertinent pour des traces très étendues horizontalement.

### 2. 🗺️ Visualisation de la Trace

Le format peut affecter la manière dont la trace est "cadrée" dans la vignette.
-   Une trace très longue et étroite sera mieux mise en valeur dans un format "16/9".
-   Une trace plus compacte ou circulaire pourrait être mieux représentée dans un format "1/1".

### 3. 📏 Cohérence Visuelle

Maintenir un format cohérent pour toutes vos vignettes peut améliorer l'organisation visuelle de votre liste de circuits.

---

## ⚠️ Recommandations

-   **Valeur par défaut ("1/1")** : C'est un format polyvalent qui convient à la plupart des traces.
-   **Expérimenter** : N'hésitez pas à essayer les différents formats pour voir lequel met le mieux en valeur vos traces GPX.
-   **Considérer la forme de la trace** : Choisissez un format qui correspond à la géométrie générale de votre trace pour éviter un cadrage inesthétique.
