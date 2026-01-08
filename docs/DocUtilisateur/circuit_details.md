# Les Données d'un Circuit

La liste principale affiche vos parcours sous forme de fiches. Chaque fiche regroupe les informations essentielles et les actions disponibles pour ce circuit.

[< Retour à l'accueil](./index.md)

## Informations Affichées (de gauche à droite)

Une fiche de circuit présente les données dans cet ordre :

1.  **Identification** :
    *   **Nom du circuit** : Titre en gras. <img src="https://api.iconify.design/mdi/filter-variant.svg?width=20" style="vertical-align: middle; margin-bottom: 3px;" title="Peut être filtré par nom"> <img src="https://api.iconify.design/mdi/sort-alphabetical-ascending.svg?width=20" style="vertical-align: middle; margin-bottom: 3px;" title="Tri alphabétique">
    *   **Ville de départ** : Commune identifiée au départ du tracé. <img src="https://api.iconify.design/mdi/filter-variant.svg?width=20" style="vertical-align: middle; margin-bottom: 3px;" title="Peut être filtré par ville"> <img src="https://api.iconify.design/mdi/sort-alphabetical-ascending.svg?width=20" style="vertical-align: middle; margin-bottom: 3px;" title="Tri par ville">

2.  **Statistiques Clés** :
    *   **Distance** : Kilométrage total. <img src="https://api.iconify.design/mdi/filter-variant.svg?width=20" style="vertical-align: middle; margin-bottom: 3px;" title="Peut être filtré par plage de distance"> <img src="https://api.iconify.design/mdi/sort-numeric-ascending.svg?width=20" style="vertical-align: middle; margin-bottom: 3px;" title="Tri par distance">
    *   **Dénivelé** : Dénivelé positif (D+). <img src="https://api.iconify.design/mdi/filter-variant.svg?width=20" style="vertical-align: middle; margin-bottom: 3px;" title="Peut être filtré par dénivelé"> <img src="https://api.iconify.design/mdi/sort-numeric-ascending.svg?width=20" style="vertical-align: middle; margin-bottom: 3px;" title="Tri par dénivelé">
    *   **Sommet** : Altitude maximale et sa position kilométrique.

3.  **Traceur et Progression** :
    *   **Traceur** : Nom de l'auteur du circuit. <img src="https://api.iconify.design/mdi/filter-variant.svg?width=20" style="vertical-align: middle; margin-bottom: 3px;" title="Peut être filtré par traceur"> <img src="https://api.iconify.design/mdi/sort-alphabetical-ascending.svg?width=20" style="vertical-align: middle; margin-bottom: 3px;" title="Tri par traceur">
    *   **Barre d'avancement d'édition** : Un bargraphe apparaît si le travail de synchronisation de la caméra est commencé mais non terminé.

4.  **Icônes et Actions rapides** :
    *   <img src="https://api.iconify.design/mdi/city.svg?color=orange&width=20" style="vertical-align: middle; margin-bottom: 3px;" title="Mise à jour des communes"> **Mise à jour des communes** : Indique l'état de recherche des noms de communes traversées. (voir [Mise à jour des Communes](./maj_communes.md)).
        > [!NOTE]
        >
        > Cette icône est visible tant que la recherche est incomplète. Elle disparaît une fois que 100% des communes ont été identifiées.
    *   <img src="https://api.iconify.design/mdi/information.svg?width=20" style="vertical-align: middle; margin-bottom: 3px;" title="Informations détaillées"> **Information et Vignette** :   
      *Au survol de la souris* : Affiche la    **vignette** au survol de la souris
      *Au clic souris* : Ouvre les **informations** du circuit. (voir [Informations Détaillées](./information_circuit.md))
    *   <img src="https://api.iconify.design/mdi/pencil.svg?color=blue&width=20" style="vertical-align: middle; margin-bottom: 3px;" title="Ouvrir l'éditeur"> **Éditer** : Ouvre le mode Édition. 
    La couleur de l'icône change selon l'avancement : 
    <img src="https://api.iconify.design/mdi/pencil.svg?color=red&width=20" style="vertical-align: middle; margin-bottom: 3px;"> Rouge (non commencé), 
    <img src="https://api.iconify.design/mdi/pencil.svg?color=orange&width=20" style="vertical-align: middle; margin-bottom: 3px;"> Orange/Jaune (en cours) 
    <img src="https://api.iconify.design/mdi/pencil.svg?color=blue&width=20" style="vertical-align: middle; margin-bottom: 3px;"> Bleu (terminé). (voir [Mode Édition](./edition_intro.md))
    *   <img src="https://api.iconify.design/mdi/sun-thermometer.svg?color=green&width=20" style="vertical-align: middle; margin-bottom: 3px;" title="Gestion Météo"> **Météo** : Ouvre le gestionnaire de scénarios météo. (voir [Gestionnaire Météo](./meteo_manager.md))
    La couleur indique la fraîcheur des données :
    <img src="https://api.iconify.design/mdi/sun-thermometer.svg?color=grey&width=20" style="vertical-align: middle; margin-bottom: 3px;"> Gris (Aucun groupe de configuré), 
    <img src="https://api.iconify.design/mdi/sun-thermometer.svg?color=green&width=20" style="vertical-align: middle; margin-bottom: 3px;"> Vert (À jour < 3h),
    <img src="https://api.iconify.design/mdi/sun-thermometer.svg?color=blue&width=20" style="vertical-align: middle; margin-bottom: 3px;"> Bleu (Données en cache),
    <img src="https://api.iconify.design/mdi/sun-thermometer.svg?color=red&width=20" style="vertical-align: middle; margin-bottom: 3px;"> Rouge (Erreur/Pas de cache).
    *   <img src="https://api.iconify.design/mdi/eye.svg?color=green&width=20" style="vertical-align: middle; margin-bottom: 3px;" title="Lancer la visualisation 3D"> **Visualiser** : Lance l'animation 3D. (voir [Mode Visualisation](./visualisation.md))
    La couleur de l'icône indique l'état de la configuration caméra : 
    <img src="https://api.iconify.design/mdi/eye.svg?color=red&width=20" style="vertical-align: middle; margin-bottom: 3px;"> Rouge (non commencé), 
    <img src="https://api.iconify.design/mdi/eye.svg?color=orange&width=20" style="vertical-align: middle; margin-bottom: 3px;"> Orange (en cours),
    <img src="https://api.iconify.design/mdi/eye.svg?color=green&width=20" style="vertical-align: middle; margin-bottom: 3px;"> Vert (terminé).
    *   <img src="https://api.iconify.design/mdi/delete.svg?color=red&width=20" style="vertical-align: middle; margin-bottom: 3px;" title="Supprimer le circuit"> **Supprimer** : Supprime définitivement le circuit de votre bibliothèque. (voir [Suppression](./suppression.md))

---
[< Retour à l'accueil](./index.md)
