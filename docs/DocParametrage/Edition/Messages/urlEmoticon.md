# 🔗 Paramètre : URL Emoticon

Ce document détaille le paramètre `urlEmoticon`, qui définit l'adresse web de la page d'aide affichée dans la fenêtre d'édition des messages.

---

## 🎯 Rôle du Paramètre

Le paramètre `urlEmoticon` permet de configurer le lien vers lequel l'utilisateur est redirigé lorsqu'il clique sur "Aide Emojis". Cela permet de fournir un accès rapide à une liste de codes emojis compatibles avec l'application.

-   **Libellé**: URL Emoticon
-   **Type**: Chaîne de caractères (URL)
-   **Valeur par défaut**: `https://github.com/ikatyang/emoji-cheat-sheet`

## ⚖️ Justification : Pourquoi une URL paramétrable ?

La flexibilité de ce paramètre permet d'adapter l'aide aux besoins de l'utilisateur ou aux évolutions des ressources en ligne.

### 1. 🛠️ Adaptabilité
-   Si la source d'aide préférée change ou si le lien par défaut devient obsolète, il peut être mis à jour sans modification du code.

### 2. 🌍 Choix de la Ressource
-   L'utilisateur peut choisir une cheat sheet qui lui semble plus complète ou plus claire.

---

## ⚠️ Recommandations

-   **Utiliser une URL valide** : Assurez-vous que l'URL commence par `http://` ou `https://`.
-   **Compatibilité** : Il est recommandé de pointer vers une page qui liste les codes emojis "standard" (ex: `:smile:`) car ce sont ceux généralement reconnus par les parseurs markdown.
-   **Vérification** : Après avoir modifié cette URL, testez le lien dans la fenêtre d'édition d'un message pour vous assurer qu'il s'ouvre correctement dans votre navigateur.
