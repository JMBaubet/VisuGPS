# 🌐 Paramètre : Activer l'API IGN par défaut

Ce document détaille le paramètre `ignActif`, qui contrôle l'activation par défaut de l'API du **Géoportail de l'IGN** pour la mise à jour des informations des communes.

---

## 🎯 Rôle du Paramètre

Le paramètre `ignActif` est un interrupteur (booléen) qui détermine si l'application doit utiliser l'API de l'IGN pour enrichir les données des communes.

-   **Libellé**: Activer l'API IGN par défaut
-   **Type**: Booléen
-   **Valeur par défaut**: `true`

## ⚖️ Justification : Pourquoi activer/désactiver l'API IGN ?

Ce paramètre offre à l'utilisateur la flexibilité de choisir les sources de données pour la mise à jour des communes.

### 1. 🚀 Performance et Rapidité

L'API IGN est généralement très performante et offre des données précises pour le territoire français. L'activer permet d'obtenir des résultats rapidement.

### 2. 📊 Qualité et Précision des Données

L'IGN est l'organisme de référence pour l'information géographique en France. L'utilisation de son API garantit une haute qualité et précision des données de communes.

### 3. 🔄 Flexibilité au Lancement de la Mise à Jour

Cet interrupteur peut être activé ou désactivé **dynamiquement** au moment de lancer le processus de mise à jour des communes, offrant un contrôle granulaire sur les sources utilisées pour chaque opération.

### 4. 🚫 Gestion des Problèmes ou des Quotas

Dans de rares cas, vous pourriez souhaiter désactiver l'API IGN :
-   Si vous rencontrez des problèmes de connexion spécifiques avec les services de l'IGN.
-   Si vous avez atteint des quotas d'utilisation (bien que les limites de l'IGN soient généreuses pour un usage normal).
-   Si vous préférez utiliser une autre source de données pour des raisons spécifiques.

---

## ⚠️ Recommandations

-   **Activé par défaut** : Il est recommandé de laisser ce paramètre activé par défaut pour bénéficier de la qualité et de la performance des données IGN.
-   **Désactiver pour les traces hors de France 🇫🇷** : Si votre trace GPS se situe en dehors du territoire français, il est fortement recommandé de désactiver l'API IGN avant de lancer la mise à jour. Cela évitera des requêtes inutiles aux serveurs de l'IGN et améliorera les performances globales du processus.
-   **Désactiver en cas de besoin** : Si vous rencontrez des problèmes ou si vous souhaitez limiter les requêtes à l'IGN, vous pouvez désactiver ce paramètre.