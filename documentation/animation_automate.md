# Automate de l'Animation (VisualizeView)

Ce document détaille les cycles de vie de l'animation au sein du composant unifié `VisualizeView.vue`. L'automate gère intelligemment les deux modes d'entrée (Standard et Variante) tout en partageant un **Socle Commun**.

---

## 1. Flux Unifié (`VisualizeView.vue`)

L'automate s'adapte dynamiquement selon que l'utilisateur visualise une **Trace Principale** (séquence contemplative complète) ou une **Variante** (accès direct au Km 0).

```mermaid
stateDiagram-v2
    classDef common fill:#ffcc00,stroke:#d4a017,stroke-width:2px,color:black;
    classDef entry fill:#e1f5fe,stroke:#01579b,color:black;
    classDef bridge fill:#ffab91,stroke:#d84315,stroke-width:2px,color:black;

    [*] --> Init
    
    Init: 🌑 Carte Masquée
    Vol_Zoom: 🔭 [FLYTO] Zoom vers Vue Globale
    Pause_Globale: 🛑 [PAUSE] Vue Globale
    Vol_Km0_STD: ✈️ [FLYTO] Vol vers Km 0
    Vol_Direct: ✈️ [FLYTO] Zoom km 0
    
    Km0: 🛑 [PAUSE] Km 0
    Anim: 🚀 [ANIM] En Mouvement
    Pause: 🛑 [PAUSE] Manuelle
    
    Arrivee: 🛑 [PAUSE] Arrivée
    Vol_Final: ✈️ [FLYTO] Vue Globale

    Init --> Vol_Zoom: traceType = 'main'
    Init --> Vol_Direct: traceType = 'variant'
    
    Vol_Zoom --> Pause_Globale
    Pause_Globale --> Vol_Km0_STD
    Vol_Km0_STD --> Km0
    Vol_Direct --> Km0
    
    Km0 --> Anim
    Anim --> Pause
    Pause --> Anim
    Pause --> Arrivee
    Anim --> Arrivee
    
    Arrivee --> Vol_Final

    %% Boutons et Navigation Finale
    GoHome: Retour Accueil
    ToVariant: Basculer vers Variantes
    ToMain: Retour Trace Principale

    Vol_Final --> ToVariant
    Vol_Final --> ToMain
    Vol_Final --> GoHome
    Pause --> GoHome

    class Km0,Anim,Pause,Arrivee,Vol_Final common
    class Init,Vol_Zoom,Pause_Globale,Vol_Km0_STD,Vol_Direct entry
    class ToVariant,ToMain bridge
```

---

## 3. Points Techniques Clés

### A. Initialisation Fluide
L'initialisation est conçue pour garantir une transition visuelle parfaite dès l'apparition de la carte :
1. La carte est initialisée avec `isInitializing = true` (masquée par un overlay).
2. Le centre est immédiatement fixé sur le **centre géographique de la trace** (calculé via Turf.js).
3. Le zoom est initialisé à la valeur de `zoomEurope` (défaut 5).
4. La séquence de zoom vers la vue globale est lancée alors que la carte est encore masquée.
5. `isInitializing` passe à `false` après un délai de 200ms : l'utilisateur voit alors l'animation déjà en cours, sans aucun sursaut initial.

### B. Contrôle Présentateur (Reprises Manuelles)
Trois points d'arrêt stratégiques peuvent être configurés pour attendre une action manuelle (`Play`) du présentateur :
- **Vue Globale** : Paramètre `repriseAutoVueTrace`. Permet de présenter le parcours complet.
- **Départ (Km 0)** : Paramètre `repriseAutoKm0`. Permet d'introduire le départ.
- **Arrivée** : Paramètre `repriseAutomatique`. Permet de conclure la séquence.

---

## 4. Synthèse des Paramètres (Rappel)
*Tous ces paramètres sont stockés dans `settingsDefault.json`.*

| Paramètre | Chemin | Usage | Valeur par défaut |
| :--- | :--- | :--- | :--- |
| `zoomEurope` | `Visualisation/Lancement/zoomEurope` | Standard | 5 |
| `durationEuropeToTrace` | `Visualisation/Lancement/durationEuropeToTrace` | Standard | 5.0 s |
| `pauseBeforeStart` | `Visualisation/Lancement/pauseBeforeStart` | Standard | 1.0 s |
| `repriseAutoVueTrace` | `Visualisation/Lancement/repriseAutoVueTrace` | Standard | **true** |
| `durationTraceToStart` | `Visualisation/Lancement/durationTraceToStart` | Standard | 2.0 s |
| `pauseAuKm0` | `Visualisation/Lancement/pauseAuKm0` | **Commun** | 0.5 s |
| `repriseAutoKm0` | `Visualisation/Lancement/repriseAutoKm0` | **Commun** | **true** |
| `vitesse` | `Visualisation/Lecture/vitesse` | **Commun** | 3730 ms/km |
| `timerReprisePause` | `Visualisation/Lecture/timerReprisePause` | **Commun** | 1.0 s |
| `delayAfterAnimationEnd` | `Visualisation/Finalisation/delayAfterAnimationEnd` | **Commun** | 3.0 s |
| `flyToGlobalDuration` | `Visualisation/Finalisation/flyToGlobalDuration` | **Commun** | 2.0 s |
| `repriseAutomatique` | `Visualisation/Finalisation/repriseAutomatique` | **Commun** | **false** |
