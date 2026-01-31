# 🕒 Paramètre : Rétention des télécommandes autorisées (Système)

Ce document détaille le paramètre `autorisees_retention_days`, qui définit la durée de conservation dans l'application des télécommandes ayant été préalablement validées par l'utilisateur.

---

> [!NOTE]
> Ce paramètre permet de garder votre liste de télécommandes propre en supprimant automatiquement les appareils que vous n'avez pas utilisés depuis un certain temps.

---

## 🎯 Rôle du Paramètre

Le paramètre `autorisees_retention_days` spécifie le nombre de jours pendant lesquels une télécommande autorisée est conservée dans l'historique de l'application sans nouvelle connexion.

-   **Libellé**: Rétention des télécommandes autorisées
-   **Type**: Entier
-   **Valeur par défaut**: 7 jours
-   **Minimum**: 1 jour
-   **Maximum**: 30 jours
-   **Unité**: jours

## ⚖️ Justification : Pourquoi gérer la rétention des télécommandes ?

La gestion du cycle de vie des autorisations est une bonne pratique tant pour l'ergonomie que pour la sécurité.

### 1. 🧹 Maintenance de la liste
-   Évite l'accumulation de vieux appareils (anciens téléphones, navigateurs temporaires) dans l'interface de gestion des télécommandes.

### 2. 🛡️ Sécurité
-   Limite la durée de validité d'une autorisation permanente. Si un appareil n'est plus utilisé, il perd son accès privilégié après la période définie, obligeant à un nouvel appairage si nécessaire.

---

## ⚠️ Recommandations

-   **Usage fréquent** : Si vous utilisez toujours les mêmes appareils, une valeur de **7 à 14 jours** est idéale pour éviter d'avoir à les ré-autoriser trop souvent.
-   **Usage ponctuel** : Si vous changez souvent de navigateur ou d'appareil de test, vous pouvez réduire cette valeur à **1 ou 2 jours** pour garder une liste toujours à jour.
