# 🌫️ Paramètre : Opacité de la zone d'avancement (Graphe)

Ce document détaille le paramètre `opaciteAvancementZone`, qui définit l'opacité de la zone rectangulaire indiquant l'avancement de la caméra sur le graphe d'édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `opaciteAvancementZone` contrôle la transparence de la zone qui met en évidence la portion de la trace actuellement visible ou en cours d'édition sur le graphe.

-   **Libellé**: Opacité de la zone d'avancement
-   **Type**: Réel (Float)
-   **Valeur par défaut**: 0.1
-   **Minimum**: 0.0 (complètement transparent)
-   **Maximum**: 1.0 (complètement opaque)
-   **Décimales**: 2

## ⚖️ Justification : Pourquoi ajuster l'opacité de la zone d'avancement ?

L'ajustement de l'opacité permet de trouver un équilibre entre la visibilité de la zone d'avancement et la lisibilité des courbes et du fond du graphe.

### 1. 👀 Lisibilité du Graphe

-   Une opacité trop élevée pourrait masquer les courbes ou les repères sous la zone d'avancement.
-   Une opacité trop faible pourrait rendre la zone d'avancement difficile à percevoir.

### 2. 🎨 Esthétique

-   Permet de personnaliser l'apparence du graphe selon les préférences de l'utilisateur.

---

## ⚠️ Recommandations

-   **Valeur par défaut (0.1)** : C'est un bon compromis qui rend la zone visible sans masquer les éléments importants du graphe.
-   **Ajuster selon les préférences** : Vous pouvez augmenter l'opacité si vous souhaitez que la zone soit plus prononcée, ou la diminuer si vous préférez une indication plus subtile.
