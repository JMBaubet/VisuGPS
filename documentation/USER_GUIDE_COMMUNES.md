# Documentation Utilisateur : Mise à jour des Communes

Cette documentation explique comment gérer la mise à jour des informations de communes associées à vos traces GPX dans l'application VisuGPS. Ces informations sont cruciales pour des fonctionnalités futures telles que la description détaillée des circuits ou des analyses géographiques.

## 🚀 Lancement de la mise à jour des communes

La mise à jour des communes se lance depuis la **Vue Principale** de l'application, où vous listez vos traces GPX.

Chaque carte de circuit présente, entre autres icônes, un bouton dédié à la mise à jour des informations de communes.

---
### **Icône "Ville"**

Cette icône représente l'état de la mise à jour des communes pour un circuit donné.

*   **Apparence :**
    *   <img src="https://api.iconify.design/mdi/city.svg?color=%23B71C1C&width=24" alt="Icône Ville Rouge" width="24" height="24" style="vertical-align: -0.25em;"> **Rouge** : Le processus de mise à jour n'a pas commencé ou a très peu progressé (< 7%).
    *   <img src="https://api.iconify.design/mdi/city.svg?color=%23FF6F00&width=24" alt="Icône Ville Orange" width="24" height="24" style="vertical-align: -0.25em;"> **Orange** : La mise à jour est en cours (7% < progression <= 13%).
    *   <img src="https://api.iconify.design/mdi/city.svg?color=%23FFB300&width=24" alt="Icône Ville Ambre" width="24" height="24" style="vertical-align: -0.25em;"> **Ambre** : La mise à jour est en cours (13% < progression <= 25%).
    *   <img src="https://api.iconify.design/mdi/city.svg?color=%23FBC02D&width=24" alt="Icône Ville Jaune" width="24" height="24" style="vertical-align: -0.25em;"> **Jaune** : La mise à jour est en cours (25% < progression <= 50%).
    *   <img src="https://api.iconify.design/mdi/city.svg?color=%23689F38&width=24" alt="Icône Ville Vert Clair" width="24" height="24" style="vertical-align: -0.25em;"> **Vert Clair** : La mise à jour approche de la fin (50% < progression <= 75%).
    *   <img src="https://api.iconify.design/mdi/city.svg?color=%232E7D32&width=24" alt="Icône Ville Vert Foncé" width="24" height="24" style="vertical-align: -0.25em;"> **Vert Foncé** : La mise à jour est presque terminée ou terminée (> 75%).

**Note :** Lorsque la mise à jour des communes atteint 100%, l'icône "Ville" n'est plus affichée pour ce circuit, indiquant que toutes les informations nécessaires ont été collectées.

---

**Pour lancer la mise à jour :**

Cliquez sur l'icône <img src="https://api.iconify.design/mdi/city.svg?color=%23B71C1C&width=24" alt="Icône Ville Rouge" width="24" height="24" style="vertical-align: -0.25em;"> (Icône Ville Rouge, ou toute autre couleur indiquant une progression incomplète) située à côté du circuit concerné.

Une fois cliquée, l'application commencera à récupérer les informations géographiques pour déterminer les communes traversées par la trace. Vous verrez la couleur de l'icône évoluer au fur et à mesure de la progression.

## 📊 Contrôle de l'avancement de la mise à jour

Lorsqu'une mise à jour est en cours, une barre d'information apparaît en haut de la vue principale (ou dans une zone dédiée), vous permettant de suivre et de contrôler le processus.

---
La barre d'information contient les éléments suivants :

1.  **Icône de statut <img src="https://api.iconify.design/mdi/city-variant.svg?color=%234CAF50&width=24" alt="Icône de Statut Vert" width="24" height="24" style="vertical-align: -0.25em;"> :** Confirme visuellement qu'un processus de mise à jour est en cours.
2.  **Nom du circuit :** Affiche le nom du circuit pour lequel la mise à jour est effectuée.
3.  **Barre de progression :**
    *   Indique le pourcentage d'avancement de la mise à jour pour le circuit en cours.
    *   La couleur de la barre évolue de la même manière que l'icône "Ville" : du rouge pour le début, au vert pour la fin. (Note: La barre de progression est un élément visuel dynamique et n'est pas représentée par une icône statique ici.)
