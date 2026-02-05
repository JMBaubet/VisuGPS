# Plan d'Implémentation : Correction du FlyTo (Saut Temporel)

## Problème Identifié
L'utilisateur constate un "saut instantané" au lieu d'une transition fluide.
**Cause** : La fonction `handleJumpRequest` appelle `updateCameraPosition` pour calculer les paramètres cibles. Or, `updateCameraPosition` applique *immédiatement* ces paramètres à la carte (`map.setCenter`, etc.) avant même que `flyTo` ne soit lancé.

## Solution Technique
Refactoriser `useCameraInterpolator.js` pour séparer le **calcul** de l'**application** des paramètres de caméra.

### 1. Modification de `useCameraInterpolator.js`
*   Ajouter une option `apply: boolean` (par défaut `true`) dans les options de la fonction.
*   Modifier la valeur de retour : Au lieu de retourner uniquement le `bearing` (ou null), retourner un objet complet :
    ```javascript
    {
        bearing: number | null, // Le bearing calculé (pour la boussole)
        target: {               // Les paramètres calculés
            center: [lng, lat],
            zoom: number,
            pitch: number,
            bearing: number
        }
    }
    ```
*   Si `apply` est `true`, appliquer les changements à la carte (comportement actuel).

### 2. Mise à jour de `VisualizeView.vue` et `VisualizeView.vue`
*   **Dans `animateLoop`** : Adapter l'appel pour utiliser le nouveau format de retour.
    *   `const result = updateCameraPosition(...)`
    *   `if (result && result.bearing !== null) currentTraceBearing.value = result.bearing;`
*   **Dans `handleJumpRequest`** :
    *   Appeler `updateCameraPosition` avec `{ apply: false }`.
    *   Récupérer l'objet `target` retourné.
    *   Utiliser cet objet `target` pour alimenter `flyToPromise`.

## Vérification
*   Vérifier que l'animation standard fonctionne toujours (le `apply: true` par défaut).
*   Vérifier que le clic sur le graphe déclenche une transition fluide (`flyTo`) vers le point cible, sans saut préalable.
*   Vérifier la fluidité du Zoom/Pitch/Bearing pendant le vol.

## Fichiers Impactés
*   `src/composables/visualize/useCameraInterpolator.js`
*   `src/views/VisualizeView.vue`
*   `src/views/VisualizeView.vue`
