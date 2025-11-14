#  Plan de Développement - Gestion Centralisée des Messages

## 1. Objectif

L'objectif de cette évolution est de refondre le système de gestion des messages pour passer d'un modèle où les messages sont définis par circuit à un modèle de **bibliothèque de messages centralisée et réutilisable**.

Cela permettra de :
-   **Mutualiser** les messages communs à plusieurs circuits.
-   **Simplifier** l'ajout de messages lors de l'édition d'un circuit.
-   **Faciliter la maintenance** et la mise à jour des messages.

## 2. Architecture Cible

### 2.1. Stockage des Données

Nous allons introduire deux fichiers pour la bibliothèque de messages et modifier le fichier d'événements existant.

-   **`public/messages_default.json`** : Un fichier JSON contenant une liste de messages génériques, livrés avec l'application. Ce fichier est en lecture seule pour l'utilisateur final (en mode production).
-   **`{dossier_app_data}/messages_user.json`** : Un fichier JSON dans le dossier de l'utilisateur, contenant ses propres messages personnalisés. Ce fichier sera créé s'il n'existe pas.
-   **`{dossier_app_data}/data/{id_circuit}/evt.json`** : Ce fichier sera modifié. Au lieu de stocker le contenu complet du message, il ne stockera plus qu'une référence (un ID) vers un message de la bibliothèque.

### 2.2. Format des Données

**`messages_default.json` / `messages_user.json` :**
```json
[
  {
    "id": "Ravitaillement_blue-darken-2_skewed-rect", // ID généré : text_couleur_shape
    "text": "Ravitaillement",
    "style": {
      "backgroundColor": "blue-darken-2",
      "textColor": "white",
      "shape": "skewed-rect" 
    }
  },
  {
    "id": "Point_eau_cyan-darken-3_skewed-rect",
    "text": "Point d'eau",
    "style": {
      "backgroundColor": "cyan-darken-3",
      "textColor": "white",
      "shape": "skewed-rect"
    }
  }
]
```
**Note :** La commande qui lira ces messages ajoutera un champ `"source": "default"` ou `"source": "user"` pour que le frontend sache d'où vient chaque message. L'`id` d'un message sera généré automatiquement sous la forme `text_couleur_shape` pour garantir son unicité et sa traçabilité. **Le texte des messages est généralement court (ex: "Ravitaillement", "Départ", "km 10").**

**`evt.json` (nouveau format pour `range_events`) :**
```json
{
  // ...
  "rangeEvents": [
    {
      "eventId": "uuid-de-l-instance-1234",
      "messageId": "Ravitaillement_blue-darken-2_skewed-rect", // Référence à la bibliothèque
      "anchorIncrement": 150,
      "startIncrement": 140,
      "endIncrement": 170,
      "coord": [2.3522, 48.8566]
    }
  ]
}
```

---

## 3. Phases de Développement

### Phase 1 : Backend et Structure des Données

**Objectif :** Mettre en place la logique côté Rust pour lire, fusionner, écrire et sécuriser les bibliothèques de messages.

-   **Étape 1.1 : Création du fichier de messages par défaut**
    -   Créer le fichier `public/messages_default.json` avec 2 ou 3 messages génériques.

