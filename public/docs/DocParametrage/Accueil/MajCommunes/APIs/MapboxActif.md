# 🗺️ Paramètre : Activer l'API Mapbox par défaut

Ce document détaille le paramètre `mapboxActif`, qui contrôle l'activation par défaut de l'API **Mapbox** pour la mise à jour des informations des communes.

---

## 🎯 Rôle du Paramètre

Le paramètre `mapboxActif` est un interrupteur (booléen) qui détermine si l'application doit utiliser l'API Mapbox pour enrichir les données des communes.

-   **Libellé**: Activer l'API Mapbox par défaut
-   **Type**: Booléen
-   **Valeur par défaut**: `false`
-   **Critique**: `true` (indique que ce paramètre est important et peut avoir des conséquences significatives sur le comportement de l'application).

## ⚖️ Justification : Pourquoi activer/désactiver l'API Mapbox ?

Ce paramètre est marqué comme "critique" car l'utilisation de l'API Mapbox a des implications importantes, notamment en termes de coûts et de configuration.

### 1. 💰 Implications Financières

Mapbox est un service commercial. Bien qu'il offre un niveau d'utilisation gratuit, toute utilisation au-delà de ce quota entraîne des coûts.

-   **Désactivé par défaut** : Le paramètre est désactivé par défaut pour éviter toute consommation involontaire de requêtes payantes.
-   **Maîtrise des coûts** : L'utilisateur doit explicitement activer ce paramètre, ce qui l'incite à être conscient des potentielles implications financières.

### 2. 🔑 Nécessite un Token API Valide

Pour utiliser l'API Mapbox, un **token API valide** doit être configuré dans les paramètres de l'application (`Système/Tokens/mapbox`).

-   Si `mapboxActif` est activé mais qu'aucun token valide n'est fourni, les requêtes échoueront.
-   La désactivation par défaut évite des messages d'erreur inutiles si l'utilisateur n'a pas configuré de token.

### 3. 🌍 Couverture Géographique et Qualité des Données

Mapbox offre une excellente couverture mondiale et des données de haute qualité.

-   Si vous travaillez avec des traces GPX en dehors de la France ou si vous préférez la qualité des données Mapbox, l'activation de ce paramètre est pertinente.

### 4. 🔄 Flexibilité au Lancement de la Mise à Jour

Cet interrupteur peut être activé ou désactivé **dynamiquement** au moment de lancer le processus de mise à jour des communes, offrant un contrôle granulaire sur les sources utilisées pour chaque opération.

---

## ⚠️ Recommandations

-   **Activer avec prudence** : N'activez ce paramètre que si vous avez un besoin spécifique de l'API Mapbox et que vous avez configuré un token API valide.
-   **Surveiller l'utilisation** : Si vous activez Mapbox, il est conseillé de surveiller votre consommation d'API sur le tableau de bord Mapbox pour gérer les coûts.