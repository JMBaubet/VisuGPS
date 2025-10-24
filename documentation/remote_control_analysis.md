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

Située dans `src/remote_client/`, cette application est une interface web monopage dynamique (HTML, CSS, JavaScript) conçue pour être exécutée sur un appareil mobile ou un autre navigateur. Elle est servie directement par le serveur HTTP intégré au backend Rust.

### 2.1. Structure des Fichiers

*   **`index.html`**: La structure HTML de base. Elle contient :
    *   Des conteneurs pour le statut, le code de couplage.
    *   Plusieurs `div` agissant comme des "pages" (`#page-visualize`, `#page-edit`, etc.), dont la visibilité est contrôlée par JavaScript en fonction de l'état de l'application principale.
*   **`style.css`**: Les styles pour toutes les vues de la télécommande.
*   **`main.js`**: Le script principal qui gère la connexion, le couplage, l'envoi/réception des messages WebSocket et la logique d'affichage des différentes pages.

### 2.2. Initialisation et Connexion

Au chargement de `main.js`, plusieurs étapes sont effectuées :

*   **Configuration Réseau Dynamique**: Les constantes `WS_SERVER_IP` et `WS_SERVER_PORT` ne sont plus codées en dur. Le backend Rust génère dynamiquement le fichier `main.js` servi au client en y injectant l'adresse IP locale du serveur et le port configuré. 
*   **`clientId`**: Un identifiant unique est généré (via `crypto.randomUUID` ou un fallback) et stocké dans le `localStorage` pour permettre à la télécommande de conserver son identité.
*   **`pairingCode`**: Un code alphanumérique aléatoire est généré pour le processus de couplage initial.
*   **Connexion WebSocket**: Une connexion est établie avec le serveur. Le script gère les tentatives de reconnexion automatiques en cas d'échec.
    *   **`ws.onopen`**: Une fois connecté, un message `pairing_request` est envoyé au serveur.

### 2.3. Processus de Couplage

Le couplage associe une télécommande à l'application desktop :

*   **Envoi de `pairing_request`**: La télécommande s'identifie auprès du serveur.
    ```json
    {
        "type": "pairing_request",
        "clientId": "votre_client_id_unique",
        "pairingCode": "CODEALEA"
    }
    ```
*   **Réception de `pairing_response`**: Le serveur répond en indiquant le statut du couplage et l'état actuel de l'application (`appState`).
    ```json
    {
        "type": "pairing_response",
        "status": "accepted" | "refused" | "already_paired",
        "reason": "message_si_refusé",
        "appState": "Visualize" // ou "Edit", "Main", etc.
    }
    ```
    *   Si le statut est `accepted` ou `already_paired`, l'interface de la télécommande s'adapte à l'`appState` reçu, affichant la page de contrôles correspondante.

### 2.4. Envoi de Commandes

Une fois couplée, la télécommande envoie des commandes via des messages de type `command`. Le message peut inclure un `payload` pour des données additionnelles.

```json
{
    "type": "command",
    "clientId": "votre_client_id_unique",
    "command": "toggle_play", // Exemple
    "payload": null // Données optionnelles
}
```
Les commandes incluent `toggle_play`, `toggle_altitude_profile`, `save_circuit`, `preview_circuit`, etc.

### 2.5. Réception des Mises à Jour d'État

La télécommande reçoit deux types de messages du serveur :

*   **`app_state_update`**: Indique un changement de vue dans l'application principale. La télécommande met à jour son interface pour afficher la page de contrôles pertinente.
    ```json
    {
        "type": "app_state_update",
        "appState": "Edit" 
    }
    ```
*   **`command_response`**: Accusé de réception pour chaque commande envoyée, informant la télécommande si la commande a réussi ou échoué.
    ```json
    {
        "type": "command_response",
        "status": "success",
        "message": "Commande toggle_play reçue.",
        "app_state": "Visualize"
    }
    ```

## 3. Le Serveur (Backend Rust)

