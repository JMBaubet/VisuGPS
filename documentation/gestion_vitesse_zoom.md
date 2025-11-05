# Plan d'implémentation : Contrôle de la Vitesse Gyroscopique et Zoom Dynamique

## 🚀 Introduction

Le système actuel de gestion de la vitesse d'animation repose sur des paliers fixes (`speedSteps`) et des changements "brusques", ce qui limite la fluidité et la précision du contrôle. L'objectif de ce plan est de remplacer cette approche par un contrôle plus granulaire et intuitif :

1.  **Contrôle Continu de la Vitesse :** Utilisation d'un `v-slider` sur l'application desktop et d'un contrôle gyroscopique incrémental via smartphone (télécommande) pour ajuster la vitesse de l'animation de manière fluide.
2.  **Zoom Dynamique :** Implémentation d'une fonction mathématique pour calculer le coefficient de zoom en temps réel, s'adaptant à la vitesse d'animation actuelle.

Ce document décrit les différentes phases d'implémentation et les tests associés pour garantir une intégration progressive et robuste.

---

## 🎯 Principes Clés

*   **Vitesse Continue (`currentSpeed`) :** Remplacer l'index de palier par une variable `currentSpeed` qui peut varier de manière fluide au sein d'une plage définie.
*   **Contrôle Incrémental Gyroscopique :** L'inclinaison du smartphone (quand un bouton est pressé) ne définit pas une vitesse absolue, mais une "vitesse de changement de vitesse" (`delta_speed_rate`).
*   **Verrouillage de Vitesse :** Relâcher le bouton gyroscopique fige la vitesse à sa valeur actuelle.
*   **Retour Facile à 1x :** Un bouton dédié permettra de réinitialiser la vitesse à `1.0`.
*   **Fonction de Zoom Paramétrable :** Le coefficient de zoom sera calculé par une fonction `f(currentSpeed)` dont les paramètres seront configurables.

---

## 🏗️ Phases d'Implémentation et de Test

### Phase 1 : Centralisation des Paramètres de Vitesse et de Zoom

**Objectif :** Préparer la transition vers une vitesse continue en définissant les nouvelles propriétés dans `settingsDefault.json` et en nettoyant le code existant lié aux paliers de vitesse.

#### 1.1 Modifications dans `src-tauri/settingsDefault.json`

*   **Supprimer les paramètres de zoom par palier existants :** Les entrées sous `Visualisation/Animation/Zooms` comme `zoom_0_5x`, `zoom_1x`... seront supprimées, car la fonction mathématique les remplacera.
*   **Ajouter les nouveaux paramètres de vitesse :**
    *   `Visualisation/Animation/Vitesse/min_value` (type `reel`, ex: 0.1) : Vitesse minimale de l'animation.
    *   `Visualisation/Animation/Vitesse/max_value` (type `reel`, ex: 20.0) : Vitesse maximale de l'animation.
    *   `Visualisation/Animation/Vitesse/default_value` (type `reel`, ex: 1.0) : Vitesse par défaut au démarrage/réinitialisation.
    *   `Visualisation/Animation/Vitesse/slider_step` (type `reel`, ex: 0.05) : Pas d'incrémentation pour le slider.
*   **Ajouter les paramètres de la fonction de zoom dynamique :**
    *   Créer un nouveau groupe `Visualisation/Animation/ZoomDynamique`.
    *   `Visualisation/Animation/ZoomDynamique/constante_A` (type `reel`, ex: 1.0) : Première constante pour la fonction `f(vitesse)`.
    *   `Visualisation/Animation/ZoomDynamique/constante_B` (type `reel`, ex: 0.2) : Deuxième constante pour la fonction `f(vitesse)`.
    *   `Visualisation/Animation/ZoomDynamique/function_type` (type `string`, ex: "power") : Type de fonction mathématique (linéaire, exponentielle, puissance, etc.). (Initialement, on peut commencer avec une fonction simple type `A / (speed^B)` ou `A - B*log(speed)`).

#### 1.2 Modifications dans `src/views/VisualizeView.vue`

*   **Supprimer `speedSteps` et `speedIndex` :** Ces variables ne seront plus utilisées.
*   **Définir `currentSpeed` :** Initialiser `currentSpeed` (`ref`) à la valeur par défaut (`default_value`) lue depuis `settings.json`.
*   **Nettoyage :** Supprimer toutes les références, si possible, à `speedSteps[speedIndex]` dans le template et le script.

#### Tests (Phase 1) :

*   **Compilation/Démarrage :** Vérifier que l'application compile et démarre sans erreur.
*   **Pas d'erreurs de paramètres :** S'assurer que `getSettingValue` lit correctement les nouveaux paramètres et qu'il n'y a pas d'erreurs liées à des paramètres manquants.
*   **Vitesse initiale :** Vérifier que la vitesse affichée et appliquée à l'animation correspond à `default_value`.

