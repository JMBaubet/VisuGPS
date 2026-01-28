# Spécifications de l'Interface de Communication (Backend <-> Télécommande)

Ce document détaille les points d'entrée (Endpoints) et les flux de données entre le serveur VisuGPS et la télécommande mobile.

## 1. Endpoints de l'API (HTTP)

Tous les appels API sont préfixés par `/api`.

| Méthode | Endpoint | Description | Payload |
| :--- | :--- | :--- | :--- |
| **GET** | `/api/health` | Vérification de l'état du serveur. | - |
| **POST** | `/api/pair` | Demande de couplage (Pairing). | `{ clientId, pairingCode, clientName }` |
| **POST** | `/api/command` | Envoi d'une commande au PC. | `{ command, payload, sessionToken }` |
| **GET** | `/api/events` | Flux SSE des événements (PC -> Mobile). | - |
| **GET** | `/api/heartbeat` | Signal de vie (Heartbeat). | - |
| **GET** | `/api/state` | Récupération forcée de l'état (Fallback). | - |

---

## 2. Server-Sent Events (SSE) : Flux `/api/events`

Le flux SSE est une connexion "ouverte" où le PC pousse des objets JSON vers le téléphone au format `data: { ... }`.

### 2.1 Événement `app_state_update`
Envoyé lorsque l'utilisateur change de vue sur le PC (Accueil, Visualisation, etc.).
```json
{
  "type": "app_state_update",
  "appState": "Main" | "Settings" | "Visualize" | "Edit"
}
```

### 2.2 Événement `animation_state_update`
Mis à jour à chaque changement de la machine d'état de l'animation.
```json
{
  "type": "animation_state_update",
  "animationState": "En_Animation" | "En_Pause" | "Termine" | "Arrete"
}
```

### 2.3 Événement `visualize_view_state_update`
Indique si les widgets (Profil, Vitesse, etc.) sont visibles ou masqués.
```json
{
  "type": "visualize_view_state_update",
  "state": {
    "isControlsCardVisible": true,
    "isAltitudeVisible": true,
    "isDistanceDisplayVisible": false,
    "isStaticWeatherVisible": false
  }
}
```

### 2.4 Événement `animation_speed_update`
Envoyé lors du changement de vitesse via le PC.
```json
{
  "type": "animation_speed_update",
  "speed": 1.5
}
```

---

## 3. Format des Commandes (Mobile -> PC)

Les commandes envoyées via `POST /api/command` suivent le format :
```json
{
  "command": "nom_de_la_commande",
  "payload": { ... }
}
```
*Note : Le payload est optionnel et dépend de la commande.*
