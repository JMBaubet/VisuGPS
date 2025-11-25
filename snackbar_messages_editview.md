## Analyse des Messages Snackbar dans `src/views/EditView.vue`

Ce document détaille tous les messages envoyés à la snackbar depuis la vue d'édition, le contexte de leur apparition et leur niveau de criticité.

### Navigation et Modes d'Interaction

| Message | Niveau | Action Utilisateur / Contexte |
| :--- | :--- | :--- |
| `Déplacement de la carte désactivé.` | `info` | L'utilisateur sélectionne l'onglet "Caméra" dans le panneau de contrôle, ce qui désactive le déplacement de la carte au profit de l'édition des paramètres de caméra. |
| `Déplacement de la carte activé.` | `info` | L'utilisateur sélectionne un autre onglet que "Caméra" (ex: "Messages", "Stop"), réactivant le déplacement libre de la carte. |

### Gestion des Événements (Pause, Flyto, Message)

| Message | Niveau | Action Utilisateur / Contexte |
| :--- | :--- | :--- |
| `Pause ajoutée avec succès` | `success` | L'utilisateur ajoute un événement "Pause" à un point de la trace et l'opération réussit. |
| `Ajout de la Pause annulé.` | `info` | L'utilisateur tente d'ajouter une pause là où un "Flyto" existe déjà, et choisit de ne pas le remplacer. |
| `Pause supprimée avec succès` | `success` | L'utilisateur supprime un événement "Pause" existant et l'opération réussit. |
| `Événement Survol ajouté avec succès` | `success` | L'utilisateur enregistre les paramètres de caméra actuels comme un événement "Flyto" et l'opération réussit. |
| `Ajout du Survol annulé.` | `info` | L'utilisateur tente d'ajouter un "Flyto" là où une "Pause" existe déjà, et choisit de ne pas la remplacer. |
| `Événement Survol supprimé avec succès` | `success` | L'utilisateur supprime un événement "Flyto" existant et l'opération réussit. |
| `Vérification du Survol...` | `info` | L'utilisateur clique sur le bouton pour prévisualiser un événement "Flyto", déclenchant une animation de la caméra vers le point enregistré. |
| `Aucun événement Survol trouvé à vérifier.` | `warning` | L'utilisateur clique sur le bouton de vérification "Flyto" à un endroit où aucun événement de ce type n'est défini. |
| `Message ajouté avec succès` / `Message mis à jour avec succès` | `success` | L'utilisateur ajoute un nouveau message contextuel à la trace ou modifie un message existant. |
| `Aucun message sélectionné.` | `error` | L'utilisateur tente d'ajouter un événement de message sans avoir préalablement sélectionné un message dans la bibliothèque. |
| `Aucun message à supprimer à ce point.` | `info` | L'utilisateur tente de supprimer un message à un endroit de la trace où aucun n'existe. |
| `Message supprimé avec succès` | `success` | L'utilisateur supprime un événement de message existant et l'opération réussit. |

### Gestion des Points de Contrôle et Zoom

| Message | Niveau | Action Utilisateur / Contexte |
| :--- | :--- | :--- |
| `Point de contrôle enregistré et tracking mis à jour.` | `success` | L'utilisateur enregistre les paramètres de caméra (zoom, pitch, cap) comme un point de contrôle, et la sauvegarde dans le fichier de tracking est réussie. |
| `Point de contrôle supprimé et tracking mis à jour.` | `info` | L'utilisateur supprime un point de contrôle, et la suppression ainsi que le recalcul de l'interpolation sont effectués avec succès. |
| `Distance de tracking mise à jour.` | `success` | Après la sauvegarde d'un point de contrôle qui est le plus éloigné sur la trace, la distance totale de tracking est mise à jour dans le fichier de configuration du circuit. |
| `Zoom de départ appliqué.` / `Zoom d'arrivée appliqué.` | `success` | L'utilisateur active la fonctionnalité de zoom progressif au départ ou à l'arrivée, et les points de la trace sont modifiés avec succès. |
| `Zoom de départ mis à jour.` / `Zoom d'arrivée mis à jour.` | `success` | Les paramètres (valeur, distance) du zoom de départ ou d'arrivée sont modifiés et appliqués avec succès. |
| `Zoom de départ supprimé.` / `Zoom d'arrivée supprimé.` | `info` | L'utilisateur désactive la fonctionnalité de zoom progressif au départ ou à l'arrivée. |
| `Paramètres de zoom mis à jour dans circuits.json.` | `success` | Les réglages de zoom (activé/désactivé, valeur, distance) sont sauvegardés dans le fichier de configuration global du circuit. |

### Erreurs et Avertissements

| Message | Niveau | Action Utilisateur / Contexte |
| :--- | :--- | :--- |
| `ID du circuit manquant pour l'édition.` | `error` | Erreur critique au chargement de la page si l'identifiant du circuit n'est pas fourni dans l'URL. |
| `Token Mapbox non configuré.` | `error` | Erreur critique au chargement si le token d'accès pour Mapbox n'est pas trouvé dans les paramètres. |
| `Données de tracking introuvables ou vides.` | `error` | Erreur critique au chargement si le fichier de tracking associé au circuit est manquant ou vide. |
| `Erreur lors de l'ajout de...` / `Erreur lors de la suppression de...` | `error` | Message générique indiquant qu'une opération de sauvegarde ou de suppression (Pause, Flyto, Message, etc.) a échoué. Le détail de l'erreur est souvent affiché à la suite. |
| `Erreur: {détail}` | `error` | Message d'erreur générique affiché lorsqu'une opération back-end (`invoke`) échoue pour une raison inattendue (ex: sauvegarde de fichier, mise à jour de configuration). |
| `La distance du zoom de départ/d'arrivée est plus grande que la trace.` | `error` | L'utilisateur a configuré une distance pour le zoom progressif qui dépasse la longueur totale de la trace, rendant l'opération impossible. |
| `Impossible de générer les couleurs de pente...` | `warning` | Le calcul pour colorer la trace selon la pente a échoué, mais l'application continue en utilisant une couleur de trace unie par défaut. |
| `Erreur lors de la génération des couleurs de pente...` | `error` | Une erreur inattendue est survenue pendant le calcul des couleurs de pente. |
| `Événement de message manquant supprimé avec succès.` | `success` | Un événement de message faisait référence à un ID de message qui n'existe plus dans la bibliothèque. L'utilisateur a choisi de supprimer cet événement invalide. |
| `Erreur de message manquant ignorée...` | `warning` | L'utilisateur a été notifié qu'un message était manquant mais a choisi d'ignorer l'erreur. L'événement invalide reste dans le fichier mais ne sera pas affiché. |
| `Erreur Mapbox: {détail}` | `error` | Une erreur provenant de l'API Mapbox GL JS est survenue (ex: problème de style, de source de données). |