-   **Étape 1.2 : Implémentation des commandes Tauri (Rust)**
    -   Modifier la commande `get_message_library()` pour qu'elle :
        1.  Lit `public/messages_default.json` et ajoute un champ `"source": "default"` à chaque message.
        2.  Lit `{dossier_app_data}/messages_user.json` et ajoute un champ `"source": "user"` à chaque message.
        3.  Fusionne les deux listes (les messages utilisateur ayant priorité en cas de conflit d'ID).
        4.  Retourne la liste complète et enrichie au frontend.
    -   Créer une commande `save_message(message: Message, target: String)` qui ajoute ou met à jour un message. L'`id` du message sera généré automatiquement sous la forme `text_couleur_shape`. Le paramètre `target` ("user" ou "default") déterminera le fichier à modifier.
    -   Créer une commande `delete_message(id: String, target: String)` qui supprime un message par ID du fichier cible.
    -   **Sécurité :** Utiliser la compilation conditionnelle (`#[cfg(debug_assertions)]`) en Rust pour que les écritures (sauvegarde, suppression) dans le fichier `default` ne soient possibles **qu'en mode développement**. En mode production, toute tentative de modification du fichier `default` retournera une erreur.

-   **Étape 1.3 : Modification de la gestion des événements**
    -   Modifier la commande `add_message_event` pour qu'elle accepte le nouveau format (`messageId`, `pre_affichage`, `post_affichage`, `coord`) et l'écrive dans `evt.json`. Il n'y aura plus d'`overrides` à gérer à ce niveau.
    -   Modifier la commande `get_events` pour qu'elle "hydrate" les événements. Pour chaque `RangeEvent` d'un circuit, elle devra :
        1.  Lire le `messageId`.
        2.  Trouver le message correspondant dans la bibliothèque (en appelant la logique de `get_message_library`).
        3.  Retourner un `RangeEvent` complet et prêt à être affiché par le frontend, avec les propriétés de style directement issues du message de la bibliothèque.

-   **🧪 Tests de validation (Phase 1) :**
    1.  Utiliser le **Tauri DevTools > Network** pour appeler `invoke('get_message_library')` et vérifier que la liste fusionnée contient bien le champ `source` pour chaque message.
    2.  En mode dev, appeler `save_message` avec `target: "default"` et vérifier que `public/messages_default.json` est modifié. Faire de même pour `delete_message`.
    3.  Appeler `save_message` avec `target: "user"` et vérifier que `messages_user.json` est modifié.
    4.  (Théorique) S'assurer que la compilation en mode production empêche bien la modification des messages par défaut.
    5.  Simuler un appel à `add_message_event` et vérifier manuellement que le `evt.json` du circuit contient bien le `messageId`.
    6.  Appeler `get_events` pour ce circuit et vérifier que le message retourné est bien complet (hydraté).

### Phase 2 : Composant de Gestion des Messages

**Objectif :** Créer une interface modale pour que l'utilisateur puisse gérer sa bibliothèque de messages, en tenant compte du mode de l'application.

-   **Étape 2.1 : Création du composant `MessageLibraryModal.vue`**
    -   Créer un nouveau fichier `src/components/MessageLibraryModal.vue`.
    -   Le composant sera une modale (`v-dialog`) contenant un `v-card`.

    **Accès à la page de test pour `MessageLibraryModal.vue` :**
    Pour tester ce composant de manière isolée, nous allons créer une route temporaire et un moyen d'y accéder :
    1.  **Créer la vue de test :**
        *   Créer un fichier `src/views/TestMessageLibraryView.vue` avec le contenu suivant :
            ```vue
            <template>
              <v-container fluid class="fill-height d-flex justify-center align-center">
                <v-btn @click="showModal = true">Ouvrir la Bibliothèque de Messages</v-btn>
                <MessageLibraryModal v-model="showModal" @select-message="handleSelectMessage" />
              </v-container>
            </template>

            <script setup>
            import { ref } from 'vue';
            import MessageLibraryModal from '@/components/MessageLibraryModal.vue';
            import { useSnackbar } from '@/composables/useSnackbar';

            const showModal = ref(false);
            const { showSnackbar } = useSnackbar();

            const handleSelectMessage = (messageId) => {
              showSnackbar(`Message sélectionné : ${messageId}`, 'success');
              showModal.value = false;
            };
            </script>
            ```
    2.  **Ajouter la route :**
        *   Modifier `src/router/index.js` pour ajouter la route suivante dans le tableau `routes` :
            ```javascript
            {
              path: '/test-messages',
              name: 'TestMessages',
              component: () => import('../views/TestMessageLibraryView.vue')
            },
            ```
    3.  **Accéder à la route de test :**
        *   Exécuter `npm run tauri dev`.
        *   **Pour le test :** Modifier temporairement `src/router/index.js` pour que la route par défaut (`/`) pointe vers `/test-messages`.
            ```javascript
            // Dans src/router/index.js
            const routes = [
              {
                path: '/',
                name: 'Home',
                // Temporairement, pour le test de MessageLibraryModal.vue
                // component: () => import('../views/MainView.vue')
                component: () => import('../views/TestMessageLibraryView.vue') // <-- Modifier ici
              },
              // ... autres routes
            ];
            ```
        *   Une fois l'application lancée, la vue de test s'affichera directement. **N'oubliez pas de revenir à la configuration originale après le test.**

-   **Étape 2.2 : Affichage de la bibliothèque**
    -   Au montage, le composant appelle `get_message_library()` pour récupérer tous les messages (enrichis avec leur `source`).
    -   Afficher les messages dans une liste (`v-list`). Chaque item affichera le texte du message et un aperçu de son style (couleur de fond, couleur de texte, forme).

-   **Étape 2.3 : CRUD des messages**
    -   Dans le formulaire d'ajout/modification, l'utilisateur pourra saisir le texte, la couleur de fond, la couleur du texte et la forme. L'`id` du message sera généré automatiquement à partir de ces champs.
    -   Ajouter un sélecteur de destination ("Public (défaut)" / "Utilisateur"). Ce sélecteur ne sera visible **qu'en mode développement** (en utilisant `v-if="import.meta.env.DEV"`).
    -   Lors de la sauvegarde, appeler `save_message` avec la `target` sélectionnée.
    -   Les boutons "Modifier" et "Supprimer" seront désactivés si `message.source === 'default'` et si l'application est en mode production (`!import.meta.env.DEV`).

-   **Étape 2.4 : Sélection d'un message**
    -   Ajouter un bouton "Sélectionner" sur chaque item de la liste.
    -   Lorsque l'utilisateur clique sur "Sélectionner", le composant doit émettre un événement (`@select-message`) avec l'`id` du message choisi et fermer la modale.

-   **🧪 Tests de validation (Phase 2) :**
    1.  Lancer la page de test.
    2.  **En mode dev :**
        *   Vérifier que le sélecteur de destination est visible dans le formulaire d'édition.
        *   Vérifier que les boutons "Modifier"/"Supprimer" sont actifs pour TOUS les messages.
        *   Tester la sauvegarde d'un message dans la bibliothèque "Publique" et "Utilisateur".
    3.  **Simuler le mode prod (si possible, sinon revue de code) :**
        *   Vérifier que le sélecteur de destination est bien masqué.
        *   Vérifier que les boutons "Modifier"/"Supprimer" sont désactivés pour les messages par défaut.
    4.  Vérifier que la sélection d'un message émet bien l'événement attendu.

### Phase 3 : Intégration dans la Vue d'Édition

**Objectif :** Remplacer l'ancienne interface de création de message par la nouvelle, basée sur la bibliothèque.

-   **Étape 3.1 : Refonte de l'onglet "Message"**
    -   Modifier `src/components/ControlTabsWidget.vue`.
    -   Supprimer les champs de saisie de texte, les sélecteurs de couleur, etc.
    -   L'onglet affichera désormais :
        -   Le texte du message actuellement sélectionné pour l'incrément courant (s'il y en a un).
        -   Un bouton "Choisir un message" qui ouvrira `MessageLibraryModal.vue`.
        -   Le `RangeSlider` pour la durée (pré/post affichage).
        -   Un bouton "Ajouter au circuit" / "Mettre à jour".

