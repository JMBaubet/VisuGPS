# La Barre d'Outils

Située tout en haut de l'écran, la barre d'outils regroupe les indicateurs d'état et les actions globales de l'application.

[< Retour à l'accueil](./index.md)

## 1. Indicateurs d'État (Gauche)

*   **Statut des Services** : Une icône vous informe de l'état des connexions (Internet, IGN, Mapbox).
    *   ![connected](https://api.iconify.design/mdi/web-check.svg?color=green&width=30) **Connecté** : Tous les services sont opérationnels.
    *   ![disconnected](https://api.iconify.design/mdi/web-off.svg?color=red&width=30) **Hors ligne** : Aucune connexion internet.
    *   ![mapbox](https://api.iconify.design/mdi/mapbox.svg?color=blue&width=30) **Mapbox injoignable** : Problème spécifique avec Mapbox.
    *   ![key](https://api.iconify.design/mdi/key-alert.svg?color=red&width=30) **Erreur Clé** : Le token Mapbox est invalide.

*   **Télécommande** :
    *   ![remote-off](https://api.iconify.design/mdi/remote-off.svg?color=blue&width=30) **Déconnectée** : Cliquez pour afficher le QR Code de connexion.
    *   ![remote-on](https://api.iconify.design/mdi/remote.svg?color=green&width=30) **Connectée** : Un appareil pilote l'application. Cliquez pour déconnecter.

*   **Mise à jour Communes** :
    *   ![city](https://api.iconify.design/mdi/city-variant.svg?color=green&width=30) Apparaît lors de la récupération des infos (altitude, code postal) pour les circuits. *(Voir [Mise à jour des Communes](./maj_communes.md))*

## 2. Mode de Fonctionnement (Centre)

Au centre de la barre d'outils, un badge peut s'afficher pour indiquer l'environnement de l'application :

*   **Pas de badge** : Mode **Production (PROD)**. C'est le mode standard pour l'utilisation quotidienne.
*   **<span style="color: #2979FF">EVAL</span>** (Bleu) : Mode **Évaluation**. Utilisé pour tester de nouvelles fonctionnalités sans affecter les données principales. **Un cadre bleu entoure l'application.**
*   **<span style="color: #FF9100">TEST</span>** (Orange) : Mode **Test**. Environnement de développement ou de test technique. **Un cadre orange entoure l'application.**

Pour en savoir plus sur la gestion de ces environnements, consultez la page dédiée. *(Voir [Modes de Fonctionnement](./modes_fonctionnement.md))*

## 3. Actions Globales (Droite)

*   **Importer** ![import](https://api.iconify.design/mdi/file-import-outline.svg?width=30) : Ouvre l'assistant d'importation de fichier GPX. *(Voir [Importation](./upload.md))*
*   **Paramètres** ![settings](https://api.iconify.design/mdi/cog.svg?width=30) : Accède au menu de configuration de l'application. *(Voir [Paramètres](./parametres.md))*
*   **Documentation** ![help](https://api.iconify.design/mdi/book-open-outline.svg?width=30) : Affiche ce manuel utilisateur directement dans l'application.

---
[< Retour à l'accueil](./index.md) | [Suivant : Filtres et Tris >](./filtres_tris.md)
