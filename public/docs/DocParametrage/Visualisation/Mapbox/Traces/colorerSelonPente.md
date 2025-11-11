# 🌈 Paramètre : Colorer la trace selon la pente

Ce document détaille le paramètre `colorerSelonPente`, qui contrôle si la couleur de la trace GPX affichée sur la carte représente la pente, ou si une couleur unique est utilisée.

---

## 🎯 Rôle du Paramètre

Le paramètre `colorerSelonPente` est un interrupteur (booléen) qui, lorsqu'activé, adapte dynamiquement la couleur de la trace GPX en fonction de la pente du terrain. Si désactivé, la trace adoptera une couleur unique définie par `couleurTrace`.

-   **Libellé**: Colorer la trace selon la pente
-   **Type**: Booléen
-   **Valeur par défaut**: `true`
-   **Surcharge**: `null` (indique que ce paramètre peut être surchargé par les paramètres spécifiques à une trace)

## ⚖️ Justification : Pourquoi colorer la trace selon la pente ?

La coloration de la trace selon la pente est un puissant outil visuel pour analyser rapidement le profil altimétrique du parcours et anticiper les difficultés.

### 1. 📈 Analyse Intuitive du Relief

-   Permet une compréhension instantanée des sections ascendantes (montées), descendantes (descentes) et plates du parcours.
-   Facilite l'identification des passages les plus raides.

### 2. 👀 Visibilité Accrue

-   Sur des terrains à fort dénivelé, la variation de couleur rend la trace plus informative et plus engageante.

### 3. 🎨 Personnalisation

-   Offre une option pour les utilisateurs qui préfèrent une trace d'une couleur uniforme pour des raisons esthétiques ou de clarté.

---

## ⚠️ Recommandations

-   **Activé par défaut** : Il est recommandé de laisser ce paramètre activé pour bénéficier de l'analyse visuelle de la pente.
-   **Désactiver pour une couleur uniforme** : Si vous préférez une trace d'une seule couleur (par exemple, pour des impressions ou pour correspondre à un thème visuel), désactivez ce paramètre et configurez `couleurTrace`.
