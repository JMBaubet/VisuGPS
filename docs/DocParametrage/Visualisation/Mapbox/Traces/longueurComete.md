# 📏 Paramètre : Longueur de la comète (Visualisation)

Ce document détaille le paramètre `longueurComete`, qui définit la longueur en mètres de la "comète" (la trace lumineuse) qui suit l'avancement de la caméra le long de la trace GPX en mode visualisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `longueurComete` contrôle la taille de la comète, c'est-à-dire la portion de trace "éclairée" derrière le point d'avancement. Une longueur appropriée permet de visualiser la progression de manière fluide et esthétique.

-   **Libellé**: Longueur de la comète (m)
-   **Type**: Entier
-   **Valeur par défaut**: 50 m
-   **Minimum**: 10 m
-   **Maximum**: 5000 m

## ⚖️ Justification : Pourquoi ajuster la longueur de la comète ?

L'ajustement de la longueur de la comète est important pour l'effet visuel de l'animation et la perception de la vitesse.

### 1. 🚀 Effet de Vitesse et Fluidité

-   Une comète plus longue peut donner une impression de vitesse plus élevée ou de fluidité accrue, en particulier sur des traces rapides.
-   Une comète plus courte peut être préférée pour des animations plus lentes ou pour des traces très détaillées.

### 2. 👀 Visibilité et Contexte

-   Une comète trop courte pourrait être difficile à percevoir, tandis qu'une comète trop longue pourrait masquer une partie trop importante de la trace ou du fond de carte.

---

## ⚠️ Recommandations

-   **Valeur par défaut (50 m)** : C'est un bon point de départ qui offre un équilibre entre visibilité et discrétion.
-   **Adapter à la vitesse de l'animation** : Pour des animations très rapides, une comète plus longue peut être plus agréable visuellement. Pour des animations lentes, une comète plus courte peut suffire.
-   **Adapter à la longueur totale de la trace** : Sur des traces très courtes, une comète proportionnellement plus courte peut être plus appropriée.
