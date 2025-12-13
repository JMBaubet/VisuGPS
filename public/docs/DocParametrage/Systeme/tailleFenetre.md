# 🖼️ Paramètre : Taille fenêtre

Ce document détaille le paramètre `tailleFenetre`, qui définit les dimensions de la fenêtre principale de l'application au démarrage.

---

## 🎯 Rôle du Paramètre

Le paramètre `tailleFenetre` impose une résolution spécifique (largeur x hauteur) à l'application dès son lancement.

-   **Libellé**: Taille fenêtre
-   **Type**: Liste
-   **Valeur par défaut**: 1920x1080
-   **Valeurs disponibles**: De 1024x768 à 5120x2880 (formats 4:3, 16:9, 16:10)

## ⚖️ Justification : Pourquoi définir la taille ?

Ce paramètre assure que l'application s'ouvre toujours avec des dimensions prévisibles et adaptées à votre espace de travail.

### 1. 🖥️ Adaptation à l'Écran
Certains utilisateurs préfèrent que l'application occupe tout l'espace disponible (ex: 2K ou 4K), tandis que d'autres préfèrent une fenêtre plus compacte pour garder d'autres outils visibles.

### 2. 🎥 Enregistrement Vidéo
Pour les créateurs de contenu qui capturent l'écran, fixer une taille standard (comme 1920x1080) garantit des vidéos au format 16:9 parfait, sans avoir à redimensionner la fenêtre manuellement.

### 3. 🔍 Cohérence Visuelle (Retina/HiDPI)
Sur les écrans haute densité (comme les Mac Retina), ce réglage utilise des "points logiques". Ainsi, "1920x1080" correspondra à une taille visuelle standard Full HD, indépendamment de la densité de pixels réelle de votre moniteur.

---

## ⚠️ Recommandations

-   **Standard (1920x1080)** : Idéal pour travailler sur des écrans Full HD ou pour préparer des captures vidéo.
-   **Grand Écran** : Si vous avez un moniteur 4K, vous pouvez choisir des résolutions plus élevées (ex: 2560x1440 ou 3840x2160) pour profiter d'une surface de carte maximale.