---

### Phase 2 : Implémentation du Slider de Vitesse sur Desktop

**Objectif :** Remplacer les boutons de changement de vitesse par un `v-slider` pour un contrôle continu et visuel de la vitesse sur l'application desktop.

#### 2.1 Modifications dans `src/views/VisualizeView.vue`

*   **Template :**
    *   Remplacer les boutons `mdi-minus`, `mdi-plus` et le texte `x{{ speedSteps[speedIndex] }}` par un `v-slider` et un affichage de `currentSpeed`.
    *   Le `v-slider` devra être bindé à `currentSpeed`, avec `min`, `max` et `step` définis par les nouveaux paramètres.
    *   Ajouter un bouton "1x" à côté du slider pour réinitialiser `currentSpeed` à `default_value`.
*   **Script :**
    *   Mettre à jour la fonction `animate` pour utiliser `currentSpeed` directement (par exemple, `accumulatedTime += deltaTime * currentSpeed.value;`).
    *   Implémenter la logique du bouton "1x".
    *   Mettre à jour le `speedMultiplier` computed pour simplement retourner `currentSpeed.value`.

#### Tests (Phase 2) :

*   **Fonctionnalité du Slider :**
    *   Glisser le curseur du slider modifie la vitesse de l'animation de manière fluide.
    *   La vitesse de l'animation correspond à la valeur affichée par le slider.
    *   Les limites min/max du slider et le pas d'incrémentation sont respectés.
*   **Bouton "1x" :** Cliquer sur le bouton "1x" réinitialise la vitesse à la valeur par défaut.
*   **Consistance visuelle :** L'affichage de la vitesse reflète correctement `currentSpeed`.

---

### Phase 3 : Intégration du Contrôle Gyroscopique sur la Télécommande

**Objectif :** Permettre au smartphone d'envoyer des commandes de `delta_speed_rate` à l'application desktop via WebSocket, activées par un bouton.

#### 3.1 Modifications Côté Smartphone (`src/remote_client/`)

*   **Interface Utilisateur :**
    *   Ajouter un nouveau bouton "Ajuster Vitesse" (ou similaire) dans le template de la télécommande.
    *   Afficher la vitesse actuelle reçue du desktop.
*   **Script (JavaScript) :**
    *   **Événements Capteur :** Écouter les événements `DeviceOrientationEvent` ou `DeviceMotionEvent` (par exemple, via `window.addEventListener('deviceorientation', ...)`) lorsque le bouton est pressé.
    *   **Logique de Contrôle :**
        *   Lorsqu'un utilisateur appuie sur le bouton :
            *   Commence à lire l'angle de pitch du smartphone.
            *   Calcule la `delta_speed_rate` basée sur l'inclinaison (avec zone morte et lissage).
            *   Envoie la `delta_speed_rate` à l'application desktop via la connexion WebSocket existante.
        *   Lorsque l'utilisateur relâche le bouton :
            *   Arrête l'envoi des `delta_speed_rate` (ou envoie 0).

#### 3.2 Modifications Côté Desktop (`src-tauri/src/lib.rs` et `src/views/VisualizeView.vue`)

*   **Backend Rust :**
    *   Créer un nouvel événement Tauri (ou une commande) pour recevoir la `delta_speed_rate` du client distant. Ce pourrait être un événement "raw" WebSocket si votre système le permet, ou une commande Tauri `update_speed_rate_from_remote`.
*   **Frontend Vue.js (`VisualizeView.vue`) :**
    *   Écouter le nouvel événement (par exemple, via `listen('remote_command::update_speed_rate', (event) => { /* ... */ })`).
    *   Stocker la `delta_speed_rate` reçue dans une `ref` (par exemple, `remoteDeltaSpeedRate`).

#### Tests (Phase 3) :

*   **Connexion :** S'assurer que la télécommande se connecte correctement.
*   **Envoi `delta_speed_rate` :** Utiliser les outils de débogage du navigateur sur le smartphone pour vérifier que la `delta_speed_rate` est correctement calculée et envoyée via WebSocket lorsque le bouton est pressé/incliné.
*   **Réception `delta_speed_rate` :** Dans l'application desktop, logger la valeur de `remoteDeltaSpeedRate` pour s'assurer qu'elle est bien reçue et mise à jour.

---

### Phase 4 : Application de la `delta_speed_rate` à la `currentSpeed`

**Objectif :** Utiliser la `delta_speed_rate` reçue pour ajuster la `currentSpeed` de l'animation dans la boucle principale.

#### 4.1 Modifications dans `src/views/VisualizeView.vue`

