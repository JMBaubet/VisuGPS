# 🌈 Paramètre : Colorer selon la pente

Ce document détaille le paramètre `Colorer selon la pente`, qui active le rendu visuel dynamique de la difficulté du terrain.

---

## 🎯 Rôle du Paramètre

Ce paramètre est un interrupteur (ON/OFF) qui détermine si la trace de la variante doit être colorée en fonction du pourcentage de pente de chaque segment.

-   **Type** : Booléen
-   **Valeur par défaut** : Vrai (Activé)

## ⚙️ Fonctionnement

-   **Activé (Vrai)** : La trace utilise le dégradé de couleurs défini globalement pour l'application (ex: Vert pour plat, Rouge pour montée raide). Cela permet d'évaluer rapidement la difficulté de la variante proposée.
-   **Désactivé (Faux)** : La trace est affichée avec une couleur unique, définie par le paramètre `Couleur de la trace`.

## ⚠️ Recommandations

-   Laissez ce paramètre activé pour mieux visualiser le profil altimétrique de votre variante lors de sa conception.
-   Désactivez-le si vous préférez une vue plus épurée ou si vous souhaitez simplement distinguer le tracé géographique sans information de relief.
