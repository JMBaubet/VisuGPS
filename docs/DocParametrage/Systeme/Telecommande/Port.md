# 🔌 Paramètre : Port du serveur de télécommande (Système)

Ce document détaille le paramètre `Port`, qui définit le numéro de port utilisé par le serveur WebSocket pour la télécommande de l'application.

---

> [!IMPORTANT]
>
>
> **PARAMÈTRE CRITIQUE** : La modification du port du serveur interrompt immédiatement toute communication avec les télécommandes couplées. Vous devrez mettre à jour l'adresse de connexion sur vos appareils mobiles pour retrouver les contrôles.

---

## 🎯 Rôle du Paramètre

Le paramètre `Port` spécifie le port réseau sur lequel l'application écoutera les connexions entrantes pour la fonctionnalité de télécommande. Il est essentiel pour établir la communication entre l'application et un client de télécommande externe.

-   **Libellé**: Port du serveur de télécommande
-   **Type**: Entier
-   **Valeur par défaut**: 9001
-   **Minimum**: 1024 (ports privilégiés)
-   **Maximum**: 65535 (ports éphémères)

## ⚖️ Justification : Pourquoi définir le port du serveur de télécommande ?

La définition du port est nécessaire pour permettre la communication réseau et la fonctionnalité de télécommande, tout en offrant une flexibilité pour éviter les conflits de ports.

### 1. 🌐 Communication Réseau

-   Un port dédié est requis pour que le serveur WebSocket puisse accepter les connexions des clients de télécommande.

### 2. 🛡️ Éviter les Conflits

-   Permet à l'utilisateur de modifier le port par défaut si celui-ci est déjà utilisé par une autre application sur son système, évitant ainsi les erreurs de connexion.

---

## ⚠️ Recommandations

-   **Valeur par défaut (9001)** : C'est un port non privilégié couramment utilisé pour les applications personnalisées et les services de développement, ce qui réduit les risques de conflit.
-   **Plage de ports** : La plage de 1024 à 65535 est recommandée pour les applications utilisateur, car les ports inférieurs à 1024 sont généralement réservés aux services système.
-   **Vérifier la disponibilité** : Si vous rencontrez des problèmes de connexion avec la télécommande, vérifiez que le port choisi n'est pas déjà utilisé.
