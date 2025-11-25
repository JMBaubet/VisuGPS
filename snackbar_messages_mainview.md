## Analyse des Messages Snackbar dans `src/views/MainView.vue`

Ce document détaille tous les messages envoyés à la snackbar depuis la vue principale et ses composants associés, le contexte de leur apparition et leur niveau de criticité.

> **Note :** La vue `MainView.vue` elle-même n'utilise pas directement `showSnackbar`. Les messages sont générés par ses composants enfants : `GpxImportDialog.vue` et `CircuitListItem.vue`.

---

## Composant : `GpxImportDialog.vue`

### Import de Fichiers GPX

| Message | Niveau | Action Utilisateur / Contexte |
| :--- | :--- | :--- |
| `Veuillez sélectionner un fichier GPX.` | `warning` | L'utilisateur clique sur le bouton "Importer" dans la boîte de dialogue d'import sans avoir sélectionné de fichier GPX dans la liste. |
| `Circuit '{nom}' importé avec succès.` | `success` | L'utilisateur a sélectionné un fichier GPX, choisi un traceur, et l'import complet (analyse, validation, génération de vignette) s'est terminé avec succès. |
| `Erreur lors de l'import: {détail}` | `error` | Une erreur s'est produite pendant le processus d'import du fichier GPX (analyse du fichier, validation, ou génération de la vignette). Le détail de l'erreur est affiché. Ce message n'apparaît pas si l'utilisateur annule volontairement l'opération. |

---

## Composant : `CircuitListItem.vue`

### Gestion des Circuits

| Message | Niveau | Action Utilisateur / Contexte |
| :--- | :--- | :--- |
| `Circuit supprimé avec succès.` | `success` | L'utilisateur clique sur le bouton de suppression (icône poubelle), confirme la suppression dans la boîte de dialogue de confirmation, et l'opération de suppression du circuit réussit. |
| `Erreur lors de la suppression du circuit : {détail}` | `error` | L'utilisateur tente de supprimer un circuit, mais l'opération échoue côté backend. Le détail de l'erreur est affiché. |

---

## Résumé par Niveau de Criticité

### Messages `success` (2)
- Import de circuit réussi
- Suppression de circuit réussie

### Messages `warning` (1)
- Aucun fichier GPX sélectionné pour l'import

### Messages `error` (2)
- Erreur lors de l'import d'un fichier GPX
- Erreur lors de la suppression d'un circuit

---

## Actions Utilisateur Principales

1. **Import d'un nouveau circuit GPX** :
   - Ouvrir la boîte de dialogue d'import (bouton dans `AppMainBar`)
   - Sélectionner un fichier GPX dans la liste
   - Choisir un traceur
   - Confirmer l'import
   - → Messages possibles : `warning` (pas de sélection), `success` (import réussi), `error` (échec)

2. **Suppression d'un circuit existant** :
   - Cliquer sur l'icône de suppression (poubelle) d'un circuit
   - Confirmer la suppression dans la boîte de dialogue
   - → Messages possibles : `success` (suppression réussie), `error` (échec)

3. **Navigation vers d'autres vues** :
   - Édition (`EditView`) : Aucun message snackbar
   - Visualisation 3D (`VisualizeView`) : Aucun message snackbar
   - Information du circuit : Aucun message snackbar (sauf si mise à jour du circuit)
