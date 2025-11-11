# 📏 Paramètre : Largeur de la vignette (px)

Ce document détaille le paramètre `largeur`, qui contrôle la largeur en pixels de la vignette 2D générée lors de l'importation d'une trace GPX.

---

## 🎯 Rôle du Paramètre

Le paramètre `largeur` définit la dimension horizontale de la miniature de la trace GPX qui est créée et stockée pour être affichée dans la liste des circuits.

-   **Libellé**: Largeur de la vignette
-   **Type**: Entier
-   **Valeur par défaut**: 512 px
-   **Minimum**: 400 px
-   **Maximum**: 1280 px
-   **Unité**: px (pixels)

## ⚖️ Justification : Pourquoi ajuster la largeur de la vignette ?

Ce paramètre permet d'équilibrer la qualité visuelle de la vignette et les ressources utilisées (taille du fichier, temps de génération).

### 1. ✨ Qualité Visuelle

Une largeur plus grande (`1280 px`) permet d'obtenir une vignette avec plus de détails et une meilleure résolution, ce qui peut être appréciable sur des écrans haute définition.

### 2. 💾 Taille du Fichier et Stockage

Une largeur plus importante entraîne une taille de fichier plus grande pour la vignette. Si vous importez un grand nombre de traces, cela peut avoir un impact sur l'espace de stockage utilisé par l'application.

### 3. ⏱️ Temps de Génération

La génération d'une vignette de plus grande taille prend légèrement plus de temps. Pour des performances optimales lors de l'importation, une taille modérée est préférable.

---

## ⚠️ Recommandations

-   **Valeur par défaut (512 px)** : C'est un bon compromis offrant une qualité visuelle satisfaisante sans impacter excessivement les performances ou le stockage.
-   **Adapter à vos besoins** :
    -   Si la qualité visuelle est primordiale et que l'espace de stockage n'est pas une contrainte, vous pouvez augmenter cette valeur.
    -   Si vous privilégiez la rapidité d'importation et un faible encombrement, vous pouvez la réduire (sans descendre en dessous du minimum recommandé pour conserver une vignette exploitable).
