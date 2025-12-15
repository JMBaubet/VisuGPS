# ⚙️ Paramètre : Pas du slider de vitesse

Ce document détaille le paramètre `slider_step`, qui définit l'incrément ou le décrément lors de l'ajustement de la vitesse de l'animation via un slider dans l'interface utilisateur.

---

## 🎯 Rôle du Paramètre

Le paramètre `slider_step` détermine la granularité du contrôle de la vitesse. Un pas plus petit permet un ajustement fin de la vitesse, tandis qu'un pas plus grand permet des changements de vitesse plus rapides.

-   **Libellé**: Pas du slider (x)
-   **Type**: Réel (Float)
-   **Valeur par défaut**: 0.05
-   **Minimum**: 0.01
-   **Maximum**: 1.0
-   **Décimales**: 2

## ⚖️ Justification : Pourquoi ajuster le pas du slider ?

L'ajustement du pas du slider est crucial pour offrir à l'utilisateur un contrôle précis et fluide sur la vitesse de l'animation.

### 1. 🖐️ Contrôle Précis

-   Un petit pas permet à l'utilisateur d'affiner la vitesse exactement comme il le souhaite, ce qui est particulièrement utile pour les petites variations ou pour trouver une vitesse optimale.

### 2. ⚡ Réactivité du Contrôle

-   Un pas plus grand peut rendre le slider plus réactif pour les utilisateurs qui préfèrent des ajustements rapides.

---

## ⚠️ Recommandations

-   **Valeur par défaut (0.05)** : Cette valeur offre une bonne balance entre précision et réactivité pour la plupart des utilisateurs.
-   **Adapter aux besoins** :
    -   Pour des ajustements très fins, une valeur comme 0.01 peut être choisie.
    -   Pour des ajustements plus rapides, une valeur comme 0.1 ou 0.2 peut être préférée.
-   **Cohérence avec la plage** : Assurez-vous que `slider_step` est cohérent avec la plage de `min_value` et `max_value`, et que le slider peut couvrir toute la plage de manière efficace.
