# ⏱️ Paramètre : Timer Mapbox (ms)

Ce document détaille le paramètre `Timer Mapbox`, qui contrôle le délai entre les requêtes envoyées aux **API de Mapbox**.

---

## 🎯 Rôle du Paramètre

Le `Timer Mapbox` définit le temps d'attente minimum, en millisecondes, que l'application respecte entre chaque appel à une API Mapbox, notamment pour le géocodage inversé (trouver un nom de commune à partir de coordonnées).

-   **Type**: Entier
-   **Valeur par défaut**: 200 ms
-   **Plage recommandée**: 50 ms - 2000 ms

## ⚖️ Justification : Pourquoi ce délai est-il important ?

Bien que les limites de Mapbox soient généralement plus élevées que celles des services publics, l'utilisation d'un timer reste une pratique essentielle pour la **stabilité** et la **maîtrise des coûts**.

### 1. 🚦 Respecter les Limites de Débit (Rate Limiting)

L'API de géocodage de Mapbox a une limite par défaut de **1000 requêtes par minute**. Cela équivaut à environ 16-17 requêtes par seconde.

-   Une valeur de **200 ms** (valeur par défaut) correspond à `1000 ms / 200 ms = 5` requêtes par seconde, ce qui est très largement en dessous de la limite et garantit de ne jamais être bloqué.
-   Ce délai prévient les pics de charge ("bursts") qui pourraient brièvement dépasser la limite si de nombreux points sont traités simultanément.

### 2. 💰 Maîtriser les Coûts

Les services Mapbox fonctionnent sur un modèle commercial. Au-delà du quota gratuit, chaque requête a un coût.

-   Un timer, en lissant les requêtes dans le temps, permet d'avoir une vision plus claire et prévisible du nombre d'appels API effectués.
-   Il agit comme un **ralentisseur de sécurité** qui empêche un processus de fond de consommer une quantité excessive de requêtes payantes en un temps très court, ce qui pourrait conduire à une facture inattendue.

### 3. ⚙️ Stabilité de l'Application

Comme pour les autres services, forcer un délai entre les requêtes empêche l'application de saturer la connexion réseau ou les ressources système, garantissant une expérience utilisateur fluide même pendant les tâches de fond.

---

## ⚠️ Recommandations

-   **Garder une valeur raisonnable** : La valeur par défaut de 200 ms est un bon compromis entre vitesse et sécurité.
-   **Adapter selon l'usage** : Si vous n'utilisez que très rarement la mise à jour via Mapbox, ce paramètre a moins d'impact. Cependant, si vous l'activez pour de grands circuits, le conserver est une bonne pratique pour les raisons évoquées ci-dessus.
