# 🏁 Paramètre : Afficher le départ

Ce document détaille le paramètre `Afficher le départ`, qui permet d'activer l'ajout automatique d'un message au début du circuit.

---

## 🎯 Rôle du Paramètre

Ce paramètre binaire détermine si un événement de type "message" doit être généré automatiquement au tout premier point du tracé (km 0) lors de l'importation d'un fichier GPX.

-   **Type**: Booléen
-   **Valeur par défaut**: `true` (Activé)

---

## 💡 Utilisation

L'objectif de ce paramètre est d'automatiser la mise en forme des circuits.
- **Activé** : Un message (défini par le paramètre `Message départ`) est inseré dès le début de la vidéo.
- **Désactivé** : Aucun message n'est ajouté au départ.

Cette automatisation est particulièrement utile pour traiter des lots de fichiers GPX sans avoir à ajouter manuellement les messages d'introduction pour chaque circuit.
