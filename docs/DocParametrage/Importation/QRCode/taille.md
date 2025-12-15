# 🖼️ Paramètre : Taille du QR code généré (px)

Ce document détaille le paramètre `taille`, qui définit la taille en pixels du côté du QR code généré pour le partage de la trace.

---

## 🎯 Rôle du Paramètre

Le paramètre `taille` contrôle la dimension (largeur et hauteur, car un QR code est carré) du QR code qui peut être généré pour une trace GPX.

-   **Libellé**: Taille du QR code
-   **Type**: Entier
-   **Valeur par défaut**: 512 px
-   **Minimum**: 400 px
-   **Maximum**: 2048 px
-   **Unité**: px (pixels)

## ⚖️ Justification : Pourquoi ajuster la taille du QR code ?

La taille du QR code est un compromis entre sa lisibilité (facilité de scan) et son encombrement visuel.

### 1. 📸 Lisibilité et Facilité de Scan

-   **Grande taille** (`> 512 px`) : Un QR code plus grand est généralement plus facile à scanner, surtout à distance ou avec des appareils photo de moindre qualité. Il contient également plus de "pixels" (modules) pour encoder l'information, ce qui peut améliorer sa robustesse aux erreurs.
-   **Petite taille** (`< 512 px`) : Un QR code plus petit peut être plus difficile à scanner, mais il prend moins de place.

### 2. 🖼️ Encombrement Visuel

Un QR code de grande taille peut être visuellement imposant, tandis qu'un QR code plus petit est plus discret.

---

## ⚠️ Recommandations

-   **Valeur par défaut (512 px)** : C'est un bon compromis qui offre une bonne lisibilité pour la plupart des usages sans être excessivement grand.
-   **Adapter à l'utilisation** :
    -   Si le QR code est destiné à être affiché sur un grand écran ou scanné à distance, une taille plus grande peut être préférable.
    -   Si le QR code est intégré dans un document ou un espace restreint, une taille plus petite peut être plus appropriée.
