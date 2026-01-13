# 🔢 Paramètre : Clé API Routage

Ce document détaille le paramètre `routingApiKey`, qui permet de renseigner la clé d'authentification nécessaire pour utiliser les services de routage externes.

---

## 🎯 Rôle du Paramètre

Le paramètre `routingApiKey` stocke la clé API fournie par le service de routage sélectionné (GraphHopper ou OpenRouteService).

-   **Libellé**: Clé API Routage
-   **Type**: Chaîne de caractères
-   **Valeur par défaut**: (Vide)

## ⚖️ Justification : Pourquoi cette clé ?

La plupart des services de routage performants (comme GraphHopper ou OpenRouteService) nécessitent une identification pour contrôler l'utilisation de leurs serveurs et éviter les abus.

### 1. Authentification
La clé permet au service de vous reconnaître et d'autoriser vos requêtes de calcul d'itinéraire.

### 2. Quotas
Les services gratuits ont souvent des limites d'utilisation (nombre de requêtes par jour/minute). La clé permet de gérer ces quotas.

## 💡 Recommandations

*   **GraphHopper** : Créez un compte sur [graphhopper.com](https://www.graphhopper.com/) pour obtenir une clé API.
*   **OpenRouteService** : Créez un compte sur [openrouteservice.org](https://openrouteservice.org/) pour obtenir une clé API.
*   Si vous utilisez une instance locale (ex: serveur GraphHopper tournant sur votre machine), vous pouvez laisser ce champ vide si aucune authentification n'est requise.
