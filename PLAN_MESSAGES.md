#  Plan de Développement - Gestion Centralisée des Messages 💬

## 1. Objectif 🎯

L'objectif de cette évolution était de refondre le système de gestion des messages pour passer d'un modèle où les messages étaient définis par circuit à un modèle de **bibliothèque de messages centralisée et réutilisable**.

Cela a permis de :
-   **Mutualiser** les messages communs à plusieurs circuits. 🔄
-   **Simplifier** l'ajout de messages lors de l'édition d'un circuit. ✨
-   **Faciliter la maintenance** et la mise à jour des messages. 🛠️

## 2. Architecture Cible 🏗️

### 2.1. Stockage des Données 💾

Nous avons introduit deux fichiers pour la bibliothèque de messages et modifié le fichier d'événements existant.

-   **`public/messages_default.json`** : Un fichier JSON contenant une liste de messages génériques, livrés avec l'application. Ce fichier est en lecture seule pour l'utilisateur final (en mode production). 🔒
-   **`{dossier_app_data}/messages_user.json`** : Un fichier JSON dans le dossier de l'utilisateur, contenant ses propres messages personnalisés. Ce fichier a été créé s'il n'existait pas. 🧑‍💻
-   **`{dossier_app_data}/data/{id_circuit}/evt.json`** : Ce fichier a été modifié. Au lieu de stocker le contenu complet du message, il ne stocke plus qu'une référence (un ID) vers un message de la bibliothèque. 🔗

### 2.2. Format des Données 📝

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
**Note :** La commande qui lit ces messages ajoute un champ `"source": "default"` ou `"source": "user"` pour que le frontend sache d'où vient chaque message. L'`id` d'un message est généré automatiquement sous la forme `text_couleur_shape` pour garantir son unicité et sa traçabilité. **Le texte des messages est généralement court (ex: "Ravitaillement", "Départ", "km 10").** 💡

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

## 3. Phases de Développement 🚀

### Phase 1 : Backend et Structure des Données ✅ Complétée

**Objectif :** La logique côté Rust pour lire, fusionner, écrire et sécuriser les bibliothèques de messages a été mise en place.

-   **Étape 1.1 : Création du fichier de messages par défaut** ✔️
    -   Le fichier `public/messages_default.json` avec 2 ou 3 messages génériques a été créé.

