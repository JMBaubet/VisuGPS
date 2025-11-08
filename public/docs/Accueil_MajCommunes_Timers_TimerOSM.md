# ⏱️ Paramètre : Timer OpenStreetMap (ms)

Ce document détaille le paramètre `Timer OpenStreetMap`, qui contrôle le délai entre les requêtes envoyées à l'API **OpenStreetMap**, via son service de géocodage **Nominatim**.

---

## 🎯 Rôle du Paramètre

Le `Timer OSM` définit le temps d'attente minimum, en millisecondes, que l'application respecte entre chaque appel à l'API Nominatim pour la mise à jour des noms de communes.

-   **Type**: Entier
-   **Valeur par défaut**: 1000 ms
-   **Plage recommandée**: 1000 ms - 5000 ms

## ⚖️ Justification : Le Respect d'un Service Communautaire

L'utilisation de ce timer est **absolument critique** pour OpenStreetMap. Contrairement aux services commerciaux ou gouvernementaux, OSM est un projet **communautaire et gratuit**, maintenu par des bénévoles et financé par des dons. Ses ressources sont limitées et partagées par des milliers d'applications dans le monde.

### 1. 🚦 Respect de la Limite de Débit la Plus Stricte

La politique d'utilisation de Nominatim est très claire et stricte :
-   **Limite absolue : 1 requête par seconde.**

-   Une valeur de **1000 ms** (valeur par défaut) correspond exactement à cette limite (`1000 ms / 1000 ms = 1 req/s`). C'est le minimum vital pour utiliser le service.
-   Tenter de descendre en dessous de cette valeur est une violation directe des conditions d'utilisation.

### 2. 🚫 Éviter le Bannissement d'IP

Le non-respect de la limite de 1 req/s est la cause la plus fréquente de **bannissement d'adresse IP** par les administrateurs d'OSM. Un bannissement peut être long, voire permanent, et rendrait la fonctionnalité de mise à jour via OSM totalement inutilisable pour l'utilisateur concerné.

Le timer est donc un **mécanisme de conformité obligatoire** pour pouvoir utiliser ce service.

### 3. 🤝 Être un Bon Citoyen Numérique

Utiliser l'API OSM de manière responsable, c'est permettre à des milliers d'autres projets (associatifs, éducatifs, non-commerciaux) de pouvoir également en bénéficier. Ralentir volontairement nos requêtes est un acte de **solidarité** envers la communauté OpenStreetMap.

---

## ⚠️ Recommandations

-   🛑 **NE JAMAIS DESCENDRE EN DESSOUS DE 1000 ms**. C'est la règle la plus importante. Le faire est une garantie quasi certaine de se faire bloquer.
-   **Augmenter la valeur est encouragé** : Si la vitesse de mise à jour n'est pas votre priorité absolue, augmenter ce délai (ex: 1500 ms, 2000 ms) est un excellent moyen de réduire encore plus la charge sur les serveurs communautaires d'OSM.
