# 🌈 Paramètre : Colorer la trace selon la pente (Édition)

Ce document détaille le paramètre `colorerSelonPente` pour le mode **édition**. Il contrôle si la couleur de la trace GPX représente la pente ou si une couleur unique est utilisée.

---

## 🎯 Rôle du Paramètre

Ce paramètre est un interrupteur (booléen) qui, lorsqu'activé, adapte dynamiquement la couleur de la trace en fonction de la pente du terrain directement dans l'éditeur.

-   **Libellé**: Colorer la trace selon la pente
-   **Type**: Booléen
-   **Valeur par défaut**: `true`

## ⚖️ Justification

Activer cette option en mode édition permet d'avoir un retour visuel immédiat sur le profil du parcours sans avoir à lancer la visualisation.

### 1. 📈 Analyse en direct

-   Permet de corréler directement les ajustements de caméra (zoom, pitch) que vous effectuez avec la pente du terrain.
-   Facilite le placement d'événements (messages, pauses) en fonction du relief.

### 2. 👀 Cohérence avec la Visualisation

-   En l'activant, l'aperçu en mode édition est fidèle au rendu final de la visualisation (si l'option y est aussi activée), ce qui évite les allers-retours entre les deux vues.

---

## ⚠️ Recommandations

-   **Activé par défaut** : Il est recommandé de laisser ce paramètre activé pour bénéficier d'un contexte visuel riche lors de l'édition.
-   **Désactiver pour la clarté** : Si vous travaillez sur un fond de carte très chargé ou si vous préférez une meilleure distinction entre la trace principale et la ligne de progression, vous pouvez désactiver ce paramètre et configurer une `Couleur de la trace` très contrastée.