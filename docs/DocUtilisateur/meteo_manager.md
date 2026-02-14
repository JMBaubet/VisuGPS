# Gestionnaire Météo

Le **Gestionnaire Météo** est l'outil central pour configurer les conditions de votre sortie et gérer les différents groupes de participants. Il est accessible via l'icône <img src="https://api.iconify.design/mdi/sun-thermometer.svg?color=black&width=20" style="vertical-align: middle;"> depuis la liste des circuits.

[< Retour aux détails du circuit](./circuit_details.md)

---

##  Gestion des Groupes (Scénarios)

VisuGPS permet de définir plusieurs groupes évoluant à des vitesses différentes, partant à des heures décalées, ou empruntant des **Variantes** de parcours différentes.

### Paramétrage des groupes
Pour chaque groupe, vous pouvez définir :
1.  **Nom** : Attribué automatiquement par le système (ex: "Gr. 1", "Gr. 2"). Les noms ne sont pas modifiables individuellement.
2.  **Heure de départ** : Réglable par pas de 5 minutes.
3.  **Vitesse Moyenne** : Vitesse estimée pour le calcul de position (pas de 0,5 km/h).
4.  **Variante associée** : Si des variantes ont été créées pour ce circuit, vous pouvez assigner chaque groupe à une trace spécifique (Trace Principale ou une Variante).
    - *Note : L'interface filtre automatiquement l'affichage pour ne montrer que les groupes concernés par la trace actuellement sélectionnée.*

### Actions sur les groupes
-   **Ajouter** : Cliquez sur le bouton **"+"** pour créer un nouveau scénario. Les nouveaux groupes héritent des réglages de vitesse et d'heure du groupe précédent.
-   **Supprimer** : Utilisez l'icône **Corbeille** <img src="https://api.iconify.design/mdi/delete.svg?color=red&width=16" style="vertical-align: middle;">. Seul le dernier groupe créé peut être supprimé.
-   **Référence** : Cochez le bouton radio <img src="https://api.iconify.design/mdi/radiobox-marked.svg?color=blue&width=16" style="vertical-align: middle;"> pour désigner le groupe de référence. 
    - Ce groupe pilotera la **boussole** (vent) lors de la visualisation.

---

## ☁️ Gestion des Données Météo

La récupération de la météo est liée à la date de départ et aux traces empruntées par vos groupes.

### Sélection de la Date
Le sélecteur de date propose les 14 prochains jours. Un code couleur vous aide à identifier la pertinence des prévisions :
-   🟢 **Vert** : Date du lendemain (prévisions généralement très fiables).
-   ⚪ **Blanc** : Date du jour ou futur.
-   🟠 **Orange** : Date passée (chargement des archives météo).

### État de Mise à Jour
Le bouton de téléchargement change de couleur pour indiquer l'état global de vos données :
-   🔴 **Rouge** : Aucune donnée météo n'est présente pour le circuit.
-   🟠 **Orange Foncé** : Données manquantes pour au moins une des traces utilisées par vos groupes.
-   🟡 **Orange Clair** : Données présentes mais datant de plus de 12h.
-   🔵 **Bleu** : Données présentes mais datant de plus de 4h.
-   🟢 **Vert** : Toutes les données sont à jour (moins de 4h).

### Téléchargement et Mise à jour
En cliquant sur le bouton de mise à jour, un menu s'ouvre :
-   **Tout mettre à jour** : Lance la récupération pour toutes les traces configurées.
-   **Liste des traces** : Permet de voir le statut individuel (🟢 ou 🔴) et de mettre à jour une seule trace spécifiquement.

### Visualisation par Trace
Si vous utilisez plusieurs traces (Principale + Variantes), le bouton **"Voir..."** permet de choisir quelle vue 3D vous souhaitez consulter. 
Un message de statut vous indique en permanence la date de la mise à jour la plus ancienne parmi toutes vos données actives.

---

## 📊 Tableau de bord Météo (Visualisation Statique)

En cliquant sur **"Voir"**, vous accédez à un tableau dynamique qui projette la météo sur toute la longueur du parcours :
-   Le tableau affiche les conditions (icône, température, vent, pluie) à chaque point kilométrique.
-   Vous pouvez modifier l'heure de départ ou la vitesse **directement depuis ce tableau** pour voir l'impact immédiat sur les conditions rencontrées.
-   **Raccourcis clavier** : Dans les champs Heure du tableau, utilisez les flèches **Haut/Bas** pour ajuster par pas de 5 minutes.

---

## 🛠️ Paramètres Associés

Certains comportements du Gestionnaire Météo dépendent de vos réglages globaux :

- **Heure de départ par défaut** : Définit l'heure initiale du premier groupe lors de la première ouverture.
- **Vitesse moyenne par défaut** : Définit la vitesse initiale lors de la création d'un circuit.
- **Plage horaire (Début/Fin journée)** : Définit les limites pour le téléchargement des données horaires.

Retrouvez ces réglages dans :
* [5.8. Météo](./parametres.md#58-météo)

---

[< Retour aux détails du circuit](./circuit_details.md)
