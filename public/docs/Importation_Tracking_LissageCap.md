# 📐 Paramètre : Nombre de segments pour le lissage du cap

Ce document détaille le paramètre `LissageCap`, qui définit le nombre de segments utilisés pour calculer la moyenne du cap de la caméra, afin de lisser les changements brusques de direction.

---

## 🎯 Rôle du Paramètre

Le paramètre `LissageCap` détermine la "douceur" avec laquelle la caméra suit les changements de direction de la trace. Un nombre plus élevé de segments pris en compte pour le calcul du cap moyen résulte en un mouvement de caméra plus fluide.

-   **Libellé**: Lissage du cap
-   **Type**: Entier
-   **Valeur par défaut**: 15
-   **Minimum**: 1
-   **Maximum**: 50
-   **Critique**: `true` (indique que ce paramètre peut avoir un impact significatif sur le comportement de la caméra).

## ⚖️ Justification : Pourquoi ajuster le lissage du cap ?

Le lissage du cap est essentiel pour une expérience de visualisation agréable, évitant les mouvements de caméra saccadés.

### 1. 🎥 Fluidité de la Caméra

-   **Valeur élevée** (`> 15`) : Produit un mouvement de caméra très doux et progressif, idéal pour des vues aériennes ou des traces avec de nombreux virages serrés qui pourraient autrement rendre la caméra instable.
-   **Valeur faible** (`< 15`) : La caméra réagit plus rapidement aux changements de direction, ce qui peut être souhaitable pour un suivi plus "nerveux" ou pour des traces très droites.

### 2. 📊 Précision du Cap

-   Un lissage trop important peut faire en sorte que la caméra "coupe les virages" visuellement, car elle anticipe moins les changements de direction.
-   Un lissage faible peut rendre le cap de la caméra plus fidèle à la direction instantanée de la trace, mais potentiellement plus saccadé.

---

## ⚠️ Recommandations

-   **Valeur par défaut (15)** : C'est un bon compromis qui offre un équilibre entre fluidité et réactivité du cap.
-   **Adapter à la trace et aux préférences** :
    -   Pour des traces avec beaucoup de virages serrés ou si vous préférez un mouvement de caméra très doux, augmentez cette valeur.
    -   Si vous souhaitez que la caméra suive la trace de manière plus directe et réactive, réduisez cette valeur.
