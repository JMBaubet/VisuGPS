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