4.  **Bascule API IGN & Mapbox :**
    *   Ces interrupteurs (switches) vous permettent d'activer ou de désactiver dynamiquement l'utilisation des APIs de l'IGN et de Mapbox *pendant* la mise à jour.
    *   <img src="https://api.iconify.design/mdi/toggle-switch.svg?color=%234CAF50" alt="Switch Activé Vert" width="48" height="30" style="vertical-align: -0.70em;"> **(activé - vert)** : L'API est utilisée pour la mise à jour.
    *   <img src="https://api.iconify.design/mdi/toggle-switch-off.svg?color=%23F44336" alt="Switch Désactivé Rouge" width="48" height="30" style="vertical-align: -0.70em;"> **(désactivé - rouge)** : L'API n'est pas utilisée.
    *   Vous pouvez, par exemple, désactiver Mapbox si vous ne souhaitez pas consommer de crédits API ou si vous rencontrez des problèmes de connexion avec ce service.
5.  **Bouton Arrêter <img src="https://api.iconify.design/mdi/stop-circle-outline.svg?color=%23F44336&width=24" alt="Bouton Arrêter Rouge" width="24" height="24" style="vertical-align: -0.25em;"> :**
    *   Cliquez sur ce bouton pour interrompre la mise à jour en cours. Les informations déjà récupérées seront conservées.

## ⚙️ Paramétrage de la mise à jour des communes

Vous pouvez configurer le comportement de la mise à jour des communes via les **paramètres de l'application**. Ces réglages sont globaux et affectent toutes les futures mises à jour.

#### **Timers (Délais d'interrogation des services)**

Ces paramètres contrôlent la fréquence à laquelle l'application interroge les APIs externes (IGN, Mapbox, OpenStreetMap) pour récupérer les données de communes.

*   **Chemin dans les paramètres :** `Accueil` > `MajCommunes` > `Timers`
*   **Paramètres disponibles :**
    *   **`timerIGN` (Délai IGN) :**
        *   **Rôle :** Définit le délai en millisecondes entre chaque demande à l'API IGN.
        *   **Impact :** Une valeur plus faible (ex: 50ms) accélère la mise à jour mais peut entraîner le dépassement des limites de requêtes de l'API IGN. Une valeur plus élevée (ex: 2000ms) ralentit le processus mais est plus sûre pour éviter les blocages.
        *   **Défaut :** 200 ms
    *   **`timerMapbox` (Délai Mapbox) :**
        *   **Rôle :** Définit le délai en millisecondes entre chaque demande à l'API Mapbox.
        *   **Impact :** Similaire à `timerIGN`. Notez que Mapbox peut avoir des coûts associés à l'utilisation.
        *   **Défaut :** 200 ms
    *   **`timerOSM` (Délai OpenStreetMap) :**
        *   **Rôle :** Définit le délai en millisecondes entre chaque demande à l'API OpenStreetMap.
        *   **Impact :** Généralement, cette API est moins restrictive.
        *   **Défaut :** 1000 ms

#### **APIs (Activation des services par défaut)**

Ces paramètres gèrent quelles APIs sont activées par défaut pour la mise à jour des communes.

*   **Chemin dans les paramètres :** `Accueil` > `MajCommunes` > `APIs`
*   **Paramètres disponibles :**
    *   **`ignActif` (Activer API IGN) :**
        *   **Rôle :** Active ou désactive l'utilisation de l'API IGN pour les futures mises à jour.
        *   **Défaut :** `Activé`
    *   **`mapboxActif` (Activer API Mapbox) :**
        *   **Rôle :** Active ou désactive l'utilisation de l'API Mapbox pour les futures mises à jour.
        *   **Impact :** Ce paramètre est marqué comme **critique**. Il est important de comprendre les implications de coût ou de limites de débit associées à l'utilisation de l'API Mapbox avant de l'activer.
        *   **Défaut :** `Désactivé`

## 🔄 Comportement au lancement et au changement de vue

*   **Au lancement de l'application :**
    *   L'application vérifie si une mise à jour de communes était en cours lors de sa dernière exécution.
    *   Si c'est le cas, la barre de contrôle de l'avancement réapparaîtra, reflétant le statut et la progression actuelle.
    *   Les réglages par défaut ou personnalisés des APIs (IGN/Mapbox) sont pris en compte.
*   **En naviguant entre les vues :**
    *   La mise à jour de communes, si elle est lancée, continuera en arrière-plan même si vous changez de vue dans l'application.
    *   Si vous revenez à la vue principale, l'état de la mise à jour sera correctement affiché et vous pourrez toujours interagir avec la barre de contrôle.
    *   Les modifications des interrupteurs API (IGN/Mapbox) sont persistantes et affectent le processus de mise à jour en temps réel.
