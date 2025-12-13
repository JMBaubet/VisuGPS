# 🖥️ Paramètre : Ecran

Ce document détaille le paramètre `ecran`, qui permet de choisir sur quel moniteur l'application doit s'ouvrir au démarrage.

---

## 🎯 Rôle du Paramètre

Le paramètre `ecran` détermine l'écran cible pour l'affichage de la fenêtre principale dans une configuration multi-écrans.

-   **Libellé**: Ecran
-   **Type**: Moniteur (Sélecteur dynamique)
-   **Valeur par défaut**: 0 (Écran principal)
-   **Valeurs disponibles**: La liste de vos écrans connectés (détectés automatiquement).

## ⚖️ Justification : Pourquoi choisir l'écran ?

Dans un environnement de travail avec plusieurs moniteurs, il est pratique de pouvoir dédier un écran spécifique à l'application sans avoir à déplacer la fenêtre à chaque lancement.

### 1. 🎛️ Organisation de l'Espace
Vous pouvez par exemple garder votre écran principal pour vos tâches courantes (mails, web) et dédier un écran secondaire à VisuGPS pour la visualisation de cartes en grand format.

### 2. 📽️ Projection
Si vous utilisez un projecteur ou un écran externe pour des présentations, vous pouvez configurer l'application pour qu'elle s'ouvre directement sur cet affichage secondaire.

---

## ⚠️ Fonctionnement

-   **Détection Automatique** : La liste des écrans est mise à jour dynamiquement lorsque vous ouvrez la fenêtre de configuration.
-   **Indexation** : Les écrans sont numérotés (0, 1, 2...) selon l'ordre détecté par votre système d'exploitation. Le nom et la résolution de chaque écran sont affichés pour vous aider à les identifier.
-   **Absence de l'écran** : Si l'écran configuré n'est plus connecté (ex: déplacement avec un ordinateur portable), l'application s'ouvrira par défaut sur l'écran principal (0).
