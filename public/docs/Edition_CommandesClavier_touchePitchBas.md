# ⌨️ Paramètre : Touche pour diminuer le pitch de la caméra

Ce document détaille le paramètre `touchePitchBas`, qui définit la touche du clavier utilisée pour diminuer l'angle d'inclinaison (pitch) de la caméra en mode édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `touchePitchBas` permet de personnaliser la touche clavier associée à l'action de diminuer le pitch de la caméra.

-   **Libellé**: Touche Pitch Bas
-   **Type**: Chaîne de caractères (String)
-   **Valeur par défaut**: "ArrowDown"

## ⚖️ Justification : Pourquoi personnaliser les touches de commande ?

La personnalisation des touches de commande améliore l'ergonomie et l'accessibilité de l'application, permettant à l'utilisateur d'adapter l'interface à ses préférences.

### 1. 🖐️ Ergonomie Personnalisée

-   Les utilisateurs peuvent avoir des préférences différentes pour les touches de commande (par exemple, des droitiers/gauchers, ou des habitudes issues d'autres logiciels).

### 2. ♿ Accessibilité

-   Permet d'adapter les commandes pour les utilisateurs ayant des besoins spécifiques ou utilisant des claviers non standard.

---

## ⚠️ Recommandations

-   **Valeur par défaut ("ArrowDown")** : C'est un choix standard et intuitif pour diminuer le pitch.
-   **Utiliser des noms de touches valides** : Assurez-vous d'utiliser des noms de touches reconnus (par exemple, "ArrowLeft", "ArrowUp", "s", " ", etc.). Les noms sont ceux renvoyés par l'événement `KeyboardEvent.key`.
-   **Éviter les conflits** : Choisissez une touche qui n'entre pas en conflit avec d'autres raccourcis clavier importants de l'application ou du système d'exploitation.