Le backend Rust gère un serveur qui remplit un double rôle : serveur HTTP pour distribuer l'application client de la télécommande, et serveur WebSocket pour la communication en temps réel. Il est implémenté dans les modules `remote_setup.rs`, `remote_control.rs`, et `remote_clients.rs`.

### 3.1. Rôle et Initialisation

*   Au démarrage (`init_remote_control`), le backend lance un serveur TCP qui écoute sur un port configurable (défaut `9001`).
*   Ce serveur est capable de distinguer les requêtes HTTP (pour servir les fichiers `index.html`, `style.css`, `main.js`) des requêtes de mise à niveau WebSocket.
*   Pour les requêtes HTTP vers `/remote/main.js`, le serveur injecte dynamiquement son IP locale et son port dans le template JavaScript avant de l'envoyer.

### 3.2. Gestion des Clients et Persistance

*   Le module `remote_clients.rs` gère la persistance des télécommandes autorisées.
*   Un fichier `remote.json` est créé dans le répertoire de l'environnement d'exécution.
*   Ce fichier stocke une liste d'objets `RemoteClient`, contenant leur `client_id`, un nom, et la date de dernière connexion.
*   Lorsqu'une télécommande se connecte, le serveur vérifie si son `client_id` est présent dans `remote.json` pour l'autoriser automatiquement.

### 3.3. Traitement des Requêtes de Couplage

Lorsqu'un message `pairing_request` est reçu :

1.  Le serveur vérifie d'abord si le `clientId` est déjà autorisé via `remote_clients::is_client_authorized`.
2.  **Si oui**, il répond immédiatement avec `pairing_response` {`status: "accepted"`} et l'état actuel de l'application.
3.  **Si non**, il émet un événement Tauri `ask_pairing_approval` vers le frontend Vue.js, contenant le `clientId` et le `pairingCode`.
4.  Le frontend affiche une modale de confirmation à l'utilisateur.
5.  L'utilisateur accepte ou refuse, ce qui déclenche une commande Tauri `reply_to_pairing_request` vers le backend.
6.  Le backend reçoit la réponse, ajoute le client à `remote.json` si accepté, et envoie la `pairing_response` finale à la télécommande.

### 3.4. Traitement des Commandes Reçues

Lorsqu'un message `command` est reçu :

1.  Le serveur vérifie que le `clientId` est autorisé.
2.  Il émet un événement Tauri spécifique à la commande vers le frontend, par exemple `remote_command::toggle_play`.
3.  Il envoie immédiatement une `command_response` à la télécommande pour accuser réception.

### 3.5. Envoi des Mises à Jour d'État

Le serveur envoie des messages `app_state_update` à toutes les télécommandes connectées chaque fois que la vue de l'application principale change. Cette information est récupérée depuis l'état global `AppState` de Tauri, qui est mis à jour par le routeur Vue.js.

## 4. Communication entre Backend Rust et Frontend Vue.js

### 4.1. Transmission des Commandes au Frontend

Le backend émet des événements Tauri avec un nommage spécifique :

*   **Côté Rust (Backend)**:
    ```rust
    // Dans le handler de message WebSocket
    app_handle.emit(&format!("remote_command::{}", remote_command.command), remote_command.payload)
        .expect("Failed to emit remote command event");
    ```

### 4.2. Réaction du Frontend aux Commandes

Le frontend Vue.js écoute ces événements spécifiques pour déclencher les actions.

*   **Côté Vue.js (Frontend)**:
    ```javascript
    import { listen } from '@tauri-apps/api/event';

    // Dans un composant, ex: VisualizeView.vue
    listen('remote_command::toggle_play', (event) => {
      console.log('Commande toggle_play reçue!');
      // Logique pour basculer la lecture/pause
    });

    listen('remote_command::save_circuit', (event) => {
        // etc.
    });
    ```

### 4.3. Mise à Jour de l'État de l'Application (Frontend -> Backend -> Télécommande)

