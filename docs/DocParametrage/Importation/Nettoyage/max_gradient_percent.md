# 📉 Paramètre : Pente Maximum Autorisée (%)

Ce document détaille le paramètre `max_gradient_percent`, qui définit la limite physiologique indépassable pour l'inclinaison de la trace.

---

> [!IMPORTANT]
>
>
> **PARAMÈTRE CRITIQUE** : Ce réglage est utilisé par l'algorithme de nettoyage pour détecter et corriger les erreurs d'altitude aberrantes (pics, sauts GPS).

---

## 🎯 Rôle du Paramètre

Le paramètre `max_gradient_percent` agit comme un "garde-fou" lors de l'importation. Il analyse chaque segment de la trace et vérifie si la pente calculée dépasse cette valeur. Si c'est le cas, l'altitude du point est corrigée (rabotée) pour ramener la pente à cette limite maximale.

-   **Libellé**: Pente Maximum Autorisée
-   **Type**: Décimal (Pourcentage)
-   **Valeur par défaut**: 30 %
-   **Minimum**: 1 %
-   **Maximum**: 100 %
-   **Unité**: % (Pourcentage)

## ⚖️ Justification : Pourquoi limiter la pente ?

### 1. 🧹 Correction des Erreurs GPS
- Les GPS barométriques ou satellitaires produisent souvent des "sauts" d'altitude (bruit) alors que vous vous déplacez sur du plat. Cela crée des pentes artificielles de 50%, 80% voire plus sur quelques mètres.
- Ce filtre élimine ces aberrations mathématiquement impossibles pour un cycliste ou un randonneur.

### 2. 🎥 Qualité de la Visualisation 3D
- Lors de l'animation en vue "suivi", une variation brutale de l'altitude provoque des mouvements de caméra désagréables (sauts).
- En limitant la pente, on garantit une caméra fluide et une trace qui épouse le relief de manière réaliste.

---

## ⚠️ Recommandations

-   **Cyclisme sur route** : Une valeur de **20%** ou **25%** est recommandée (il est très rare de dépasser 25% sur route).
-   **VTT / Trail / Randonnée** : Une valeur de **30%** ou **40%** peut être nécessaire pour accepter les passages très raides.
-   **Valeur trop basse** : Si vous réglez ce paramètre trop bas (ex: 5%), vous risquez d'"écraser" le relief réel de votre sortie et de fausser le dénivelé total.
