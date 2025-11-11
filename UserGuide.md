# Guide de Rédaction de la Documentation Utilisateur

Ce document décrit les contraintes et les directives à suivre lors de la création de la documentation utilisateur pour le projet VisuGPS. L'objectif est de produire des documents clairs, concis, visuellement attrayants et faciles à comprendre pour les utilisateurs finaux.

## 1. Format et Structure

*   **Format :** Tous les documents de documentation utilisateur doivent être rédigés en **Markdown**.
*   **Structure :**
    *   Utiliser des titres (`#`, `##`, `###`) pour organiser le contenu de manière logique.
    *   Commencer par une introduction claire décrivant l'objectif du document.
    *   Diviser les sections en fonction des fonctionnalités ou des étapes du processus.
    *   Utiliser des listes à puces ou numérotées pour les étapes ou les énumérations.

## 2. Contenu et Clarté

*   **Clarté et Concision :** Le langage doit être simple, direct et facile à comprendre. Éviter le jargon technique autant que possible, ou l'expliquer si nécessaire.
*   **Orientation Utilisateur :** Se concentrer sur ce que l'utilisateur doit savoir pour accomplir une tâche, plutôt que sur les détails d'implémentation.
*   **Comportement de l'Application :** Décrire le comportement attendu de l'application dans différents scénarios (ex: au lancement, au changement de vue, en cas d'erreur).
*   **Paramètres :** Si des paramètres sont pertinents pour la fonctionnalité documentée, expliquer leur rôle, leur chemin dans l'interface de configuration et leurs valeurs par défaut/impact.

## 3. Illustrations Graphiques (Icônes et IHM)

Les illustrations visuelles sont essentielles pour la compréhension.

*   **Type d'illustrations :** Utiliser des icônes et des représentations simplifiées d'éléments d'interface utilisateur (IHM).
*   **Format :** Les icônes doivent être intégrées en **SVG** pour une meilleure qualité et adaptabilité.
*   **Source des Icônes :** Privilégier l'utilisation de la bibliothèque **Iconify** via ses URLs directes.
    *   **Exemple :** `<img src="https://api.iconify.design/mdi/web-check.svg?color=green&width=24" alt="Icône Web Check" width="24" height="24" style="vertical-align: -0.25em;">`
*   **Couleurs :** Les icônes doivent utiliser les **couleurs exactes** définies dans le code de l'application (ex: `red-darken-2`, `primary`, `success`, `error`). Ces couleurs doivent être converties en codes hexadécimaux ou noms de couleurs CSS valides pour les URLs Iconify.
*   **Alignement Vertical :** Assurer que les icônes sont correctement alignées verticalement avec le texte environnant. Utiliser le style `style="vertical-align: -0.25em;"` (ou une valeur ajustée si nécessaire) dans la balise `<img>`.
*   **IHM complexes :** Pour les éléments d'IHM plus complexes (ex: une barre de progression avec plusieurs composants), une description textuelle claire peut être préférable à une image SVG unique si la représentation SVG devient trop complexe ou difficile à maintenir. Si une image est nécessaire, elle doit être une capture d'écran ou un SVG simplifié.

## 4. Langue

*   **Langue :** La documentation doit être rédigée en **français**.

## 5. Exemples de Bonnes Pratiques

*   **Pour une icône de statut :**
    *   `Icône de statut <img src="https://api.iconify.design/mdi/check-circle.svg?color=%234CAF50&width=24" alt="Icône Succès" width="24" height="24" style="vertical-align: -0.25em;"> :` Indique une opération réussie.
*   **Pour un bouton d'action :**
    *   `Cliquez sur le bouton <img src="https://api.iconify.design/mdi/play.svg?color=%232196F3&width=24" alt="Icône Lecture" width="24" height="24" style="vertical-align: -0.25em;"> pour démarrer l'animation.`

En suivant ces directives, nous nous assurerons que la documentation utilisateur de VisuGPS est de haute qualité et répond aux besoins de nos utilisateurs.
