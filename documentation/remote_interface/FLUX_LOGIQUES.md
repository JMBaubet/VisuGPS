# Flux Logiques : Couplage, Connexion et Heartbeat

Ce document décrit les séquences logiques permettant d'établir et de maintenir la liaison entre le PC et la télécommande.

## 1. Flux de Couplage (Pairing)

Le couplage est nécessaire la première fois qu'un appareil se connecte ou si son autorisation a été révoquée.

```mermaid
sequenceDiagram
    participant Mobile as Téléphone
    participant Backend as Backend Rust (Axum)
    participant FS as Système de Fichiers (JSON)
    participant Desktop as App Desktop (JS/Vue)

    Note over Mobile: Scan du QR Code (ex: ABC123D4)
    Mobile->>Backend: POST /api/pair { clientId, code }
    
    Backend->>Backend: Vérification session active
    alt Session Occupée (Autre Client Actif)
        Backend-->>Mobile: 409 Conflict { "status": "busy" }
    else Session Libre
        Backend->>FS: Lecture remote_blacklist.json
        alt Est Blacklisté
            FS-->>Backend: Trouvé
            Backend-->>Mobile: 403 Forbidden { "status": "refused" }
        else Non Blacklisté
            Backend->>FS: Lecture remote.json
            alt Est déjà Autorisé
                FS-->>Backend: Trouvé (UUID identique)
                Backend-->>Mobile: 200 OK { "status": "already_paired" }
            else Nouveau Client
                Backend->>Desktop: Event "remote_pairing_request"
                Desktop-->>User: Affiche Dialogue d'Approbation
                
                alt Utilisateur Accepte
                    User->>Desktop: Clique "Accepter"
                    Desktop->>Backend: Command "reply_to_pairing_request" (accept=true)
                    Backend->>FS: Ajout dans remote.json
                    Backend-->>Mobile: 200 OK { "status": "accepted" }
                else Utilisateur Refuse (ou Timeout)
                    User->>Desktop: Clique "Refuser"
                    Desktop->>Backend: Command "reply_to_pairing_request" (accept=false)
                    Backend->>FS: Ajout dans remote_blacklist.json
                    Backend-->>Mobile: 403 Forbidden { "status": "refused" }
                end
            end
        end
    end
    
    Note over Mobile: Lancement SSE seulement si accepté
```

## 2. Système de Heartbeat (Maintien de Vie)

Le Heartbeat assure que le PC sait si la télécommande est toujours "là" malgré la nature asynchrone du réseau.

```mermaid
sequenceDiagram
    participant Mobile as Téléphone
    participant Backend as Backend (Axum)
    participant Monitor as Tâche de Fond (Rust)

    loop Toutes les 3 secondes
        Mobile->>Backend: GET /api/heartbeat?clientId=UUID
        Backend->>Backend: Vérif clientId vs active_client_id
        alt Match
            Backend->>Backend: Mise à jour AtomicI64 (Now)
            Backend-->>Mobile: 200 OK
        else Mis-match ou Busy
            Backend-->>Mobile: 409 Conflict
        end
    end

    loop Toutes les 5 secondes
        Monitor->>Monitor: Vérifie AtomicI64
        alt Différence > 10 secondes
            Monitor->>Backend: Réinitialise active_client_id = None
            Monitor->>Backend: Émet "disconnected"
            Note right of Monitor: Icône Desktop devient Bleue
        end
    end
```

## 3. Reconnexion et Watchdog

La télécommande mobile (v18+) implémente une double sécurité :
1.  **Reconnexion Native** : L'objet `EventSource` tente automatiquement de se reconnecter au flux SSE en cas de coupure réseau.
2.  **Watchdog JS** : Si aucun battement de coeur réussi n'a eu lieu depuis plus de 10s, la télécommande force un signal de vie immédiat pour tenter de "réveiller" la liaison.
3.  **App State Sync** : À chaque changement de vue sur le Desktop, une notification SSE est envoyée, forçant la télécommande à rafraîchir son interface et son état de connexion (Passage au vert).
