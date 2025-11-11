# 🖼️ Paramètre : Taille de la vignette

Ce document détaille le paramètre `TailleVignette`, qui contrôle la largeur en pixels de la vignette 2D affichée au survol d'un circuit dans la liste de l'écran d'accueil.

---

## 🎯 Rôle du Paramètre

Le paramètre `TailleVignette` définit la dimension horizontale de la miniature de la trace GPX qui apparaît lorsque vous survolez un élément de la liste des circuits.

-   **Libellé**: Taille de la vignette
-   **Type**: Entier
-   **Valeur par défaut**: 512 px
-   **Minimum**: 400 px
-   **Maximum**: 1024 px
-   **Unité**: px (pixels)

## ⚖️ Justification : Pourquoi ajuster la taille de la vignette ?

Ce paramètre permet d'équilibrer la richesse visuelle et l'utilisation de l'espace à l'écran.

### 1. ✨ Détail Visuel

Une vignette plus grande (`1024 px`) permet de visualiser plus de détails de la trace et du fond de carte, ce qui peut être utile pour identifier rapidement un circuit.

### 2. 📏 Utilisation de l'Espace Écran

Une vignette trop grande peut occuper une part importante de l'écran, réduisant l'espace disponible pour la liste des circuits elle-même ou d'autres éléments de l'interface. Une taille plus petite (`400 px`) est plus discrète.

### 3. 🚀 Performance de l'Interface

Bien que les vignettes soient pré-générées, leur affichage dynamique peut avoir un léger impact sur les performances, surtout si de nombreuses vignettes sont affichées rapidement. Une taille plus petite peut contribuer à une interface plus réactive sur des systèmes moins puissants.

---

## ⚠️ Recommandations

-   **Valeur par défaut (512 px)** : C'est un bon équilibre entre le détail visuel et l'utilisation de l'espace.
-   **Adapter à votre écran** : Sur un écran haute résolution, vous pourriez préférer une vignette plus grande. Sur un écran plus petit, une taille réduite pourrait être plus confortable.
