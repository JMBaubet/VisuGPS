# Comprendre l'Interface des Paramètres

VisuGPS est conçu pour être simple à utiliser tout en offrant une grande souplesse de configuration. Voici comment interpréter ce que vous voyez dans le menu des réglages.

[< Retour à l'Index des Paramètres](./parametres.md)

---

## 1. Code Couleur des Icônes

Dans la liste des paramètres, chaque ligne commence par une icône de dossier ou de fichier. Sa couleur vous indique immédiatement l'état du réglage :

*   <img src="https://api.iconify.design/mdi/file-cog-outline.svg?width=20&color=grey" style="vertical-align: middle;"> **Icône Grise** : Le paramètre utilise sa **valeur par défaut**. C'est le réglage recommandé par l'application pour un fonctionnement optimal.
*   <img src="https://api.iconify.design/mdi/file-cog-outline.svg?width=20&color=%23FBC02D" style="vertical-align: middle;"> **Icône Jaune** : Le paramètre a été **modifié**. Vous avez personnalisé cette valeur et elle diffère du réglage d'usine.

---

## 2. Paramètres Experts et Critiques

Certains réglages sont plus sensibles que d'autres car ils touchent au moteur de calcul ou aux connexions réseau.

*   **Titre en Noir/Blanc** : Réglage standard (couleurs, affichage, préférences visuelles). Modifiez-les sans crainte.
*   **Titre en <span style="color: #FF9800">Orange</span>** : **Paramètre Critique**. Ces réglages (ex: lissage de trace, tokens API) demandent une attention particulière. Une valeur inappropriée peut rendre l'animation moins fluide ou bloquer certains services.

---

## 3. Les Outils de Configuration

Chaque ligne de paramètre propose des outils pour vous aider :

### 📖 Accès à la Documentation
L'icône <img src="https://api.iconify.design/mdi/book-open-page-variant-outline.svg?width=20&color=%232196F3" style="vertical-align: middle;"> ouvre directement la page d'aide correspondante dans ce manuel. Elle vous explique l'utilité exacte du réglage et les valeurs conseillées.

### 🔄 Revenir à la Valeur par Défaut
Si vous avez modifié un paramètre et souhaitez revenir au réglage d'usine :
1.  Cliquez sur le paramètre pour ouvrir sa fenêtre de modification.
2.  Si une modification est détectée, l'icône <img src="https://api.iconify.design/mdi/format-color-marker-cancel.svg?width=20&color=%232196F3" style="vertical-align: middle;"> (Réinitialiser) apparaît à droite de la valeur.
3.  Un clic sur cette icône remet instantanément la valeur par défaut.

---

## 4. Apparence : Mode Clair ou Sombre

VisuGPS vous permet de choisir l'ambiance visuelle de l'interface selon votre confort ou la luminosité de votre pièce.

*   **Emplacement** : Le commutateur se situe en haut à droite de la page des **Paramètres**.
*   **Fonctionnement** : 
    *   Basculez vers <img src="https://api.iconify.design/mdi/white-balance-sunny.svg?width=20&color=%23FBC02D" style="vertical-align: middle;"> pour le **Mode Clair** (fond blanc, texte sombre).
    *   Basculez vers <img src="https://api.iconify.design/mdi/weather-night.svg?width=20" style="vertical-align: middle;"> pour le **Mode Sombre** (fond sombre, texte clair).

Le choix est automatiquement mémorisé par l'application pour vos prochaines sessions.

---

## 5. Organisation par Univers

Pour vous repérer, les grands groupes de paramètres utilisent des icônes colorées dans l'arborescence :
*   🟢 **Accueil** : Gestion de la bibliothèque et des circuits.
*   🔵 **Importation** : Réglages appliqués lors de l'ajout de nouveaux fichiers.
*   🟠 **Édition** : Configuration de votre espace de travail 3D (Caméra, Graphes).
*   🟡 **Visualisation** : Options pour l'animation finale (Widgets, Vitesse).
*   ⚙️ **Système** : Réglages techniques et réseau (Tokens, Ports).

---

## 6. Sauvegarde et Migration

*   **Fichier settings.json** : Vos réglages sont enregistrés automatiquement dans votre dossier utilisateur. Il est déconseillé de les modifier manuellement.
*   **Mises à jour** : Lors d'une montée de version de VisuGPS, vos préférences sont conservées et migrées automatiquement vers le nouveau format si nécessaire.

---
[< Accéder à l'Index des Paramètres](./parametres.md)
