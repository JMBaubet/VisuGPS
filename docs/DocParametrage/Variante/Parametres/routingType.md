# 🚲 Paramètre : Profil de Routage

Ce document détaille le paramètre `routingType`, définissant le mode de déplacement par défaut pour les variantes.

---

## 🎯 Rôle du Paramètre

Le paramètre `routingType` définit le profil de véhicule ou de déplacement utilisé par défaut pour calculer l'itinéraire de votre variante.

-   **Libellé**: Profil de routage
-   **Type**: Liste de choix
-   **Valeur par défaut**: `Route uniquement`
-   **Options**: `Route + Pistes cyclables`, `Route uniquement`, `VTT / Chemin`

## ⚖️ Justification : Pourquoi choisir le bon profil ?

Le profil influence directement les chemins empruntés par le calculateur d'itinéraire.

### 1. 🌲 Adaptation au terrain

-   **Route + Pistes cyclables** : Profil polyvalent (type VTC), emprunte les routes et les aménagements cyclables, évite les difficultés techniques majeures.
-   **Route uniquement** : Profil Vélo de route, privilégie strictement l'asphalte et les surfaces lisses, optimisé pour la vitesse.
-   **VTT / Chemin** : Profil VTT, accepte les sentiers, les chemins de terre, les pentes plus raides et les surfaces techniques.

---

## ⚠️ Recommandations

-   **Défaut** : Choisissez le profil qui correspond à 80% de vos usages.
-   **Flexibilité** : Rappelez-vous que ce n'est qu'une valeur par défaut. Vous pourrez (selon l'implémentation de l'interface) changer ce profil point par point lors de la création de la variante.
