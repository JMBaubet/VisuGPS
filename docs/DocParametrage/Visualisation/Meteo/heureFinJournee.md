# 🌇 Paramètre : Heure de fin de journée

Ce document détaille le paramètre `Heure de fin de journée`, qui définit la fin de la plage horaire pour la récupération des données météo.

---

## 🎯 Rôle du Paramètre

Ce paramètre fixe l'heure de fin (incluse) jusqu'à laquelle les prévisions météorologiques seront téléchargées et mises en cache.

- **Libellé**: Heure de fin de journée
- **Type**: Entier (0-23)
- **Unité**: Heures
- **Valeur par défaut**: 20

## ⚖️ Justification : Optimisation et Pertinence

Tout comme pour l'heure de début, définir une heure de fin permet de ne conserver que les données pertinentes pour votre usage.

### 1. 💾 Optimisation du Cache
-   Réduit le volume de données téléchargées et stockées.
-   Évite de polluer le cache avec des données nocturnes souvent inutiles pour les activités de plein air.

### 2. 🌙 Activités Tardives
-   Définit la limite temporelle de votre simulation météo. Au-delà de cette heure, la visualisation utilisera les dernières données connues.

---

## ⚠️ Recommandations

-   **Marge de sécurité** : Si vous prévoyez des sorties longues pouvant se terminer tard (couchers de soleil, retours nocturnes), n'hésitez pas à augmenter cette valeur (ex: `22` ou `23`).
-   **Cohérence** : Assurez-vous que cette heure est postérieure à l'heure de début de journée + la durée estimée de vos parcours les plus longs.
