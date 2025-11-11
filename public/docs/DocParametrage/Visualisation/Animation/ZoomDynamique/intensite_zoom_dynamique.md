# 📈 Paramètre : Intensité du zoom dynamique (Visualisation)

Ce document détaille le paramètre `intensite_zoom_dynamique`, qui définit l'intensité de l'effet de zoom dynamique appliqué à la caméra pendant l'animation, basé sur la vitesse actuelle.

---

## 🎯 Rôle du Paramètre

Le paramètre `intensite_zoom_dynamique` contrôle la "force" de l'ajustement automatique du niveau de zoom de la caméra en fonction de la vitesse de déplacement. Une valeur plus élevée signifie que le zoom s'adaptera plus fortement, zoomant davantage à basse vitesse et dézoomant plus à haute vitesse.

-   **Libellé**: Intensité du zoom dynamique
-   **Type**: Entier
-   **Valeur par défaut**: 10
-   **Minimum**: 2
-   **Maximum**: 50
-   **Pas (step)**: 1

## ⚖️ Justification : Pourquoi ajuster l'intensité du zoom dynamique ?

L'ajustement de l'intensité du zoom dynamique améliore l'expérience de visualisation en adaptant le champ de vision à la vitesse de l'animation, rendant les mouvements plus fluides et l'observation plus pertinente.

### 1. 👀 Amélioration de l'Expérience Visuelle

-   À basse vitesse, un zoom accru permet d'observer plus de détails du terrain et de suivre plus précisément la trace.
-   À haute vitesse, un dézoom automatique offre une vue plus large, essentielle pour ne pas "perdre" la trace et anticiper les virages ou changements de direction.

### 2. ⚙️ Adaptabilité

-   Permet à l'utilisateur de calibrer l'effet selon ses préférences ou le type de trace (par exemple, une trace en montagne nécessitera peut-être un zoom accru à basse vitesse pour bien voir le relief).

---

## ⚠️ Recommandations

-   **Valeur par défaut (10)** : C'est un bon compromis qui offre un effet de zoom dynamique perceptible sans être trop agressif.
-   **Expérimentation** : Testez différentes valeurs pour trouver celle qui correspond le mieux à votre style de visualisation. Une valeur plus faible rendra l'effet plus subtil, une valeur plus élevée le rendra plus prononcé.
