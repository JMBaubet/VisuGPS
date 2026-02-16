# 🐛 Paramètre : Mode Debug

Ce document détaille le paramètre `Debug`, qui permet d'activer les fonctionnalités réservées au développement même lorsque l'application est compilée en mode production.

---

## 🎯 Rôle du Paramètre

Le paramètre `Debug` permet de forcer l'affichage des outils de diagnostic et de débogage qui sont normalement masqués pour l'utilisateur final.

-   **Libellé**: Mode Debug
-   **Type**: Booléen
-   **Valeur par défaut**: Désactivé (false)
-   **Identifiant**: `Debug`

## ⚖️ Justification : Pourquoi activer le mode Debug ?

Ce paramètre est utile pour le support technique ou pour les utilisateurs avancés souhaitant diagnostiquer des problèmes précis.

### 1. 🔍 Outils de Diagnostic
Active l'affichage d'icônes ou de menus supplémentaires permettant de visualiser des informations techniques (ex: bouton "bug" sur les circuits).

### 2. 📂 Inspection des Chemins
Affiche les chemins internes des paramètres dans l'interface de réglages (tooltips), facilitant la communication avec le support technique.


---

## ⚠️ Recommandations

-   **Utilisation Normale (Désactivé)** : Sauf besoin spécifique de diagnostic, laissez ce paramètre désactivé pour conserver une interface épurée et éviter des manipulations techniques accidentelles.
-   **Support Technique** : Activez ce mode si l'équipe de développement vous le demande pour faciliter le dépannage.
