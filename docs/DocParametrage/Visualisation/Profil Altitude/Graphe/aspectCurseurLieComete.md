# 🔗 Paramètre : Aspect curseur lié à la comète (Profil Altimétrique)

Ce document détaille le paramètre `aspectCurseurLieComete`, qui détermine si le curseur de progression du profil altimétrique doit hériter son apparence de la comète affichée sur la carte 3D.

---

## 🎯 Rôle du Paramètre

Le paramètre `Aspect curseur lié à la comète` permet de synchroniser visuellement l'indicateur d'avancement du graphique d'altitude avec la comète qui parcourt la trace en 3D.

-   **Libellé**: Aspect curseur lié à la comète
-   **Type**: Booléen (Vrai/Faux)
-   **Valeur par défaut**: Vrai (Activé)

## ⚖️ Justification : Pourquoi lier l'aspect du curseur ?

Cette synchronisation vise à renforcer la cohérence visuelle entre les deux modes de visualisation (Carte 3D et Profil 2D).

### 1. 👁️ Cohérence Visuelle

-   En utilisant la même **couleur** et la même **opacité**, l'utilisateur identifie immédiatement le lien entre la comète sur la carte et le curseur sur le graphique.
-   L'utilisateur n'a pas à chercher deux indicateurs visuellement différents pour la même information (la position actuelle).

### 2. 📏 Représentation de l'Échelle

-   Contrairement à une simple ligne verticale, le curseur synchronisé adapte sa **largeur** pour représenter la longueur réelle de la comète (ex: 50m, 100m) à l'échelle du graphique.
-   Cela permet de visualiser quelle portion du relief est actuellement couverte par la "queue" de la comète.

### 3. ✨ Esthétique Personnalisée

-   Les choix esthétiques faits pour la comète (couleurs vives, opacité subtile) sont automatiquement répercutés sur l'interface 2D, rendant l'expérience plus uniforme et agréable.

---

## ⚠️ Recommandations

-   **Activé (Recommandé)** : Laissez ce paramètre activé pour une expérience utilisateur optimale et une meilleure compréhension des échelles.
-   **Désactivé** : Désactivez ce paramètre si vous préférez un curseur fin et précis (type ligne simple) pour la lecture du graphique, indépendamment de la taille de la comète.

