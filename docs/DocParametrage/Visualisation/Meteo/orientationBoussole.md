# 🧭 Paramètre : Orientation de la boussole

Ce document détaille le paramètre `Orientation de la boussole`, qui configure le référentiel utilisé par le widget boussole dynamique.

---

## 🎯 Rôle du Paramètre

Ce paramètre détermine ce que représente le "Haut" de la boussole affichée à l'écran : est-ce la direction de la route (votre cap de déplacement) ou la direction dans laquelle la caméra regarde ?

- **Libellé**: Orientation de la boussole
- **Type**: Liste de choix
- **Valeurs possibles**:
    - **Trace** (Recommandé)
    - **Camera**
- **Valeur par défaut**: Trace

## ⚖️ Justification : Comprendre le vent relatif

La météo à vélo est avant tout une histoire de vent : "L'ai-je de face ou de dos ?".

### 1. 🚲 Mode "Trace" (Cap Vélo)

-   En mode **Trace**, le haut de la boussole indique toujours la direction de la route.
-   Si la flèche du vent vient d'en haut, vous avez le vent de face. Si elle vient d'en bas, vent de dos.
-   C'est le mode le plus intuitif pour un cycliste car il simule le ressenti sur le vélo, quelle que soit la position de la caméra qui vous filme.

### 2. 🎥 Mode "Camera" (Cap Vue)

-   En mode **Camera**, le haut de la boussole correspond à l'axe de la caméra.
-   Utile si vous faites des plans artistiques (panoramiques) et que vous voulez savoir où se situe le Nord par rapport à l'image.

---

## ⚠️ Recommandations

-   **Pour analyser un parcours** : Réglez sur **Trace**. C'est le seul moyen fiable de voir rapidement les sections exposées au vent de face.
-   **Pour la réalisation vidéo** : Le mode **Camera** peut aider à orienter les plans en fonction du soleil ou des éléments géographiques.
