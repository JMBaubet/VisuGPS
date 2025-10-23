# Documentation Technique : Fonctionnement de la Télécommande VisuGPS

Ce document décrit l'architecture et le flux de communication du système de télécommande pour l'application VisuGPS. Il couvre le client web (la télécommande elle-même), le serveur WebSocket intégré au backend Rust de l'application desktop, et la manière dont les commandes sont transmises au frontend Vue.js.

## Table des Matières

1.  [Introduction](#1-introduction)
2.  [L'Application de Télécommande (Client Web)](#2-lapplication-de-télécommande-client-web)
    *   [2.1. Structure des Fichiers](#21-structure-des-fichiers)
    *   [2.2. Initialisation et Connexion](#22-initialisation-et-connexion)
    *   [2.3. Processus de Couplage](#23-processus-de-couplage)
    *   [2.4. Envoi de Commandes](#24-envoi-de-commandes)
    *   [2.5. Réception des Mises à Jour d'État](#25-réception-des-mises-à-jour-détat)
3.  [Le Serveur WebSocket (Backend Rust de l'Application Desktop)](#3-le-serveur-websocket-backend-rust-de-lapplication-desktop)
    *   [3.1. Rôle et Initialisation](#31-rôle-et-initialisation)
    *   [3.2. Gestion des Connexions et des Clients](#32-gestion-des-connexions-et-des-clients)
    *   [3.3. Traitement des Requêtes de Couplage](#33-traitement-des-requêtes-de-couplage)
    *   [3.4. Traitement des Commandes Reçues](#34-traitement-des-commandes-reçues)
    *   [3.5. Envoi des Mises à Jour d'État](#35-envoi-des-mises-à-jour-détat)
4.  [Communication entre Backend Rust et Frontend Vue.js](#4-communication-entre-backend-rust-et-frontend-vuejs)
    *   [4.1. Transmission des Commandes au Frontend](#41-transmission-des-commandes-au-frontend)
    *   [4.2. Réaction du Frontend aux Commandes](#42-réaction-du-frontend-aux-commandes)
    *   [4.3. Mise à Jour de l'État de l'Application (Frontend -> Backend -> Télécommande)](#43-mise-à-jour-de-létat-de-lapplication-frontend---backend---télécommande)
5.  [Flux de Communication Global](#5-flux-de-communication-global)
6.  [Considérations et Améliorations Futures](#6-considérations-et-améliorations-futures)

---

## 1. Introduction

Le système de télécommande VisuGPS permet de contrôler à distance certaines fonctionnalités de l'application desktop principale, notamment lors de la visualisation 3D d'une trace GPX. Il est composé d'une application web légère (le client de télécommande) et d'un serveur WebSocket intégré au backend Rust de l'application desktop. La communication entre le backend Rust et le frontend Vue.js de l'application desktop assure la synchronisation et l'exécution des commandes.

## 2. L'Application de Télécommande (Client Web)

Située dans `src/remote_client/`, cette application est une interface web simple (HTML, CSS, JavaScript) conçue pour être exécutée sur un appareil mobile (smartphone, tablette) ou un autre navigateur web.

### 2.1. Structure des Fichiers

*   **`index.html`**: La structure HTML de base de la télécommande. Elle contient :
    *   Un titre (`<h1>VisuGPS Télécommande</h1>`).
    *   Un `div` pour afficher le statut de la connexion (`#status`).
    *   Un `div` pour afficher le code de couplage (`#pairing-code`).
    *   Un `div` (`#controls`) qui contient les boutons de commande (Play/Pause, Commandes, Profil Altitude, Communes, Distance, Accueil), initialement masqué.
    *   Des liens vers `style.css` et `main.js`.
*   **`style.css`**: Contient les règles CSS pour styliser l'interface utilisateur de la télécommande.
*   **`main.js`**: Le script JavaScript principal qui gère toute la logique de connexion, de couplage et d'envoi/réception des messages WebSocket.

### 2.2. Initialisation et Connexion

Au chargement de `main.js`, plusieurs étapes d'initialisation sont effectuées :

*   **`WS_SERVER_IP` et `WS_SERVER_PORT`**: L'adresse IP et le port du serveur WebSocket de l'application desktop sont définis. Actuellement, l'IP (`192.168.1.52`) est codée en dur, ce qui est un point d'amélioration futur.
*   **`clientId`**: Un identifiant unique pour le client de la télécommande est généré (via `generateUUID` ou `crypto.randomUUID`) et stocké dans le `localStorage` du navigateur. Cela permet à la télécommande de conserver son identité entre les sessions.
*   **`pairingCode`**: Un code alphanumérique aléatoire de 8 caractères est généré (`generateRandomCode`). Ce code est affiché à l'utilisateur et sera utilisé pour le processus de couplage avec l'application desktop.
*   **Connexion WebSocket**: Une tentative de connexion est établie avec le serveur WebSocket de l'application desktop (`new WebSocket(WS_URL)`).
    *   **`ws.onopen`**: Une fois la connexion établie, le `clientId` et le `pairingCode` sont envoyés au serveur via un message de type `"pairing_request"`.
    *   **`ws.onclose`**: En cas de déconnexion, une tentative de reconnexion est effectuée après un délai de 3 secondes.
    *   **`ws.onerror`**: Gère les erreurs de connexion.

### 2.3. Processus de Couplage

Le couplage est une étape cruciale pour associer une télécommande spécifique à l'application desktop :

*   **Envoi de `pairing_request`**: Dès que la connexion WebSocket est ouverte, la télécommande envoie un message JSON au serveur :
    ```json
    {
        "type": "pairing_request",
        "clientId": "votre_client_id_unique",
        "pairingCode": "CODEALEA"
    }
    ```
*   **Réception de `pairing_response`**: Le serveur répond avec un message de type `"pairing_response"` :
    ```json
    {
        "type": "pairing_response",
        "status": "accepted" | "refused" | "already_paired",
        "reason": "message_si_refusé"
    }
    ```
    *   Si le statut est `"accepted"` ou `"already_paired"`, le `div` des contrôles (`#controls`) est affiché, et le `div` du code de couplage est masqué.
    *   Si le statut est `"refused"`, un message d'erreur est affiché.

### 2.4. Envoi de Commandes

Une fois couplée, la télécommande peut envoyer des commandes à l'application desktop. Chaque bouton de commande (`#play-pause`, `#toggle-commands`, etc.) est associé à un écouteur d'événements `click`. Lorsqu'un bouton est cliqué, un message JSON de type `"command"` est envoyé au serveur :

```json
{
    "type": "command",
    "clientId": "votre_client_id_unique",
    "command": "toggle_play" // Exemple de commande
}
```
Les commandes actuellement définies sont :
*   `toggle_play`
*   `toggle_commands_widget`
*   `toggle_altitude_profile`
*   `toggle_communes_display`
*   `toggle_distance_display`
*   `toggle_home`

### 2.5. Réception des Mises à Jour d'État

La télécommande est également capable de recevoir des mises à jour d'état de l'application desktop via des messages de type `"app_state_update"` :

```json
{
    "type": "app_state_update",
    "appState": "Visualisation" // Exemple d'état
}
```
Le `main.js` utilise cet état pour adapter l'interface de la télécommande. Par exemple, les contrôles ne sont affichés que si `message.appState` est `"Visualisation"`.

## 3. Le Serveur WebSocket (Backend Rust de l'Application Desktop)

Le serveur WebSocket est une composante du backend Rust de l'application VisuGPS (probablement implémenté dans des modules comme `remote_control.rs` ou `remote_clients.rs`). Il est responsable de la gestion des connexions WebSocket, du couplage des télécommandes, du traitement des commandes reçues et de l'envoi des mises à jour d'état.

### 3.1. Rôle et Initialisation

*   Le serveur écoute les connexions entrantes sur le port `9001` (défini dans `main.js` de la télécommande).
*   Il doit être initialisé au démarrage de l'application desktop.

### 3.2. Gestion des Connexions et des Clients

*   Le serveur maintient une liste des clients WebSocket connectés et potentiellement couplés.
*   Chaque connexion est associée à un `clientId` unique.

### 3.3. Traitement des Requêtes de Couplage

Lorsqu'un message `"pairing_request"` est reçu :

1.  Le serveur extrait le `clientId` et le `pairingCode`.
2.  Il doit présenter ce `pairingCode` à l'utilisateur de l'application desktop (probablement via une interface graphique dans le frontend Vue.js) pour validation.
3.  L'utilisateur de l'application desktop doit pouvoir accepter ou refuser le couplage.
4.  En fonction de la décision de l'utilisateur, le serveur envoie une réponse `"pairing_response"` à la télécommande avec le statut approprié (`"accepted"`, `"refused"`, `"already_paired"`).
5.  Si le couplage est accepté, le `clientId` est enregistré comme un client couplé.

### 3.4. Traitement des Commandes Reçues

Lorsqu'un message `"command"` est reçu d'un client couplé :

1.  Le serveur vérifie que le `clientId` est bien celui d'une télécommande couplée et autorisée.
2.  Il extrait la `command` (par exemple, `"toggle_play"`).
3.  Il doit ensuite traduire cette commande en une action spécifique au sein de l'application desktop. Cela implique généralement d'émettre un événement Tauri vers le frontend Vue.js.

### 3.5. Envoi des Mises à Jour d'État

Le serveur Rust doit être capable d'envoyer des messages `"app_state_update"` à toutes les télécommandes couplées lorsque l'état pertinent de l'application desktop change (par exemple, lorsque l'utilisateur navigue vers la vue de visualisation 3D).

## 4. Communication entre Backend Rust et Frontend Vue.js

La communication entre le backend Rust (où réside le serveur WebSocket) et le frontend Vue.js (l'interface utilisateur de l'application desktop) est essentielle pour que la télécommande puisse interagir avec l'application. Tauri fournit des mécanismes pour cela.

### 4.1. Transmission des Commandes au Frontend

Lorsque le backend Rust reçoit une commande de la télécommande (par exemple, `"toggle_play"`), il ne l'exécute pas directement. Au lieu de cela, il émet un événement Tauri que le frontend Vue.js peut écouter.

*   **Côté Rust (Backend)**:
    ```rust
    // Exemple dans le handler de message WebSocket
    if message.type == "command" && message.command == "toggle_play" {
        app_handle.emit_all("remote_command", "toggle_play").unwrap();
    }
    ```
    (`app_handle` est une référence à l'application Tauri, et `"remote_command"` est le nom de l'événement.)

### 4.2. Réaction du Frontend aux Commandes

Le frontend Vue.js écoute les événements émis par le backend Rust.

*   **Côté Vue.js (Frontend)**:
    ```javascript
    import { listen } from '@tauri-apps/api/event';

    // Dans un composant Vue ou un hook (par exemple, onMounted)
    listen('remote_command', (event) => {
      console.log('Commande reçue du backend :', event.payload);
      if (event.payload === 'toggle_play') {
        // Logique pour basculer la lecture/pause dans le frontend
        // Par exemple, appeler une fonction du lecteur vidéo/animation
      }
      // Gérer d'autres commandes
    });
    ```
    Le `event.payload` contiendra la commande envoyée par le backend.

### 4.3. Mise à Jour de l'État de l'Application (Frontend -> Backend -> Télécommande)

Pour que la télécommande puisse adapter son interface (par exemple, n'afficher les contrôles que dans la vue de visualisation), le frontend doit informer le backend de ses changements d'état, qui à son tour les transmettra aux télécommandes.

*   **Côté Vue.js (Frontend)**: Lorsque l'application desktop change de vue (par exemple, passe à `VisualizeView`), le frontend peut appeler une commande Tauri pour informer le backend :
    ```javascript
    import { invoke } from '@tauri-apps/api/tauri';

    // Lorsque la vue change
    invoke('update_app_state', { newState: 'Visualisation' });
    ```
*   **Côté Rust (Backend)**: Le backend reçoit cette commande, met à jour son propre état interne, puis envoie un message `"app_state_update"` à toutes les télécommandes couplées via WebSocket.
    ```rust
    #[tauri::command]
    fn update_app_state(app_handle: tauri::AppHandle, new_state: String) {
        // Mettre à jour l'état interne du backend
        // ...
        // Envoyer la mise à jour à toutes les télécommandes couplées
        // (Logique pour itérer sur les clients WebSocket et envoyer le message)
        let message = json!({ "type": "app_state_update", "appState": new_state });
        // ws_server.send_to_all_coupled_clients(message.to_string());
    }
    ```

## 5. Flux de Communication Global

Voici un résumé du flux de communication :

1.  **Initialisation**:
    *   Télécommande démarre, génère `clientId` et `pairingCode`.
    *   Télécommande tente de se connecter au serveur WebSocket Rust.
2.  **Couplage**:
    *   Télécommande envoie `pairing_request` au serveur Rust.
    *   Serveur Rust affiche `pairingCode` à l'utilisateur desktop.
    *   Utilisateur desktop accepte/refuse le couplage.
    *   Serveur Rust envoie `pairing_response` à la télécommande.
    *   Si accepté, télécommande affiche les contrôles.
3.  **Changement d'État de l'Application Desktop**:
    *   Frontend Vue.js de l'application desktop change de vue (ex: `VisualizeView`).
    *   Frontend appelle une commande Tauri (`update_app_state`) vers le backend Rust.
    *   Backend Rust reçoit la commande et envoie `app_state_update` à toutes les télécommandes couplées.
    *   Télécommandes adaptent leur UI en fonction de `appState`.
4.  **Envoi de Commande**:
    *   Utilisateur clique sur un bouton de la télécommande.
    *   Télécommande envoie `command` au serveur Rust.
    *   Serveur Rust reçoit `command` et émet un événement Tauri (`remote_command`) vers le frontend Vue.js.
    *   Frontend Vue.js écoute `remote_command` et exécute l'action correspondante.

## 6. Considérations et Améliorations Futures

*   **Adresse IP Codée en Dur**: L'adresse IP du serveur WebSocket est actuellement codée en dur dans `main.js`. Il serait préférable de la rendre configurable (par exemple, via une découverte de service, une configuration utilisateur, ou en la transmettant dynamiquement depuis l'application desktop).
*   **Sécurité du Couplage**: Le couplage actuel repose sur un code aléatoire. Pour des environnements plus sensibles, une authentification plus robuste pourrait être envisagée.
*   **Gestion des Clients**: Le backend Rust doit gérer efficacement l'ajout, la suppression et la persistance des clients couplés.
*   **Robustesse des Commandes**: Assurer que toutes les commandes sont correctement gérées et que des messages d'erreur appropriés sont renvoyés en cas de problème.
*   **Feedback Visuel**: Améliorer le feedback visuel sur la télécommande pour indiquer si une commande a été reçue et exécutée avec succès par l'application desktop.
