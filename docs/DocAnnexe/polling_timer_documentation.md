# Le Timer dans le Polling Réseau

> [!IMPORTANT]
>
>
> **PARAMÈTRE CRITIQUE** : Un intervalle trop court peut surcharger votre processeur et être interprété comme une activité abusive par les services tiers. À l'inverse, un intervalle trop long rendra la télécommande peu réactive.

---

## 🖥️ Qu'est-ce que le "Polling" ?

Le *polling* (ou "vérification à intervalle régulier") est une technique où une application vérifie l'état d'un service réseau de manière répétée à une fréquence définie.

C'est un peu comme demander "Est-ce qu'on est arrivés ?" toutes les 5 minutes lors d'un long trajet en voiture. L'application contacte périodiquement un serveur pour obtenir des informations à jour.

Dans VisuGPS, nous l'utilisons par exemple pour vérifier :
- Si la connexion Internet est active.
- Si les serveurs de Mapbox sont accessibles.

## ⏱️ Rôle du Timer (Intervalle de Polling)

Le **timer**, ou l'intervalle, est simplement le temps d'attente entre chaque vérification. Il détermine la fréquence à laquelle l'application va "sonder" le service réseau.

- Il est généralement exprimé en **millisecondes (ms)**.
- Par exemple, un timer de `30000 ms` signifie que l'application effectuera une vérification toutes les 30 secondes.

## ⚖️ Trouver le Bon Équilibre

Le choix de la durée du timer est un compromis crucial entre la réactivité de l'application et l'utilisation des ressources.

1. **Intervalle Court (ex: 5 000 ms)**
   
   - **Avantages** : L'application détecte très vite les changements d'état (ex: une connexion retrouvée). L'information est toujours "fraîche".
   - **Inconvénients** : Augmente la charge sur le réseau, sur le serveur distant, et consomme plus de ressources sur l'appareil de l'utilisateur (CPU, batterie).

2. **Intervalle Long (ex: 60 000 ms)**

   - **Avantages** : Très économe. Réduit le trafic réseau et la charge sur les serveurs.
   - **Inconvénients** : Il peut y avoir une latence importante entre le moment où un état change (ex: perte de connexion) et le moment où l'application s'en rend compte.

---

## ✅ Bonnes pratiques

- **Adapter le timer au besoin** : Une vérification de statut de connexion n'a pas besoin d'être aussi fréquente qu'une mise à jour de position GPS en temps réel.
- **Respecter les services tiers** : Pour des services comme Mapbox, un polling trop agressif pourrait être considéré comme un abus et potentiellement mener à un blocage temporaire.
- **Rendre le timer configurable** : Permettre à l'utilisateur d'ajuster cette valeur dans les paramètres (comme nous le faisons) est une excellente approche pour s'adapter à différents besoins et conditions réseau.
