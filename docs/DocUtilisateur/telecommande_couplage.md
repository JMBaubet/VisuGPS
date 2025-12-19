# Connexion de la Télécommande

VisuGPS vous permet de piloter votre présentation depuis un mobile. Cette section détaille la procédure de connexion sécurisée.

[< Retour au sommaire](./exploitation.md)

## Activation

1.  Assurez-vous que votre ordinateur et votre appareil mobile sont connectés au **même réseau Wi-Fi**.
2.  Lancez VisuGPS sur votre ordinateur.
3.  Dans la barre d'outils (en haut), repérez l'icône  <img src="https://api.iconify.design/mdi/remote-off.svg?color=%232196F3&width=24" style="vertical-align: middle;" />. Cliquez dessus pour démarrer le couplage.

## Couplage

Le processus se déroule en deux étapes pour garantir la sécurité de la connexion.

### Étape 1 : Connexion initiale
Une fenêtre "Connecter une télécommande" s'ouvre sur votre ordinateur :
1.  Un **QR Code** à scanner avec votre mobile.
2.  Une adresse URL directe (ex: `http://192.168.1.15:9001/remote`) à saisir si le QR code ne fonctionne pas.

### Étape 2 : Confirmation
Une fois la connexion initiée depuis le mobile :
1.  Une nouvelle boîte de dialogue **"Demande de Couplage"** apparaît sur l'ordinateur.
2.  Elle affiche un **Code de couplage** (ex: **<span style="color: blue">I3S2TE3R</span>**).
3.  Vérifiez que ce code correspond exactement à celui affiché sur l'écran de votre mobile.
4.  Cliquez sur **<span style="color: green">ACCEPTER</span>** pour finaliser la connexion.

Une fois connecté, l'icône de télécommande <img src="https://api.iconify.design/mdi/remote.svg?color=%234CAF50&width=24" style="vertical-align: middle;" /> dans la barre d'outils passe au vert.

> [!NOTE]
> Quand une télécommande est connectée, vous pouvez cliquer sur l'icône <img src="https://api.iconify.design/mdi/remote.svg?color=%234CAF50&width=24" style="vertical-align: middle;" />  pour déconnecter la télécommande. Confirmez ce choix en cliquant sur <span style="background-color: #2196F3; color: white; padding: 2px 5px; border-radius: 4px;">Déconnecter</span>. Une demande de couplage sera de nouveau effectuée si nécessaire.

> [!WARNING]
> **Attention** : Si vous cliquez sur **<span style="color: red">REFUSER</span>**, l'appareil sera **définitivement banni**. L'objectif est de vous protéger contre des utilisateurs malveillants qui tenteraient de prendre le contrôle de votre présentation.

> [!NOTE]
> Ce couplage initial est mémorisé : vous n'aurez plus besoin de le refaire pour cet appareil lors des prochaines utilisations.

## Dépannage

Si la connexion échoue :
*   Vérifiez que le **Pare-feu (Firewall)** de votre ordinateur autorise VisuGPS à communiquer sur le port 9001.
*   Confirmez que les deux appareils sont bien sur le même réseau local.
*   Essayez de changer le port dans les [<span style="color: #FFC107">Paramètres</span>](./parametres.md) si le 9001 est déjà utilisé.

---
[Suivant : Utilisation de la télécommande >](./telecommande_utilisation.md)
