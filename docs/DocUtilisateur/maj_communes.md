# Mise à jour des Communes

VisuGPS peut enrichir vos traces GPX en récupérant le nom des communes traversées. Cette fonctionnalité permet d'avoir lors de la visualisation 3D, l'affichage des communes traversées.

[< Retour aux détails du circuit](./circuit_details.md)


## 1. Principe de Fonctionnement

Lorsque vous avez importé un fichier GPX, vous pouvez cliquer sur l'icône <img src="https://api.iconify.design/mdi/city-variant.svg?color=red&width=20" style="vertical-align: middle; margin-bottom: 3px;"> du circuit. À partir de ce moment, l'application analyse chaque point de la trace pour déterminer la commune correspondante. 

Le processus se déroule en plusieurs passes pour optimiser la performance et respecter les limites des API externes. On commence par mettre à jour le nom de communes, tous les 1600m, puis tous les 800m, puis tous les 400m, etc.

> [!NOTE]
>
>
> Ce processus peut prendre plusieurs dizaines de minutes en fonction de la longueur de la trace. Il s'exécute intégralement en **tâche de fond**, ce qui signifie que vous pouvez continuer à utiliser l'application normalement sans interférence.

### Indicateur d'Avancement
L'icône <img src="https://api.iconify.design/mdi/city-variant.svg?width=20" style="vertical-align: middle; margin-bottom: 3px;"> change de couleur pour indiquer le taux de complétion :

| Couleur | Avancement | Signification |
| :--- | :--- | :--- |
| **<span style="color: #D32F2F">Rouge</span>** | 0 - 7% | Mise à jour non commencée ou à peine débutée. |
| **<span style="color: #F57C00">Orange</span>** | 7 - 13% | Mise à jour tous les 1600 m réalisée. |
| **<span style="color: #FFA000">Ambre</span>** | 13 - 25% | Mise à jour tous les 800 m réalisée. |
| **<span style="color: #FBC02D">Jaune</span>** | 25 - 50% | Mise à jour tous les 400 m réalisée. |
| **<span style="color: #689F38">Vert Clair</span>** | 50 - 75% |   Mise à jour tous les 200 m réalisée. |
| **<span style="color: #388E3C">Vert Foncé</span>** | > 75% | Mise à jour tous les 100 m en cours de finamisation. |

Une fois à 100%, l'icône disparaît de la ligne du circuit et le bouton n'est plus cliquable.

## 2. Contrôle en Temps Réel

Lorsqu'une mise à jour est lancée, une barre d'état apparaît dans la barre d'outils. 
Si une mise à jour des communes est en cours pour un circuit, il n'est pas possible de relancer une mise à jour pour un autre circuit. 
Si vous avez un circuit plus prioritaire, vous pouvez arrêter la mise à jour en cours en cliquant sur le bouton <img src="https://api.iconify.design/mdi/stop-circle-outline.svg?width=20&color=red" style="vertical-align: middle; margin-bottom: 3px;">.

Si l'application est arrêtée alors qu'une mise à jour des communes est en cours, au démarrage de l'application, la mise à jour des communes sera automatiquement relancée.

### Switches (Interrupteurs)
Cette barre contient deux interrupteurs permettant d'activer ou désactiver les APIs **pendant** le processus :

*   **Switch IGN** : Permet de couper l'interrogation de l'IGN si :
    * votre trace n'est pas en France,
    * Vous observez des lenteurs
*   **Switch Mapbox** : Permet de couper l'interrogation de Mapbox si :
    * Vous observez des lenteurs.

> [!NOTE]
>
>
> Si vous coupez les deux sources, la mise à jour s'appuiera uniquement sur les données OpenStreetMap (plus lentes et moins précises).

VisuGPS utilise plusieurs services en cascade pour garantir le meilleur résultat :

1.  **IGN (Institut National de l'Information Géographique)** : 
    *   À utiliser uniquement pour la France.
    *   Pas toujours disponible.
    *   Très précis pour le découpage administratif français.
2.  **Mapbox** : 
    *   Service mondial, utilisé en complément ou si l'IGN ne répond pas.
    *   Peux être limmité par des quotas d'uti
    *   Nécessite une configuration correcte du Token Mapbox.
3.  **OpenStreetMap (OSM)** : 
    *   Solution de secours universelle.
    *   Peu rapide mais fiable quelque soit le pays.

## 3. Configuration

Vous pouvez ajuster le comportement de la mise à jour via les [Paramètres](./parametres.md) (**Accueil > MajCommunes**).

### Timers (Délais)
Pour éviter de saturer les services (et de se faire bloquer), des délais d'attente sont configurés entre chaque requête :
*   **Timer IGN** : Délai par défaut de 200ms.
*   **Timer Mapbox** : Délai par défaut de 200ms.
*   **Timer OpenStreetMap** : Délai plus long (1000ms) car le service est plus restrictif.

### Activation des APIs
Vous avez la possibilité d'activer ou désactiver globalement certaines APIs :
*   **Activer l'API IGN** : Recommandé pour une utilisation en France.
*   **Activer l'API Mapbox** : Utile pour une couverture internationale.

## 4. Impact sur l'Application

La récupération des noms de communes permet d'enrichir la visualisation 3D en affichant dynamiquement le nom de la commune survolée.

---

### 🛠️ Paramètres Liés
Retrouvez les réglages détaillés associés à cette fonctionnalité dans la section :
* [1.1. MajCommunes](./parametres.md#11-majcommunes)

---
[< Retour aux détails du circuit](./circuit_details.md)
