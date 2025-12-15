# ⏱️ Paramètre : Timer IGN (ms)

Ce document détaille le paramètre `Timer IGN`, qui contrôle le délai entre les requêtes envoyées à l'API du **Géoportail de l'IGN**.

---

## 🎯 Rôle du Paramètre

Le `Timer IGN` définit le temps d'attente minimum, en millisecondes, que l'application respecte entre chaque appel à l'API de l'IGN lors de la mise à jour des noms de communes.

-   **Type**: Entier
-   **Valeur par défaut**: 200 ms
-   **Plage recommandée**: 50 ms - 2000 ms

## ⚖️ Justification : Pourquoi ce délai est-il crucial ?

L'utilisation de ce timer est une pratique de **programmation défensive** et de **respect des infrastructures** du service public.

### 1. 🚦 Respect des Limites de Débit (Rate Limiting)

L'IGN, comme tout fournisseur de services en ligne, met en place des limites pour garantir la stabilité de sa plateforme. Pour les services de type "autocomplétion" ou "géocodage", la limite est d'environ **10 requêtes par seconde**.

-   Une valeur de **200 ms** (valeur par défaut) correspond à `1000 ms / 200 ms = 5` requêtes par seconde. Cette configuration est **sûre et conservatrice**, laissant une marge de sécurité importante pour éviter d'atteindre la limite.
-   Descendre à **100 ms** (`10 req/s`) est techniquement possible, mais vous rapproche de la limite stricte.

### 2. 🚫 Éviter le Blocage Temporaire

Si un utilisateur dépasse la limite de requêtes, l'IGN répond avec une erreur `HTTP 429 (Too Many Requests)` et **bloque l'adresse IP de l'utilisateur pendant 5 secondes**. Pendant ce temps, aucune requête à l'IGN ne peut aboutir, ce qui interrompt le processus de mise à jour.

Le timer est donc un **garde-fou proactif** pour ne jamais atteindre ce seuil de blocage.

### 3. ⚙️ Stabilité de l'Application

Forcer un délai entre les requêtes empêche l'application d'envoyer des centaines de requêtes en une seule rafale, ce qui pourrait :
-   Saturer la connexion réseau de l'utilisateur.
-   Consommer excessivement les ressources (CPU, mémoire) de l'application, la rendant instable.

---

## ⚠️ Recommandations

-   **Ne pas descendre en dessous de 100 ms** : Un délai inférieur vous expose à un risque élevé de blocage temporaire par l'IGN.
-   **Augmenter le délai si nécessaire** : Si vous rencontrez des erreurs réseau ou si vous utilisez une connexion internet lente, augmenter cette valeur (ex: 300 ms ou plus) peut rendre le processus de mise à jour plus fiable, bien que plus lent.
