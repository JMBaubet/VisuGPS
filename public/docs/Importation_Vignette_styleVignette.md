# 🗺️ Paramètre : Style de carte pour la création des vignettes

Ce document détaille le paramètre `styleVignette`, qui définit le style de carte Mapbox utilisé pour générer les vignettes 2D des traces GPX.

---

## 🎯 Rôle du Paramètre

Le paramètre `styleVignette` est une chaîne de caractères (URL de style Mapbox) qui indique l'apparence du fond de carte sur lequel la trace GPX sera dessinée pour créer la vignette.

-   **Libellé**: Style de la vignette
-   **Type**: Chaîne de caractères (String)
-   **Valeur par défaut**: "mapbox://styles/mapbox/streets-v12"

## ⚖️ Justification : Pourquoi choisir un style de carte ?

Le choix du style de carte est crucial pour la lisibilité et l'esthétique des vignettes.

### 1. 👀 Lisibilité de la Trace

-   Certains styles de carte (comme `streets-v12`) sont clairs et mettent en évidence les routes et les points d'intérêt, ce qui peut aider à situer la trace.
-   D'autres styles (comme `satellite-v9`) offrent une vue satellite, utile pour visualiser le terrain réel.

### 2. 🎨 Esthétique

Le style de carte contribue à l'aspect général de la vignette. Vous pouvez choisir un style qui correspond à vos préférences visuelles.

### 3. 🌐 Disponibilité des Styles Mapbox

Mapbox propose une variété de styles prédéfinis (streets, satellite, light, dark, outdoors, etc.). Vous pouvez également créer vos propres styles personnalisés via Mapbox Studio.

---

## ⚠️ Recommandations

-   **Valeur par défaut ("mapbox://styles/mapbox/streets-v12")** : C'est un style polyvalent qui offre une bonne lisibilité de la trace sur un fond de carte clair.
-   **Expérimenter** : N'hésitez pas à essayer d'autres styles Mapbox pour voir celui qui convient le mieux à vos besoins.
-   **Styles personnalisés** : Si vous avez un compte Mapbox, vous pouvez utiliser l'URL de vos propres styles personnalisés.
