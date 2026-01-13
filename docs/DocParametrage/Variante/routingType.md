# 🚲 Paramètre : Profil de Routage

Ce document détaille le paramètre `routingType`, définissant le mode de déplacement par défaut pour les variantes.

---

## 🎯 Rôle du Paramètre

Le paramètre `routingType` définit le profil de véhicule ou de déplacement utilisé par défaut pour calculer l'itinéraire de votre variante.

-   **Libellé**: Profil de routage
-   **Type**: Liste de choix
-   **Valeur par défaut**: `bike`
-   **Options**: `bike`, `mtb`, `racingbike`, `car`, `foot`

## ⚖️ Justification : Pourquoi choisir le bon profil ?

Le profil influence directement les chemins empruntés par le calculateur d'itinéraire.

### 1. 🌲 Adaptation au terrain

-   **bike** : Touring/VTC, évite les gros dénivelés et les chemins trop techniques.
-   **mtb** : VTT, accepte les sentiers, les pentes raides et les surfaces non pavées.
-   **racingbike** : Vélo de route, reste strictement sur l'asphalte et les surfaces lisses.
-   **foot** : Piéton, permet d'emprunter des escaliers, des passages étroits et des sens interdits.

---

## ⚠️ Recommandations

-   **Défaut** : Choisissez le profil qui correspond à 80% de vos usages.
-   **Flexibilité** : Rappelez-vous que ce n'est qu'une valeur par défaut. Vous pourrez (selon l'implémentation de l'interface) changer ce profil point par point lors de la création de la variante.