-   **Étape 1.2 : Implémentation des commandes Tauri (Rust)** ✔️
    -   La commande `get_message_library()` a été modifiée pour :
        1.  Lire `public/messages_default.json` et ajouter un champ `"source": "default"` à chaque message.
        2.  Lire `{dossier_app_data}/messages_user.json` et ajouter un champ `"source": "user"` à chaque message.
        3.  Fusionner les deux listes (les messages utilisateur ayant priorité en cas de conflit d'ID).
        4.  Retourner la liste complète et enrichie au frontend.
    -   Une commande `save_message(message: Message, target: String)` a été créée pour ajouter ou mettre à jour un message. L'`id` du message est généré automatiquement sous la forme `text_couleur_shape`. Le paramètre `target` ("user" ou "default") détermine le fichier à modifier.
    -   Une commande `delete_message(id: String, target: String)` a été créée pour supprimer un message par ID du fichier cible.
    -   **Sécurité :** La compilation conditionnelle (`#[cfg(debug_assertions)]`) en Rust a été utilisée pour que les écritures (sauvegarde, suppression) dans le fichier `default` ne soient possibles **qu'en mode développement**. En mode production, toute tentative de modification du fichier `default` retourne une erreur. 🛡️

-   **Étape 1.3 : Modification de la gestion des événements** ✔️
    -   La commande `add_message_event` a été modifiée pour accepter le nouveau format (`messageId`, `pre_affichage`, `post_affichage`, `coord`) et l'écrire dans `evt.json`. Il n'y a plus d'`overrides` à gérer à ce niveau.
    -   La commande `get_events` a été modifiée pour "hydrater" les événements. Pour chaque `RangeEvent` d'un circuit, elle :
        1.  Lit le `messageId`.
        2.  Trouve le message correspondant dans la bibliothèque (en appelant la logique de `get_message_library`).
        3.  Retourne un `RangeEvent` complet et prêt à être affiché par le frontend, avec les propriétés de style directement issues du message de la bibliothèque.

-   **🧪 Tests de validation (Phase 1) :** ✔️
    1.  Le **Tauri DevTools > Network** a été utilisé pour appeler `invoke('get_message_library')` et vérifier que la liste fusionnée contient bien le champ `source` pour chaque message.
    2.  En mode dev, `save_message` avec `target: "default"` a été appelé et il a été vérifié que `public/messages_default.json` est modifié. De même pour `delete_message`.
    3.  `save_message` avec `target: "user"` a été appelé et il a été vérifié que `messages_user.json` est modifié.
    4.  (Théorique) Il a été assuré que la compilation en mode production empêche bien la modification des messages par défaut.
    5.  Un appel à `add_message_event` a été simulé et il a été vérifié manuellement que le `evt.json` du circuit contient bien le `messageId`.
    6.  `get_events` a été appelé pour ce circuit et il a été vérifié que le message retourné est bien complet (hydraté).

### Phase 2 : Composant de Gestion des Messages 🖼️ ✅ Complétée

**Objectif :** Une interface modale pour que l'utilisateur puisse gérer sa bibliothèque de messages, en tenant compte du mode de l'application, a été créée.

-   **Étape 2.1 : Création du composant `MessageLibraryModal.vue`** ✔️
    -   Un nouveau fichier `src/components/MessageLibraryModal.vue` a été créé.
    -   Le composant est une modale (`v-dialog`) contenant un `v-card`.

    **Accès à la page de test pour `MessageLibraryModal.vue` :** ✔️
    Pour tester ce composant de manière isolée, une route temporaire et un moyen d'y accéder ont été créés :
    1.  **Créer la vue de test :** ✔️
        *   Un fichier `src/views/TestMessageLibraryView.vue` a été créé avec le contenu spécifié.
    2.  **Ajouter la route :** ✔️
        *   `src/router/index.js` a été modifié pour ajouter la route `/test-messages`.
    3.  **Accéder à la route de test :** ✔️
        *   `npm run tauri dev` a été exécuté.
        *   **Pour le test :** `src/router/index.js` a été modifié temporairement pour que la route par défaut (`/`) pointe vers `/test-messages`.

-   **Étape 2.2 : Affichage de la bibliothèque** ✔️
    -   Au montage, le composant appelle `get_message_library()` pour récupérer tous les messages (enrichis avec leur `source`).
    -   Les messages sont affichés dans une liste (`v-list`). Chaque item affiche le texte du message et un aperçu de son style (couleur de fond, couleur de texte, forme).

-   **Étape 2.3 : CRUD des messages** ✔️
    -   Dans le formulaire d'ajout/modification, l'utilisateur peut saisir le texte, la couleur de fond, la couleur du texte et la forme. L'`id` du message est généré automatiquement à partir de ces champs.
    -   Un sélecteur de destination ("Public (défaut)" / "Utilisateur") a été ajouté. Ce sélecteur est visible **uniquement en mode développement** (`v-if="import.meta.env.DEV"`).
    -   Lors de la sauvegarde, `save_message` est appelé avec la `target` sélectionnée.
    -   Les boutons "Modifier" et "Supprimer" sont désactivés si `message.source === 'default'` et si l'application est en mode production (`!import.meta.env.DEV`).

-   **Étape 2.4 : Sélection d'un message** ✔️
    -   Un bouton "Sélectionner" a été ajouté sur chaque item de la liste.
    -   Lorsque l'utilisateur clique sur "Sélectionner", le composant émet un événement (`@select-message`) avec l'`id` du message choisi et ferme la modale.

-   **🧪 Tests de validation (Phase 2) :** ✔️
    1.  La page de test a été lancée.
    2.  **En mode dev :**
        *   Il a été vérifié que le sélecteur de destination est visible dans le formulaire d'édition.
        *   Il a été vérifié que les boutons "Modifier"/"Supprimer" sont actifs pour TOUS les messages.
        *   La sauvegarde d'un message dans la bibliothèque "Publique" et "Utilisateur" a été testée.
    3.  **Simuler le mode prod (si possible, sinon revue de code) :**
        *   Il a été vérifié que le sélecteur de destination est bien masqué.
        *   Il a été vérifié que les boutons "Modifier"/"Supprimer" sont désactivés pour les messages par défaut.
    4.  Il a été vérifié que la sélection d'un message émet bien l'événement attendu.

### Phase 3 : Intégration dans la Vue d'Édition ✍️ ✅ Complétée

**Objectif :** L'ancienne interface de création de message a été remplacée par la nouvelle, basée sur la bibliothèque.

-   **Étape 3.1 : Refonte de l'onglet "Message"** ✔️
    -   `src/components/ControlTabsWidget.vue` a été modifié.
    -   Les champs de saisie de texte, les sélecteurs de couleur, etc. ont été supprimés.
    -   L'onglet affiche désormais :
        -   Le texte du message actuellement sélectionné pour l'incrément courant (s'il y en a un).
        -   Un bouton "Choisir un message" qui ouvre `MessageLibraryModal.vue`.
        -   Le `RangeSlider` pour la durée (pré/post affichage).
        -   Un bouton "Ajouter au circuit" / "Mettre à jour".

