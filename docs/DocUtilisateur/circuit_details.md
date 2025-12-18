# Les Données d'un Circuit

La liste principale affiche vos parcours sous forme de fiches. Chaque fiche regroupe les informations essentielles et les actions disponibles pour ce circuit.

[< Retour à l'accueil](./index.md) | [< Précédent : Filtres et Tris](./filtres_tris.md)

## Informations Affichées (de gauche à droite)

Une fiche de circuit présente les données dans cet ordre :

1.  **Identification** :
    *   **Nom du circuit** : Titre en gras. ![filter](https://api.iconify.design/mdi/filter-variant.svg?width=16 "Peut être filtré par nom") ![sort](https://api.iconify.design/mdi/sort-alphabetical-ascending.svg?width=16 "Tri alphabétique")
    *   **Ville de départ** : Commune identifiée au départ du tracé. ![filter](https://api.iconify.design/mdi/filter-variant.svg?width=16 "Peut être filtré par ville") ![sort](https://api.iconify.design/mdi/sort-alphabetical-ascending.svg?width=16 "Tri par ville")

2.  **Statistiques Clés** :
    *   **Distance** : Kilométrage total. ![filter](https://api.iconify.design/mdi/filter-variant.svg?width=16 "Peut être filtré par plage de distance") ![sort](https://api.iconify.design/mdi/sort-numeric-ascending.svg?width=16 "Tri par distance")
    *   **Dénivelé** : Dénivelé positif (D+). ![filter](https://api.iconify.design/mdi/filter-variant.svg?width=16 "Peut être filtré par dénivelé") ![sort](https://api.iconify.design/mdi/sort-numeric-ascending.svg?width=16 "Tri par dénivelé")
    *   **Sommet** : Altitude maximale et sa position kilométrique.

3.  **Traceur et Progression** :
    *   **Traceur** : Nom de l'auteur du circuit. ![filter](https://api.iconify.design/mdi/filter-variant.svg?width=16 "Peut être filtré par traceur") ![sort](https://api.iconify.design/mdi/sort-alphabetical-ascending.svg?width=16 "Tri par traceur")
    *   **Barre d'avancement d'édition** : Un bargraphe apparaît si le travail de synchronisation de la caméra est commencé mais non terminé.

4.  **Icônes et Actions rapides** :
    *   **Mise à jour des communes** ![city](https://api.iconify.design/mdi/city.svg?color=orange&width=20 "Mise à jour des communes") : Indique l'état de recherche des noms de communes traversées. (voir [Mise à jour des Communes](./maj_communes.md)).
        > **Note** : Cette icône n'est visible que si la recherche est incomplète. Elle disparaît une fois que 100% des communes ont été identifiées.
    *   **Information et Vignette** ![info](https://api.iconify.design/mdi/information.svg?width=20 "Informations détaillées") : Affiche la **vignette** au survol de la souris et ouvre les détails complets au clic.
    *   **Éditer** ![edit](https://api.iconify.design/mdi/pencil.svg?color=blue&width=20 "Ouvrir l'éditeur") : Ouvre le mode Édition. La couleur de l'icône change selon l'avancement : ![non commencé](https://api.iconify.design/mdi/pencil.svg?color=red&width=16) Rouge (non commencé), ![en cours](https://api.iconify.design/mdi/pencil.svg?color=orange&width=16) Orange/Jaune (en cours) ou ![terminé](https://api.iconify.design/mdi/pencil.svg?color=blue&width=16) Bleu (terminé). (voir [Mode Édition](./edition_intro.md))
    *   **Visualiser** ![visualize](https://api.iconify.design/mdi/eye.svg?color=green&width=20 "Lancer la visualisation 3D") : Lance l'animation 3D. La couleur de l'icône indique l'état de la configuration caméra : ![non commencé](https://api.iconify.design/mdi/eye.svg?color=red&width=16) Rouge (non commencé), ![en cours](https://api.iconify.design/mdi/eye.svg?color=orange&width=16) Orange (en cours) ou ![terminé](https://api.iconify.design/mdi/eye.svg?color=green&width=16) Vert (terminé). (voir [Mode Visualisation](./visualisation.md))
    *   **Supprimer** ![delete](https://api.iconify.design/mdi/delete.svg?color=red&width=20 "Supprimer le circuit") : Supprime définitivement le circuit de votre bibliothèque. (voir [Suppression](./suppression.md))

---
[< Retour à l'accueil](./index.md)