*   **Variable d'État :** Introduire une `ref` `currentRemoteDeltaSpeedRate` initialisée à 0, qui sera mise à jour par l'événement de la phase 3.
*   **Boucle `animate` :**
    *   Dans la fonction `animate`, *avant* de calculer `accumulatedTime`, ajuster `currentSpeed.value` :
        `currentSpeed.value = Math.max(min_speed_value, Math.min(max_speed_value, currentSpeed.value + currentRemoteDeltaSpeedRate.value * deltaTime));`
    *   S'assurer que la `currentSpeed` reste dans les bornes `min_value` et `max_value` définies dans `settings.json`.

#### Tests (Phase 4) :

*   **Vitesse par gyroscopie :**
    *   Appuyer sur le bouton Gyro sur le smartphone et incliner le téléphone : la vitesse de l'animation doit progressivement augmenter ou diminuer en fonction de l'inclinaison.
    *   Relâcher le bouton : la vitesse doit se stabiliser.
    *   Faire des tests aux limites (inclinaison maximale) pour vérifier que la vitesse atteint bien les bornes min/max.
*   **Absence d'interférence :** Vérifier que le slider desktop fonctionne toujours correctement et qu'il n'y a pas de conflits inattendus.

---

### Phase 5 : Re-implémentation du Retour à 1x et des Boutons +/- (Télécommande)

**Objectif :** Fournir des méthodes robustes pour ajuster la vitesse via la télécommande, incluant un retour rapide à la vitesse par défaut.

#### 5.1 Modifications Côté Smartphone (`src/remote_client/`)

*   **Nouveaux Boutons :**
    *   Ajouter un bouton "1x" : Envoie une commande (`set_speed_to_1x`) à l'application de bureau.
    *   Ajouter des boutons pour ajuster la vitesse par "petits pas" (`+` et `-`) : Envoient des commandes (`increase_speed_step`, `decrease_speed_step`) à l'application de bureau. L'incrément/décrément serait une valeur fixe paramétrée.

#### 5.2 Modifications Côté Desktop (`src-tauri/src/lib.rs` et `src/views/VisualizeView.vue`)

*   **Backend Rust :**
    *   Ajouter de nouveaux événements/commandes Tauri pour `set_speed_to_1x`, `increase_speed_step`, `decrease_speed_step`.
*   **Frontend Vue.js (`VisualizeView.vue`) :**
    *   Écouter ces nouveaux événements et les utiliser pour directement modifier `currentSpeed.value` (en respectant les bornes min/max).

#### Tests (Phase 5) :

*   **Bouton "1x" Télécommande :** Vérifier que la vitesse revient à 1x.
*   **Boutons "+/-" Télécommande :** Vérifier que la vitesse augmente/diminue par incréments fixes.
*   **Intégration Globale :** S'assurer que tous les contrôles (desktop slider, gyroscopie, boutons télécommande) fonctionnent de concert sans conflit.

---

### Phase 6 : Fonction Mathématique pour le Zoom Dynamique

**Objectif :** Remplacer le simple multiplicateur de zoom par une fonction mathématique `f(currentSpeed)` qui définit le coefficient de zoom de manière plus élaborée.

#### 6.1 Modifications dans `src/views/VisualizeView.vue`

*   **Définition de la fonction :** Créer un `computed` `dynamicZoomCoefficient` qui utilise `currentSpeed.value` et les paramètres de `Visualisation/Animation/ZoomDynamique` (`constante_A`, `constante_B`, `function_type`) pour calculer le coefficient de zoom.
    *   Exemple de fonction : `coefficient = constante_A / (currentSpeed.value ** constante_B)`.
*   **Application :** Remplacer `zoom * zoomCoefficient.value` par `zoom * dynamicZoomCoefficient.value` dans la fonction `animate`.

#### Tests (Phase 6) :

*   **Comportement du zoom :**
    *   Faire varier la vitesse et observer le comportement du zoom. Pour les vitesses > 1x, le zoom devrait être plus faible que le zoom de base, et inversement pour les vitesses < 1x.
    *   Ajuster les constantes `A` et `B` dans `settings.json` et vérifier que le comportement du zoom change comme prévu.

---

## ⚠️ Considérations Générales

*   **Feedback UX :** Pendant toute l'implémentation, penser au feedback visuel et haptique. L'utilisateur doit toujours savoir quelle est la vitesse actuelle (affichage numérique), et si le contrôle gyroscopique est actif. Des vibrations peuvent être utilisées pour les limites de vitesse ou le passage à 1x.
*   **Messages d'Erreur :** Gérer les cas où les capteurs ne sont pas disponibles sur le smartphone ou où la communication est interrompue.
*   **Paramétrage :** S'assurer que toutes les constantes (zone morte, sensibilité d'inclinaison, pas d'incrémentation, constantes de la fonction de zoom) sont paramétrables via `settings.json` pour faciliter l'ajustement.

En suivant ce plan pas à pas, nous minimiserons les risques et construirons une fonctionnalité robuste et intuitive.
