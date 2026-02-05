# Plan d'Implémentation : Durée de Saut Configurable

## Objectif
Permettre à l'utilisateur de configurer la durée de l'animation de saut temporel (FlyTo) via les paramètres de l'application.

## Changements

### 1. Mise à jour de `src-tauri/settingsDefault.json`
*   Ajouter un paramètre `jumpDuration` dans le groupe `Visualisation` > `Lecture`.
*   **Id** : `jumpDuration`
*   **Libellé** : `Durée déplacement`
*   **Type** : `reel` (ou `entier` si on veut des ms, la demande parle de pas de 0.25 donc `reel`)
*   **Défaut** : `2.0`
*   **Min** : `0`
*   **Max** : `10`
*   **Pas** : `0.25`
*   **Unit** : `s`
*   **Description** : "Durée de la transition lors d'un saut temporel (clic sur graphe)."

### 2. Mise à jour des Vues
*   **Fichiers** : `src/views/VisualizeView.vue` et `src/views/VisualizeView.vue`
*   **Action** :
    *   Récupérer la valeur du setting : `const jumpDuration = computed(() => getSettingValue('Visualisation/Lecture/jumpDuration') ?? 2.0);`
    *   Utiliser cette valeur dans `handleJumpRequest` pour le `flyToPromise`.
    *   Convertir en millisecondes si nécessaire (flyTo attend des ms ? Non, mon wrapper `flyToPromise` semble prendre ce qu'on lui donne, mais `map.flyTo` prend des ms par défaut ? Dans `VisualizeView` on voit `duration: 2000`. Donc c'est des ms. Il faudra multiplier par 1000).

## Vérification
*   Vérifier que le paramètre apparaît dans l'écran de paramétrage (géré automatiquement par `settingsDefault.json`).
*   Modifier la valeur (ex: 5s).
*   Faire un saut et chronométrer visuellement.