-   **Étape 3.2 : Logique d'interaction**
    -   Dans `EditView.vue`, ajouter la logique pour afficher `MessageLibraryModal.vue`.
    -   Écouter l'événement `@select-message` du modal pour savoir quel message l'utilisateur a choisi.
    -   Modifier `handleAddMessageEvent` dans `EditView.vue` pour qu'il envoie le `messageId` sélectionné, ainsi que la durée (pré/post affichage). Il n'y aura plus de surcharges (`overrides`) à envoyer.

-   **🧪 Tests de validation (Phase 3) :**
    1.  Ouvrir la vue d'édition.
    2.  Aller sur l'onglet "Message" et vérifier que la nouvelle interface s'affiche.
    3.  Cliquer sur "Choisir un message" et vérifier que la modale de la bibliothèque s'ouvre.
    4.  Sélectionner un message. La modale doit se fermer et le nom du message doit s'afficher dans l'onglet.
    5.  Cliquer sur "Ajouter au circuit". Vérifier (via les logs ou en inspectant le fichier `evt.json`) que l'événement est ajouté avec le bon `messageId`.
    6.  Se déplacer sur un incrément où un message existe déjà et vérifier que les informations (nom, durée, taille) sont correctement affichées.

### Phase 4 : Adaptation de la Vue de Visualisation

