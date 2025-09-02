# Guide de contribution au projet VisuGPS

Merci de l'intérêt que vous portez au projet VisuGPS ! Nous sommes ravis que vous souhaitiez contribuer. Ce guide est conçu pour vous aider à démarrer.

---

## 💡 Comment contribuer ?

Il existe de nombreuses façons de nous aider. Voici quelques pistes :

* **Signaler un bug** : Si vous rencontrez un problème, veuillez ouvrir une "Issue" sur GitHub.
* **Proposer une fonctionnalité** : Vous avez une idée d'amélioration ? Proposez-la dans une "Issue" avant de commencer à coder.
* **Contribuer au code** : Corrigez un bug ou implémentez une nouvelle fonctionnalité.
* **Améliorer la documentation** : Si vous trouvez des erreurs ou des imprécisions dans les fichiers `README.md` ou `CONTRIBUTING.md`, n'hésitez pas à les corriger.

---

## 💻 Contribution au code

### 1. Configuration de l'environnement

Pour que l'application puisse afficher les cartes, vous devez obtenir votre propre **token d'accès Mapbox**. C'est un processus gratuit et simple.

1.  Rendez-vous sur [mapbox.com](https://www.mapbox.com/) et créez un compte gratuit.
2.  Sur votre page de profil, copiez votre **token d'accès public**.
3.  À la racine de ce projet, créez un fichier nommé **`.env`**.
4.  Ajoutez-y la ligne suivante, en remplaçant le texte par votre propre token :
    
    `VUE_APP_MAPBOX_TOKEN=votre_token_mapbox`
    
    **Attention** : Ce fichier `.env` ne doit jamais être partagé ou publié. Il est géré par le fichier `.gitignore` afin de ne pas être committé dans le dépôt.

### 2. Démarrer

Une fois votre `.env` configuré, vous pouvez suivre les étapes de développement habituelles :

1.  **Forkez** le dépôt.
2.  **Clonez** votre fork sur votre machine locale.
3.  **Créez une branche** pour vos modifications : `git checkout -b ma-super-branche`.
4.  **Codez vos modifications**.
5.  **Testez vos changements** : Assurez-vous que le projet fonctionne toujours comme prévu.
6.  **Faites un "commit"** avec un message clair : `git commit -m "feat: ajout de la fonctionnalité XYZ"`.
7.  **"Pushez"** votre branche sur votre fork : `git push origin ma-super-branche`.
8.  **Ouvrez une "Pull Request"** sur le dépôt principal.

---

## 🐞 Signaler un bug

Pour nous aider à résoudre les problèmes le plus rapidement possible, veuillez suivre ces étapes lorsque vous ouvrez une "Issue" :

1.  **Recherchez d'abord** : Vérifiez si le bug n'a pas déjà été signalé.
2.  **Décrivez clairement le problème** : Expliquez ce qui ne fonctionne pas et le comportement que vous attendiez.
3.  **Fournissez les étapes de reproduction** : Décrivez comment reproduire le bug étape par étape. Si possible, incluez des captures d'écran ou des extraits de code.
4.  **Mentionnez votre environnement** : Indiquez votre système d'exploitation, les versions des logiciels utilisés (Node.js, Tauri, etc.).

---

## 📜 Code de conduite

Afin de maintenir un environnement ouvert et accueillant, nous vous demandons de respecter notre [Code de conduite](CODE_OF_CONDUCT.md).

---

## 🙏 Reconnaissance

Toutes les contributions sont les bienvenues et très appréciées ! Les contributeurs fréquents pourront être mentionnés dans le `README.md` du projet.