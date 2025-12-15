# 🔎 Paramètre : Ajouter les distances

Ce document détaille le paramètre `ajouter`, qui permet d'activer ou de désactiver l'ajout automatique des bornes kilométriques lors de l'importation de nouvelles traces GPX.

---

## 🎯 Rôle du Paramètre

Le paramètre `ajouter` contrôle si le système doit générer et insérer automatiquement des événements de "borne kilométrique" pour une trace fraichement importée.

-   **Libellé**: Ajouter les distances
-   **Type**: Booléen
-   **Valeur par défaut**: `true`

## ⚖️ Justification : Pourquoi contrôler l'ajout des bornes kilométriques ?

L'ajout automatique de bornes kilométriques peut être utile pour la structuration initiale d'une trace. Cependant, il peut être désactivé si l'utilisateur préfère gérer manuellement ces repères ou s'il n'en a pas besoin.

### 1. ⏱️ Gain de temps

L'automatisation réduit le travail manuel de positionnement des bornes.

### 2. ✂️ Flexibilité

Permet de désactiver la fonction si elle n'est pas pertinente pour certains cas d'usage.

## ⚠️ Recommandations

-   **Activé par défaut** : Laisser ce paramètre à `true` est recommandé pour la plupart des utilisateurs afin de bénéficier de l'assistance à la structuration des traces.
-   **Désactiver si besoin** : Si vous préférez une personnalisation complète ou n'utilisez pas les bornes kilométriques, désactivez-le.
