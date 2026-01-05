# ⛅ Paramètre : Activer la météo

Ce document détaille le paramètre `Activer la météo`, qui contrôle le système global de récupération et d'affichage des données météorologiques.

---

## 🎯 Rôle du Paramètre

Le paramètre `Activer la météo` agit comme un interrupteur principal pour toutes les fonctionnalités liées à la météo dans l'application.

- **Libellé**: Activer la météo
- **Type**: Booléen (Vrai/Faux)
- **Valeur par défaut**: Vrai (Activé)

## ⚖️ Justification : Pourquoi désactiver la météo ?

Bien que l'ajout de la météo enrichisse la visualisation, il peut être nécessaire de la désactiver dans certains contextes.

### 1. 🚀 Performance et Réseau

-   La récupération des données météo nécessite des appels API vers Open-Meteo.
-   Sur une connexion très lente ou inexistante, ces appels peuvent échouer ou ralentir le chargement initial de la visualisation.

### 2. 🎬 Pureté de l'Animation

-   Pour certaines vidéos ou présentations, l'utilisateur peut souhaiter une visualisation épurée, sans widgets supplémentaires ni informations superflues.

---

## ⚠️ Recommandations

-   **Activé (Recommandé)** : Laissez cette option activée pour profiter de l'expérience complète et immersive avec les conditions réelles du parcours.
-   **Désactivé** : Utilisez cette option si vous n'avez pas d'accès internet au moment de la visualisation ou si vous souhaitez une interface minimale.
