# 🔑 Paramètre : Clé API GraphHopper

Ce document détaille le paramètre `Clé API GraphHopper`, nécessaire pour utiliser les fonctions de routage automatique.

---

## 🎯 Rôle du Paramètre

Ce champ permet de stocker votre clé d'authentification personnelle pour le service tiers **GraphHopper**. Ce service est utilisé par VisuGPS pour calculer automatiquement des itinéraires (routes, chemins) entre deux points lors de la création de variantes.

-   **Type** : Secret (Chaîne de caractères)
-   **Valeur par défaut** : Vide

## 📝 Comment obtenir une clé ?

Le service de routage nécessite un compte valide :

1.  Rendez-vous sur le site [GraphHopper](https://www.graphhopper.com/).
2.  Créez un compte et connectez-vous.
3.  Dans votre tableau de bord, générez une nouvelle **API Key**.
4.  Copiez cette clé et collez-la dans ce champ paramètre de VisuGPS.

## ⚠️ Important

-   Cette clé est stockée localement sur votre machine.
-   Sans cette clé, vous devrez tracer vos variantes point par point manuellement (mode "Ligne directe"), ce qui est beaucoup plus fastidieux.
