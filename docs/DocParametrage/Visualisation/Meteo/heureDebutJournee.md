# 🌅 Paramètre : Heure de début de journée

Ce document détaille le paramètre `Heure de début de journée`, qui définit le début de la plage horaire pour la récupération des données météo.

---

## 🎯 Rôle du Paramètre

Ce paramètre fixe l'heure de début (incluse) à partir de laquelle les prévisions météorologiques seront téléchargées et mises en cache.

- **Libellé**: Heure de début de journée
- **Type**: Entier (0-23)
- **Unité**: Heures
- **Valeur par défaut**: 6

## ⚖️ Justification : Optimisation et Pertinence

La récupération des données météo a un coût (taille des fichiers, temps de traitement). Il est inutile de charger des données pour des périodes où vous ne pratiquerez probablement pas d'activité.

### 1. 💾 Optimisation du Cache
-   En limitant la plage horaire, on réduit la taille du fichier JSON de météo stocké pour chaque circuit.
-   Cela évite de stocker des données (température, vent) en pleine nuit si vos activités se déroulent uniquement de jour.

### 2. 🎯 Pertinence des Données
-   Pour une visualisation efficace, seules les heures "utiles" (celles où vous êtes susceptible de rouler/voler) sont nécessaires.

---

## ⚠️ Recommandations

-   **Marge de sécurité** : Prévoyez une marge. Si vous partez parfois très tôt (ex: 5h du matin), réglez ce paramètre sur `5` ou `4`.
-   **Impact** : Si votre simulation commence *avant* cette heure, le système utilisera les données de l'heure disponible la plus proche (ce qui peut être imprécis si l'écart est grand).