-   **Étape 3.2 : Logique d'interaction** ✔️
    -   Dans `EditView.vue`, la logique pour afficher `MessageLibraryModal.vue` a été ajoutée.
    -   L'événement `@select-message` du modal est écouté pour savoir quel message l'utilisateur a choisi.
    -   `handleAddMessageEvent` dans `EditView.vue` a été modifié pour envoyer le `messageId` sélectionné, ainsi que la durée (pré/post affichage). Il n'y a plus de surcharges (`overrides`) à envoyer.

-   **🧪 Tests de validation (Phase 3) :** ✔️
    1.  La vue d'édition a été ouverte.
    2.  L'onglet "Message" a été visité et il a été vérifié que la nouvelle interface s'affiche.
    3.  Le bouton "Choisir un message" a été cliqué et il a été vérifié que la modale de la bibliothèque s'ouvre.
    4.  Un message a été sélectionné. La modale se ferme et le nom du message s'affiche dans l'onglet.
    5.  Le bouton "Ajouter au circuit" a été cliqué. Il a été vérifié (via les logs ou en inspectant le fichier `evt.json`) que l'événement est ajouté avec le bon `messageId`.
    6.  Un déplacement sur un incrément où un message existe déjà a été effectué et il a été vérifié que les informations (nom, durée, taille) sont correctement affichées.

### Phase 4 : Adaptation de la Vue de Visualisation 👁️ ✅ Complétée

**Objectif :** Il a été assuré que la vue de visualisation affiche correctement les messages en utilisant le nouveau système et le format SVG.

-   **Étape 4.1 : Adaptation de l'affichage** ✔️
    -   Dans `VisualizeView.vue`, la commande `get_events` (modifiée en Phase 1) fournit déjà les `RangeEvent` complets et hydratés.
    -   La création des popups Mapbox a été modifiée pour utiliser une fonction `createMessageSVG`.
    -   Cette fonction `createMessageSVG` génère un SVG dynamique basé sur le modèle suivant, en utilisant les données du message hydraté (texte, couleurs) pour ajuster les dimensions et la taille de la police du SVG.

    ```xml
    <svg viewBox="-0 -178 500 325" xmlns="http://www.w3.org/2000/svg">
      <!-- Le fill du rect est lié à event.background_color -->
      <rect fill="#B71C1C" x="-0" y="0" width="500" height="150" rx="20" transform="skewY(-20)" />
      <!-- Le contenu du texte est lié à event.text -->
      <!-- La position y du texte (110) et la taille de la police (100px) sont ajustées dynamiquement -->
      <text text-anchor="start" x="9" y="110" transform="skewY(-20)">
        photo
      </text>

      <style>
        <![CDATA[
        text {
          font: bold 100px Bahnschrift, Semi-Bold Semi-Condensed; /* font-size et font-family dynamiques */
          fill: white; /* fill est lié à event.text_color */
        }
        ]]>
      </style>
    </svg>
    ```

-   **🧪 Tests de validation (Phase 4) :** ✔️
    1.  La visualisation d'un circuit contenant des événements de message a été lancée.
    2.  Il a été vérifié que les messages s'affichent au bon moment et au bon endroit.
    3.  Il a été vérifié qu'ils ont bien la nouvelle forme SVG.
    4.  Un message avec une surcharge de texte et/ou de taille a été testé et il a été vérifié qu'elle est bien appliquée.

### Phase 5 : Finalisation 🧹 ✅ Complétée

**Objectif :** Le code a été nettoyé et il a été assuré que l'évolution est stable.

-   **Étape 5.1 : Nettoyage du code** ✔️
    -   L'ancien code de gestion des messages (propriétés, fonctions) qui n'était plus utilisé a été supprimé.
    -   Les anciens paramètres de `settingsDefault.json` liés au style des messages, car le style est maintenant géré dans la bibliothèque, ont été supprimés.
    -   La route temporaire `/test-messages` de `src/router/index.js` a été supprimée.
    -   Le fichier `src/views/TestMessageLibraryView.vue` a été supprimé.
-   **Étape 5.2 : Tests de régression** ✔️
    -   Toutes les fonctionnalités liées aux messages (ajout, suppression, édition, visualisation) ont été testées de nouveau pour s'assurer que rien n'a été cassé.
    -   Les autres types d'événements (Pause, Flyto) ont été testés pour s'assurer qu'ils n'ont pas été affectés.