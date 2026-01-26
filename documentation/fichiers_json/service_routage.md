# Documentation Service : Routage Géospatial

Ce document décrit le fonctionnement technique de l'interrogation des services de routage tiers dans VisuGPS.

## 🚀 Services Supportés

L'application utilise principalement deux moteurs de routage :
1.  **GraphHopper** : Idéal pour les profils typés "Loisirs" et "VTT".
2.  **OpenRouteService (ORS)** : Très puissant pour les profils "Route" et "Pistes cyclables".

---

## ⚙️ Mécanisme d'interrogation

### 1. Trigger (Déclenchement)
Le service est appelé par le frontend dès qu'une modification structurelle est apportée à la variante :
*   **Ajout d'un point** (Ancre ou Waypoint) via un clic sur la carte.
*   **Suppression d'un point** spécifique.
*   **Changement du profil global** (Toolbar) : Impacte uniquement le segment en cours de création tant qu'il n'est pas finalisé.
*   **Mise à jour forcée** (Sidebar) : En cliquant sur l'icône de profil d'un segment, on force l'application des réglages actuels de la Toolbar sur ce segment précis (même s'il était verrouillé).

### 2. Verrouillage (Routing Lock)
Pour garantir la cohérence d'un tracé mixant plusieurs profils :
*   Un segment est **verrouillé** dès sa première finalisation (2ème ancre posée).
*   Une fois verrouillé, il ne répond plus aux changements globaux de la barre d'outils.
*   Cela permet de conserver un segment en mode "VTT" tout en traçant le suivant en mode "Route".

### 2. Flux de données
1.  **Frontend** : Envoie une liste de coordonnées `[lon, lat]` et le profil souhaité (ex: `racingbike`).
2.  **Backend (Rust)** : Reçoit la demande via la commande `calculate_route`.
3.  **Appel API** : Le backend effectue une requête HTTP POST/GET vers le service configuré (avec la clé API stockée dans les paramètres).
4.  **Retour** : Le service renvoie un GéoJSON contenant le chemin précis suivant les routes.

### 3. Stratégie de Résilience (Failover)
L'application implémente une stratégie de **bascule automatique** :
*   Si le service préféré (ex: GraphHopper) échoue, le backend tente automatiquement d'utiliser le second service (ex: ORS).
*   **Retry** : En cas d'erreur réseau temporaire, l'application effectue jusqu'à **3 tentatives** avec un délai exponentiel avant d'abandonner.
*   **Fallback ultime** : Si tous les services échouent, le backend génère une **ligne droite** entre les points pour ne pas bloquer l'utilisateur.

---

## 🚦 Profils de Routage Disponibles
Les profils varient selon le service mais sont harmonisés dans l'interface :
*   **Route (Racing Bike)**
*   **VTT (Mountain Bike)**
*   **Randonnée (Hike)**

---

## 🛠️ Configuration
Les clés API se configurent dans les paramètres de l'application sous :
`Variante > Paramètres > Clé API GraphHopper`
`Variante > Paramètres > Clé API OpenRouteService`
