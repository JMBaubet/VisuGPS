# 💬 Paramètre : Message départ

Ce document détaille le paramètre `Message départ`, qui définit quel message de la bibliothèque utiliser pour marquer le début du circuit.

---

## 🎯 Rôle du Paramètre

Ce paramètre spécifie l'identifiant du message qui sera inséré au point de départ si l'option `Afficher le départ` est activée.

-   **Type**: Message (Sélection dans la bibliothèque)
-   **Valeur par défaut**: `_Départ_green`

---

## 💡 Utilisation

Plutôt que de saisir un texte à chaque fois, ce paramètre fait référence à un objet "Message" complet stocké dans votre bibliothèque locale, incluant :
- Le texte (ex: "Départ", "Start", "Go")
- Le style (Couleur de fond, police)

Vous pouvez sélectionner n'importe quel message de votre bibliothèque en cliquant sur la valeur dans les paramètres. Cela permet de standardiser vos messages de départ (par exemple, avoir un style "Course" et un style "Randonnée").
