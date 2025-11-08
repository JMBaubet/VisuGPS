# Paramètres de Mise à Jour des Communes (Accueil/MajCommunes)

Ce document décrit les paramètres de configuration liés à la mise à jour des informations des communes, accessibles via la section `Accueil/MajCommunes` dans les paramètres de l'application.

---

## 1. Timers

Ces paramètres contrôlent la fréquence des requêtes vers les différentes APIs utilisées pour la mise à jour des données des communes.

### 1.1. `timerIGN`

*   **Libellé**: Timer IGN (ms)
*   **Description**: Délai en millisecondes entre chaque interrogation du géoportail de l'IGN.
*   **Type**: Entier
*   **Valeur par défaut**: 200
*   **Minimum**: 50
*   **Maximum**: 2000
*   **Unité**: ms

### 1.2. `timerMapbox`

*   **Libellé**: Timer Mapbox (ms)
*   **Description**: Délai en millisecondes entre chaque interrogation de l'API Mapbox.
*   **Type**: Entier
*   **Valeur par défaut**: 200
*   **Minimum**: 50
*   **Maximum**: 2000
*   **Unité**: ms

### 1.3. `timerOSM`

*   **Libellé**: Timer OpenStreetMap (ms)
*   **Description**: Délai en millisecondes entre chaque interrogation de l'API OpenStreetMap.
*   **Type**: Entier
*   **Valeur par défaut**: 1000
*   **Minimum**: 250
*   **Maximum**: 5000
*   **Unité**: ms

---

## 2. APIs

Ces paramètres permettent d'activer ou de désactiver l'utilisation de certaines APIs pour la mise à jour des communes.

### 2.1. `ignActif`

*   **Libellé**: Activer l'API IGN par défaut
*   **Description**: Si coché, les requêtes vers l'API de l'IGN seront activées.
*   **Type**: Booléen
*   **Valeur par défaut**: `true`

### 2.2. `mapboxActif`

*   **Libellé**: Activer l'API Mapbox par défaut
*   **Description**: Si coché, les requêtes vers l'API Mapbox seront activées.
*   **Type**: Booléen
*   **Valeur par défaut**: `false`
*   **Critique**: `true` (indique que ce paramètre est important et peut avoir des conséquences significatives sur le comportement de l'application).

---

## 3. Justification des Paramètres "Timers"

Ce document explique la nécessité et l'importance des paramètres de temporisation (`timerIGN`, `timerMapbox`, `timerOSM`) présents dans la section `Accueil/MajCommunes` des paramètres de VisuGPS.

---

### 3.1. Le Contexte : La Mise à Jour des Communes

La fonctionnalité de mise à jour des communes consiste à interroger des services cartographiques externes pour obtenir le nom de la commune correspondant à des coordonnées géographiques (un processus appelé **géocodage inversé**). Lorsqu'un grand nombre de points doit être traité, l'application peut être amenée à envoyer des centaines, voire des milliers de requêtes à ces services.

Les paramètres `Timers` permettent d'introduire un délai artificiel entre chaque requête envoyée à un service donné. Loin d'être une simple optimisation, cette temporisation est un **mécanisme de protection indispensable** pour plusieurs raisons critiques.

### 3.2. Raison n°1 : Respecter les Limites de Débit (Rate Limiting)

C'est la justification la plus importante. Tous les fournisseurs d'API, qu'ils soient publics, communautaires ou commerciaux, imposent une **limite sur le nombre de requêtes** qu'un utilisateur peut effectuer sur une période donnée (par exemple, des requêtes par seconde ou par minute). Cette pratique, appelée *rate limiting*, est essentielle pour eux afin de :
-   Garantir une qualité de service équitable à tous les utilisateurs.
-   Se protéger contre les abus et les attaques par déni de service (DoS).
-   Gérer la charge de leur infrastructure serveur.

Voici les politiques des services utilisés par VisuGPS :

*   **OpenStreetMap (via Nominatim)**: La politique est très stricte avec une limite absolue de **1 requête par seconde**. Le service étant financé par des dons et maintenu par une communauté, il est crucial de ne pas le surcharger. Le non-respect de cette règle conduit quasi systématiquement à un bannissement de l'adresse IP.
    *   Le `timerOSM` (défaut : 1000 ms) est donc réglé pour respecter scrupuleusement cette limite.

*   **IGN Géoportail**: Les limites varient selon le service. Pour l'autocomplétion (le service le plus proche du géocodage inversé en termes de charge), la limite est de **10 requêtes/seconde**. Un dépassement entraîne un blocage temporaire de 5 secondes.
    *   Le `timerIGN` (défaut : 200 ms, soit 5 requêtes/seconde) est réglé de manière conservatrice pour rester bien en deçà de cette limite.

*   **Mapbox**: La limite est plus élevée (généralement 1000 requêtes/minute pour le géocodage), mais elle existe et dépend du contrat. Un pic soudain de centaines de requêtes pourrait la déclencher.
    *   Le `timerMapbox` (défaut : 200 ms) permet de lisser les requêtes et de s'assurer de ne jamais atteindre cette limite, même lors du traitement d'un grand circuit.

### 3.3. Raison n°2 : Éviter le Blocage d'IP (IP Ban)

Envoyer des requêtes en rafale, même si le nombre total reste sous la limite journalière, est un comportement suspect pour un serveur. Cela peut être interprété comme une attaque ou un bug. En conséquence, les fournisseurs d'API utilisent des mécanismes automatiques qui **bloquent temporairement (ou parfois de façon permanente) l'adresse IP** source pour se protéger.

Les paramètres `Timers` agissent comme un mécanisme de **throttling côté client**. Ils forcent VisuGPS à s'auto-limiter, garantissant que les requêtes sont espacées dans le temps et ne forment jamais de rafales. C'est une stratégie proactive pour ne jamais être considéré comme un client "agressif" et pour éviter tout blocage.

### 3.4. Raison n°3 : Assurer la Stabilité de l'Application

Sans temporisation, le traitement d'un circuit de 100 km avec un point tous les 100 mètres (1000 points) pourrait déclencher 1000 requêtes réseau quasi simultanément.
*   **Saturation Réseau**: Cela pourrait saturer la connexion internet de l'utilisateur.
*   **Consommation de Ressources**: Cela consommerait une quantité importante de ressources système (mémoire, CPU) pour gérer toutes ces connexions concurrentes, ce qui pourrait ralentir, voire faire planter l'application.

Les timers permettent de créer une **file d'attente de requêtes** qui sont traitées de manière séquentielle et contrôlée (une toutes les X millisecondes). Cela assure une charge de travail constante et prévisible pour l'application, garantissant une expérience utilisateur fluide et stable.

---

## Conclusion

En résumé, les paramètres `timerIGN`, `timerMapbox`, et `timerOSM` sont des **garde-fous critiques**. Ils ne sont pas là pour ralentir l'application, mais au contraire pour garantir son bon fonctionnement sur le long terme en :
1.  **Respectant** les conditions d'utilisation des services tiers.
2.  **Prévenant** le blocage de l'application par ces services.
3.  **Assurant** la stabilité et la réactivité de VisuGPS pendant les tâches de fond.

Ils permettent à VisuGPS d'être un "bon citoyen" du web des API.