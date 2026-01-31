# 🚫 Paramètre : Rétention des télécommandes interdites (Système)

Ce document détaille le paramètre `interdites_retention_days`, qui définit la durée pendant laquelle un appareil refusé reste dans la liste noire (Blacklist).

---

> [!IMPORTANT]
> Une télécommande dans la liste noire ne pourra plus tenter de se connecter à l'application. La rétention définit quand cet appareil sera "pardonné" et pourra de nouveau tenter un couplage.

---

> [!TIP]
> Si vous avez bloqué un appareil par erreur, vous n'êtes pas obligé d'attendre la fin du délai de rétention. Vous pouvez vider manuellement la liste à tout moment depuis la barre d'outils des paramètres.

---

## 🎯 Rôle du Paramètre
...
### 2. ⏳ Droit à l'oubli
-   Une erreur de manipulation (clic accidentel sur "Refuser" au lieu de "Accepter") peut arriver. La rétention garantit que le blocage n'est pas infini et que l'appareil pourra être reconnecté plus tard.

---

## 🧹 Purge manuelle des interdictions

En complément de la rétention automatique, une fonction de **Purge** est disponible dans l'interface :
-   **Localisation** : Bouton `Purger la liste noire` (icône télécommande barrée de rouge) dans la barre d'outils de la vue **Paramètres**.
-   **Action** : Supprime instantanément TOUS les blocages en cours.
-   **Utilité** : Pratique pour ré-autoriser immédiatement un appareil qui avait été banni.

---

## ⚠️ Recommandations

-   **Valeur de sécurité** : Une durée de **14 jours** (valeur par défaut) offre un bon compromis entre protection et flexibilité.
-   **Vérification** : Avant de cliquer sur "Purger", assurez-vous de bien vouloir ré-autoriser l'accès à tous les appareils précédemment refusés.
