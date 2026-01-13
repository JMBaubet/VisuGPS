# 🎨 Paramètre : Couleur Trace Maîtresse

Ce document détaille le paramètre `originalColor`, utilisé dans la configuration des variantes.

---

## 🎯 Rôle du Paramètre

Le paramètre `originalColor` définit la couleur utilisée pour afficher la trace maîtresse (trace originale) lors de l'édition d'une variante.

-   **Libellé**: Couleur trace maîtresse
-   **Type**: Couleur (Material Design)
-   **Valeur par défaut**: `grey-lighten-1` (Gris clair)

## ⚖️ Justification : Pourquoi ajuster cette couleur ?

Cette distinction visuelle permet de différencier le parcours initial des modifications que vous apportez.

### 1. 👀 Contraste et Visibilité

-   **Clarté** : Une couleur neutre pour la trace originale permet de mieux faire ressortir le tracé de la variante (en couleur vive).
-   **Contexte** : Selon votre fond de carte (Satellite, Sombre, Clair), vous pourriez avoir besoin d'ajuster cette couleur pour qu'elle reste visible sans être dominante.

---

## ⚠️ Recommandations

-   **Valeur par défaut** : Le gris clair est idéal pour ne pas surcharger la vue tout en restant lisible.
-   **Cohérence** : Évitez d'utiliser la même couleur que celle de l'aperçu (`previewColor`) pour ne pas créer de confusion.
