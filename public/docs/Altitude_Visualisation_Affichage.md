# 📊 Paramètre : Afficher le profil altimétrique

Ce document détaille le paramètre `Affichage`, qui contrôle la visibilité du profil altimétrique dans la vue de visualisation.

---

## 🎯 Rôle du Paramètre

Le paramètre `Affichage` est un interrupteur (booléen) qui, lorsqu'activé, affiche le graphique du profil altimétrique de la trace. Lorsqu'il est désactivé, le profil n'est pas visible.

-   **Libellé**: Afficher le profil
-   **Type**: Booléen
-   **Valeur par défaut**: `true`
-   **Surcharge**: `null` (indique que ce paramètre peut être surchargé par les paramètres spécifiques à une trace)

## ⚖️ Justification : Pourquoi contrôler l'affichage du profil altimétrique ?

Le contrôle de l'affichage du profil altimétrique permet à l'utilisateur de choisir s'il souhaite visualiser cette information supplémentaire, en fonction de ses besoins et de la clarté de l'interface.

### 1. 👀 Clarté de l'Interface

-   Permet de désencombrer l'interface si le profil altimétrique n'est pas nécessaire pour une visualisation donnée.
-   Utile pour les utilisateurs qui préfèrent se concentrer uniquement sur la carte 3D.

### 2. 📈 Analyse Altimétrique

-   Lorsqu'activé, il fournit une représentation graphique essentielle des changements d'altitude le long de la trace, facilitant l'analyse des montées et des descentes.

---

## ⚠️ Recommandations

-   **Valeur par défaut (`true`)** : L'affichage par défaut du profil altimétrique est recommandé car il fournit une information clé sur la trace.
-   **Désactiver si non nécessaire** : Si l'espace d'écran est limité ou si l'analyse altimétrique n'est pas la priorité, vous pouvez désactiver ce paramètre.
