# ⏰ Paramètre : Heure de départ par défaut

Ce document détaille le paramètre `Heure de départ par défaut`, qui définit l'heure de référence pour le calcul des prévisions météorologiques.

---

## 🎯 Rôle du Paramètre

Ce paramètre fixe l'heure à laquelle le parcours est supposé commencer si aucune heure spécifique n'a été définie pour le circuit en cours.

- **Libellé**: Heure de départ par défaut
- **Type**: Heure (HH:MM)
- **Valeur par défaut**: 09:30

## ⚖️ Justification : Importance de la synchronisation temporelle

La météo n'est pas statique ; elle évolue tout au long de la journée. Pour afficher une météo réaliste à chaque kilomètre, le système doit savoir "quand" vous y êtes.

### 1. 🌡️ Évolution des Températures

-   Partir à 08:00 (frais) vs 14:00 (chaud) change radicalement les conditions affichées.
-   Ce paramètre permet d'avoir une base cohérente pour tous les nouveaux circuits importés.

### 2. 📅 Gestion des "Trous" de Données

-   Les fichiers GPX ne contiennent pas de données temporelles (timestamps).
-   En l'absence de ces données, ce paramètre combiné à la vitesse moyenne permet de simuler un horodatage complet du parcours.



---

## ⚠️ Recommandations

-   **Adapter à votre pratique** : Si vous êtes un cycliste matinal, réglez cette valeur sur votre heure de sortie habituelle (ex: 07:30).
-   **Surcharge par circuit** : N'oubliez pas que vous pouvez modifier cette heure pour chaque circuit individuellement en gérant des groupes qui partent à des horaires différents et qui roulent à des vitesses différentes dans les informations du circuit.
