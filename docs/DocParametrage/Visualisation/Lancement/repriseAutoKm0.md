# ▶️ Paramètre : Reprise automatique au Km 0

Ce document détaille le paramètre `repriseAutoKm0`, qui contrôle le comportement de l'animation lors de la pause au point de départ (Km 0), juste avant que l'animation ne commence à avancer sur le parcours.

---

## 🎯 Rôle du Paramètre

Le paramètre `repriseAutoKm0` détermine si l'animation démarre automatiquement après la pause au Km 0, ou si elle attend une action manuelle du présentateur.

-   **Libellé**: Reprise automatique au Km 0
-   **Type**: Booléen
-   **Valeur par défaut**: `true` (démarrage automatique activé)

## ⚖️ Justification : Pourquoi contrôler le démarrage au Km 0 ?

Ce paramètre est particulièrement important car le Km 0 représente le moment clé du **départ effectif** de l'animation le long du parcours.

### 1. 🎬 Mode Automatique (`true`)

-   L'animation démarre automatiquement après le délai configuré dans **"Durée de la pause au Km 0"**
-   Lecture fluide sans intervention manuelle
-   Parfait pour les vidéos enregistrées
-   Expérience immersive et continue

### 2. 🎤 Mode Présentateur (`false`)

-   L'animation reste en pause au Km 0 jusqu'à ce que le présentateur appuie sur Play
-   Permet d'introduire le parcours avant de le lancer
-   Idéal pour expliquer le contexte, la difficulté, les points clés
-   Crée un effet de suspense avant le départ
-   Facilite la synchronisation avec d'autres contenus (diaporama, vidéo complémentaire, etc.)

---

## ⚠️ Recommandations

-   **Valeur par défaut (`true`)** : Conservez l'activation pour une expérience automatique fluide
-   **Mode présentateur (`false`)** : Désactivez pour les présentations live où vous souhaitez maîtriser le moment exact du départ
-   **Cohérence** : Combinez avec les autres paramètres de reprise automatique ([Vue Trace](repriseAutoVueTrace.md), [Arrivée](../Finalisation/repriseAutomatique.md)) pour un contrôle complet

## 📝 Notes Importantes

-   Ce paramètre affecte également la **réinitialisation** de l'animation (retour au Km 0 après avoir atteint l'arrivée)
-   Si désactivé, le présentateur devra appuyer sur Play à chaque réinitialisation

## 🔗 Paramètres Liés

-   [Durée de la pause au Km 0](pauseAuKm0.md) : Durée de la pause avant démarrage (si mode automatique)
-   [Reprise automatique après vue trace](repriseAutoVueTrace.md) : Contrôle après la vue globale
-   [Reprise automatique à l'arrivée](../Finalisation/repriseAutomatique.md) : Contrôle à la fin du parcours
