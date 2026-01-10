# Gestionnaire Météo

Le **Gestionnaire Météo** permet de configurer les conditions météorologiques pour votre circuit. Il est accessible via l'icône <img src="https://api.iconify.design/mdi/sun-thermometer.svg?color=black&width=20" style="vertical-align: middle;"> depuis la liste des circuits.

[< Retour aux détails du circuit](./circuit_details.md)

---

## 📅 Date de départ

La première étape consiste à définir la date de votre sortie.
-   Cette date servira de base pour récupérer les archives météo. On ne peut récupérer que les données météo des 14 prochains jours.
-   **Important** : Le système vérifie automatiquement si des données météo sont disponibles pour cette date. Si c'est le cas, elles sont affichées, avec le temps passé depuis la date de récupération.

## 👥 Scénarios (Groupes)

VisuGPS permet de définir plusieurs groupes de cyclistes évoluant à des vitesses différentes ou partant à des heures différentes.

### Ajouter un groupe
Cliquez sur le bouton **"+"** pour ajouter un nouveau groupe.
Chaque groupe possède :
1.  **Nom** : Par défaut "Gr. 1", "Gr. 2"...
2.  **Heure de départ** : L'heure à laquelle ce groupe commence le parcours. 
La précision est de 5 minutes.
3.  **Vitesse Moyenne** : La vitesse estimée pour ce groupe. 
La précision est de 0,5 km/h.

### Supprimer un groupe
Utilisez l'icône **Corbeille** <img src="https://api.iconify.design/mdi/delete.svg?color=red&width=16" style="vertical-align: middle;"> pour retirer un groupe.
*Note : Il doit toujours rester au moins un groupe.*

## ⭐ Groupe de Référence

Parmi vos groupes, l'un d'eux doit être désigné comme **Référence**.
-   C'est ce groupe qui sera utilisé pour **piloter la boussole** (vent, orientation) et l'affichage principal des widgets lors de la visualisation.
-   Pour définir le groupe de référence, cochez le bouton radio <img src="https://api.iconify.design/mdi/radiobox-marked.svg?color=blue&width=16" style="vertical-align: middle;"> correspondant.
-   Le groupe de référence est mis en évidence dans l'interface de visualisation.

## 💾 Sauvegarde

-   **Mettre à jour** : Enregistre votre configuration (Date + Groupes) dans le fichier du circuit.
-   **Télécharger Météo** : Cette action force la récupération des données météo depuis le serveur pour la date choisie.
-   **Visualiser Météo** : Ouvre un tableau détaillé des prévisions météo pour chaque groupe tout au long du parcours. Cette option n'est disponible que si les données météo sont présentes.

---

[< Retour aux détails du circuit](./circuit_details.md)
