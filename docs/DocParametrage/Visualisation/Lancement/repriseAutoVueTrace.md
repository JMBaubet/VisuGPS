# ▶️ Paramètre : Reprise automatique après vue trace

Ce document détaille le paramètre `repriseAutoVueTrace`, qui contrôle le comportement de l'animation lors de la pause sur la vue globale de la trace, juste après le zoom depuis l'Europe.

---

## 🎯 Rôle du Paramètre

Le paramètre `repriseAutoVueTrace` détermine si l'animation reprend automatiquement après la pause d'observation de la trace complète, ou si elle attend une action manuelle du présentateur.

-   **Libellé**: Reprise automatique après vue trace
-   **Type**: Booléen
-   **Valeur par défaut**: `true` (reprise automatique activée)

## ⚖️ Justification : Pourquoi contrôler la reprise après la vue trace ?

Ce paramètre offre une flexibilité essentielle selon le contexte d'utilisation de l'animation.

### 1. 🎬 Mode Automatique (`true`)

-   L'animation reprend automatiquement après le délai configuré dans **"Durée de la pause sur la trace"**
-   Parfait pour une lecture fluide et autonome
-   Adapté aux vidéos enregistrées ou aux animations en boucle
-   Créé une expérience cinématographique continue

### 2. 🎤 Mode Présentateur (`false`)

-   L'animation reste en pause jusqu'à ce que le présentateur appuie sur le bouton Play
-   Idéal pour les présentations commentées en direct
-   Permet de laisser le public observer la trace complète
-   Facilite la synchronisation avec un discours ou une narration
-   Donne au présentateur le contrôle total du rythme

---

## ⚠️ Recommandations

-   **Valeur par défaut (`true`)** : Conservez l'activation pour une expérience automatique fluide
-   **Mode présentateur (`false`)** : Désactivez pour les présentations live où vous souhaitez contrôler manuellement chaque étape
-   **Cohérence** : Combinez avec les autres paramètres de reprise automatique ([Km 0](repriseAutoKm0.md), [Arrivée](../Finalisation/repriseAutomatique.md)) pour un contrôle complet

## 🔗 Paramètres Liés

-   [Durée de la pause sur la trace](pauseBeforeStart.md) : Durée de la pause avant reprise (si mode automatique)
-   [Reprise automatique au Km 0](repriseAutoKm0.md) : Contrôle au point de départ
-   [Reprise automatique à l'arrivée](../Finalisation/repriseAutomatique.md) : Contrôle à la fin du parcours
