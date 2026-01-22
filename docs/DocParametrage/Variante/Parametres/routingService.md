# 🗺️ Paramètre : Service de Routage

Ce document détaille le paramètre `routingService`, utilisé pour le calcul d'itinéraires dans les variantes.

---

## 🎯 Rôle du Paramètre

Le paramètre `routingService` permet de choisir le fournisseur tiers utilisé pour calculer l'itinéraire entre les points que vous définissez.

-   **Libellé**: Service de routage
-   **Type**: Liste de choix
-   **Valeur par défaut**: `OpenRouteService`
-   **Options**: `GraphHopper`, `OpenRouteService`

## ⚖️ Justification : Pourquoi changer de service ?

Chaque service utilise ses propres algorithmes et parfois des données légèrement différentes (bien que basées sur OpenStreetMap).

### 1. 🔄 Fiabilité et Disponibilité

-   Si l'un des services est temporairement indisponible ou lent, basculer sur l'autre vous permet de continuer à travailler.

### 2. 🛣️ Qualité du tracé

-   Parfois, un routeur refusera de passer par un certain chemin (privé, barrière, erreur de carte). L'autre service pourrait être plus permissif ou interpréter les données différemment.

---

## ⚠️ Recommandations

-   **Priorité** : Utilisez `OpenRouteService` par défaut, il est généralement plus satisfaisant pour les tracés complexes.
-   **Alternative** : Ne changez pour `GraphHopper` que si vous rencontrez des problèmes de tracé avec le premier.