**Objectif :** S'assurer que la vue de visualisation affiche correctement les messages en utilisant le nouveau système et le format SVG.

-   **Étape 4.1 : Adaptation de l'affichage**
    -   Dans `VisualizeView.vue`, la commande `get_events` (modifiée en Phase 1) devrait déjà fournir les `RangeEvent` complets et hydratés.
    -   Modifier la création des popups Mapbox pour utiliser une fonction `createMessageSVG`.
    -   Cette fonction `createMessageSVG` devra générer un SVG dynamique basé sur le modèle suivant, en utilisant les données du message hydraté (texte, couleurs) pour ajuster les dimensions et la taille de la police du SVG.

    ```xml
    <svg viewBox="-0 -178 500 325" xmlns="http://www.w3.org/2000/svg">
      <!-- Le fill du rect sera lié à event.background_color -->
      <rect fill="#B71C1C" x="-0" y="0" width="500" height="150" rx="20" transform="skewY(-20)" />
      <!-- Le contenu du texte sera lié à event.text -->
      <!-- La position y du texte (110) et la taille de la police (100px) devront être ajustées dynamiquement -->
      <text text-anchor="start" x="9" y="110" transform="skewY(-20)">
        photo
      </text>

      <style>
        <![CDATA[
        text {
          font: bold 100px Bahnschrift, Semi-Bold Semi-Condensed; /* font-size et font-family dynamiques */
          fill: white; /* fill sera lié à event.text_color */
        }
        ]]>
      </style>
    </svg>
    ```

-   **🧪 Tests de validation (Phase 4) :**
    1.  Lancer la visualisation d'un circuit contenant des événements de message.
    2.  Vérifier que les messages s'affichent au bon moment et au bon endroit.
    3.  Vérifier qu'ils ont bien la nouvelle forme SVG.
    4.  Tester un message avec une surcharge de texte et/ou de taille et vérifier qu'elle est bien appliquée.

### Phase 5 : Finalisation

**Objectif :** Nettoyer le code et s'assurer que l'évolution est stable.

-   **Étape 5.1 : Nettoyage du code**
    -   Supprimer l'ancien code de gestion des messages (propriétés, fonctions) qui n'est plus utilisé.
    -   Supprimer les anciens paramètres de `settingsDefault.json` liés au style des messages, car le style est maintenant géré dans la bibliothèque.
    -   Supprimer la route temporaire `/test-messages` de `src/router/index.js`.
    -   Supprimer le fichier `src/views/TestMessageLibraryView.vue`.
-   **Étape 5.2 : Tests de régression**
    -   Tester de nouveau toutes les fonctionnalités liées aux messages (ajout, suppression, édition, visualisation) pour s'assurer que rien n'a été cassé.
    -   Tester les autres types d'événements (Pause, Flyto) pour s'assurer qu'ils n'ont pas été affectés.
