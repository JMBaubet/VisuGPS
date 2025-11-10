# ⌨️ Paramètre : Touche de recul pour la caméra

Ce document détaille le paramètre `toucheAvancementArriere`, qui définit la touche du clavier utilisée pour faire reculer la caméra le long de la trace en mode édition.

---

## 🎯 Rôle du Paramètre

Le paramètre `toucheAvancementArriere` permet de personnaliser la touche clavier associée à l'action de reculer la caméra sur la trace.

-   **Libellé**: Touche Avancement Arrière
-   **Type**: Chaîne de caractères (String)
-   **Valeur par défaut**: "ArrowLeft"

## ⚖️ Justification : Pourquoi personnaliser les touches de commande ?

La personnalisation des touches de commande améliore l'ergonomie et l'accessibilité de l'application, permettant à l'utilisateur d'adapter l'interface à ses préférences.

### 1. 🖐️ Ergonomie Personnalisée

-   Les utilisateurs peuvent avoir des préférences différentes pour les touches de commande (par exemple, des droitiers/gauchers, ou des habitudes issues d'autres logiciels).

### 2. ♿ Accessibilité

-   Permet d'adapter les commandes pour les utilisateurs ayant des besoins spécifiques ou utilisant des claviers non standard.

---

## ⚠️ Recommandations

-   **Valeur par défaut ("ArrowLeft")** : C'est un choix standard et intuitif pour le recul.
-   **Utiliser des noms de touches valides** : Assurez-vous d'utiliser des noms de touches reconnus (par exemple, "ArrowRight", "ArrowDown", "z", " ", etc.). Les noms sont ceux renvoyés par l'événement `KeyboardEvent.key`.
-   **Éviter les conflits** : Choisissez une touche qui n'entre pas en conflit avec d'autres raccourcis clavier importants de l'application ou du système d'exploitation.
