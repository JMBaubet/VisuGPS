# La Barre d'Outils

Située tout en haut de l'écran, la barre d'outils regroupe les indicateurs d'état et les actions globales de l'application.

[< Retour à l'accueil](./index.md)

## 1. Indicateurs d'État (Gauche)

*   **Statut des Services** : Une icône vous informe de l'état des connexions (Internet, IGN, Mapbox).
    *   <img src="https://api.iconify.design/mdi/web-check.svg?color=green&width=20" style="vertical-align: middle; margin-bottom: 3px;"> **Connecté** : Tous les services sont opérationnels.
    *   <img src="https://api.iconify.design/mdi/web-off.svg?color=red&width=20" style="vertical-align: middle; margin-bottom: 3px;"> **Hors ligne** : Aucune connexion internet.
    *   <img src="https://api.iconify.design/mdi/mapbox.svg?color=blue&width=20" style="vertical-align: middle; margin-bottom: 3px;"> **Mapbox injoignable** : Problème spécifique avec Mapbox.
    *   <img src="https://api.iconify.design/mdi/key-alert.svg?color=red&width=20" style="vertical-align: middle; margin-bottom: 3px;"> **Erreur Clé** : Le token Mapbox est invalide.

*   **Télécommande** :
    *   <img src="https://api.iconify.design/mdi/remote-off.svg?color=blue&width=20" style="vertical-align: middle; margin-bottom: 3px;"> **Déconnectée** : Cliquez pour afficher le QR Code de connexion.
    *   <img src="https://api.iconify.design/mdi/remote.svg?color=green&width=20" style="vertical-align: middle; margin-bottom: 3px;"> **Connectée** : Un appareil pilote l'application. Cliquez pour déconnecter.

*   **Mise à jour Communes** :
    *   <img src="https://api.iconify.design/mdi/city-variant.svg?color=green&width=20" style="vertical-align: middle; margin-bottom: 3px;"> Apparaît lors de la récupération des infos (altitude, code postal) pour les circuits. *(Voir [Mise à jour des Communes](./maj_communes.md))*

## 2. Mode de Fonctionnement (Centre)

Au centre de la barre d'outils, un badge peut s'afficher pour indiquer l'environnement de l'application :

*   **Pas de badge** : Mode **Production (PROD)**. C'est le mode standard pour l'utilisation quotidienne.
*   **<span style="background-color: rgba(41, 121, 255, 0.1); color: #2979FF; padding: 2px 8px; border-radius: 4px;">EVAL</span>** (Bleu) : Mode **Évaluation**. Utilisé pour tester de nouvelles fonctionnalités sans affecter les données principales. **Un cadre bleu entoure l'application.**
*   **<span style="background-color: rgba(255, 145, 0, 0.1); color: #FF9100; padding: 2px 8px; border-radius: 4px;">TEST</span>** (Orange) : Mode **Test**. Environnement de développement ou de test technique. **Un cadre orange entoure l'application.**

Pour en savoir plus sur la gestion de ces environnements, consultez la page dédiée. *(Voir [Modes de Fonctionnement](./modes_fonctionnement.md))*

## 3. Actions Globales (Droite)

*   **Importer** <img src="https://api.iconify.design/mdi/file-import-outline.svg?width=20" style="vertical-align: middle; margin-bottom: 3px;"> : Ouvre l'assistant d'importation de fichier GPX. *(Voir [Importation](./upload.md))*
*   **Paramètres** <img src="https://api.iconify.design/mdi/cog.svg?width=20" style="vertical-align: middle; margin-bottom: 3px;"> : Accède au menu de configuration de l'application. *(Voir [<span style="color: #FFC107">Paramètres</span>](./parametres.md))*
*   **Documentation** <img src="https://api.iconify.design/mdi/book-open-outline.svg?width=20&color=%232196F3" style="vertical-align: middle; margin-bottom: 3px;"> : Affiche ce manuel utilisateur directement dans l'application.

---
[< Retour à l'accueil](./index.md) | [Suivant : Filtres et Tris >](./filtres_tris.md)