Le flux est maintenant plus direct :

1.  **Côté Vue.js**: Le routeur de Vue (`vue-router`) change la vue.
2.  Un `watcher` sur la route actuelle met à jour une variable dans l'état global `AppState` de Tauri via une commande (ex: `update_current_view`).
3.  **Côté Rust**: Le backend, via la fonction `send_app_state_update`, lit périodiquement ou sur déclenchement cet `AppState` et envoie le message `app_state_update` aux télécommandes si l'état a changé.

## 5. Flux de Communication Global

1.  **Initialisation & Service**: 
    *   Le serveur Rust démarre et écoute.
    *   L'utilisateur de la télécommande navigue vers `http://<ip_serveur>:<port>/remote`.
    *   Le serveur sert le client web avec la configuration réseau injectée.
2.  **Couplage**:
    *   La télécommande se connecte au WebSocket et envoie `pairing_request`.
    *   Le serveur vérifie `remote.json`. Si le client est inconnu, il demande validation à l'utilisateur desktop. Si connu, il accepte directement.
    *   Le serveur répond `pairing_response` avec l'état de l'application.
    *   La télécommande affiche l'interface corresp
* **Blacklist des clients refusés** : Lorsqu'une demande de couplage est refusée par l'utilisateur, l'identifiant du client est ajouté à un fichier `remote_blacklist.json`. Toute tentative future de connexion en provenance de cet appareil est immédiatement rejetée avec un message explicite ("Cet appareil a été bloqué.").  
* **Révocation manuelle de l'autorisation** : La déconnexion volontaire d'une télécommande depuis l'application principale révoque désormais son autorisation. Le client est supprimé du fichier `remote.json` et devra se coupler de nouveau via le QR code pour se reconnecter.  
* **Connexion unique et restrictions de couplage** : Le serveur n'autorise qu'un seul client connecté à la fois. Les tentatives de connexion d'un deuxième appareil sont refusées. De plus, le couplage d'une nouvelle télécommande n'est possible que lorsque l'application desktop se trouve sur la page d'accueil ou dans les paramètres.  
* **Indicateur de statut** : Une icône dans la barre d'outils principale indique l'état de la télécommande : verte (mdi-remote) lorsqu'une télécommande est connectée et active, bleue (mdi-remote-off) sinon.  
* **Affichage du QR code de connexion** : Lorsqu'aucune télécommande n'est connectée, un clic sur l'icône de télécommande ouvre une fenêtre modale affichant l'URL de connexion dynamique et un QR code généré à la volée par le backend. Cela facilite le processus de couplage pour les nouveaux appareils.ondant à l'état.
3.  **Envoi de Commande**:
    *   L'utilisateur clique sur un bouton de la télécommande.
    *   La télécommande envoie `command` au serveur.
    *   Le serveur émet un événement Tauri (`remote_command::...`) vers le frontend Vue.js.
    *   Le serveur renvoie une `command_response` à la télécommande.
    *   Le frontend Vue.js exécute l'action.

## 6. Considérations et Améliorations Futures

*   **Adresse IP Codée en Dur**: **Résolu**. L'adresse IP et le port sont maintenant injectés dynamiquement par le serveur Rust lors du service des fichiers du client web.
*   **Sécurité du Couplage**: Le système utilise maintenant un code pour le premier couplage et un mécanisme de liste blanche persistante (`remote.json`), ce qui améliore la sécurité.
*   **Gestion des Clients**: **Résolu**. La persistance des clients autorisés est gérée par le module `remote_clients.rs` et le fichier `remote.json`.
*   **Robustesse des Commandes**: La communication est plus robuste grâce aux messages `command_response` qui fournissent un retour immédiat à la télécommande.
*   **Feedback Visuel**: **Partiellement résolu**. La télécommande reçoit une confirmation pour chaque commande, mais pourrait être améliorée pour montrer l'état actuel des widgets (ex: si le profil d'altitude est visible ou non).
