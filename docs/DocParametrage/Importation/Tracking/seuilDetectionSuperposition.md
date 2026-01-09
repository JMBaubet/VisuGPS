# Seuil de détection des superpositions

## 📋 Description

Ce paramètre définit la **distance maximale** (en mètres) pour considérer que deux segments de la trace GPX se superposent géographiquement.

---

## 🎯 Utilisation

Lors de l'import d'un fichier GPX, VisuGPS analyse automatiquement la trace pour détecter les **allers-retours** : des segments où le parcours passe deux fois au même endroit (par exemple, une boucle avec retour au point de départ).

Cette détection permet d'afficher correctement la pente sur les segments aller **et** retour, au lieu de n'afficher que la pente du dernier passage.

---

## ⚙️ Valeurs

| Propriété | Valeur |
|-----------|--------|
| **Type** | Nombre entier |
| **Défaut** | 20 mètres |
| **Minimum** | 5 mètres |
| **Maximum** | 100 mètres |
| **Unité** | m (mètres) |

---

## 💡 Impact du paramètre

### Valeur trop faible (< 10m)

❌ **Risque** : Ne pas détecter certains allers-retours réels
- Les segments qui passent à 15-20m l'un de l'autre ne seront pas détectés
- La pente du retour écrasera celle de l'aller

### Valeur optimale (15-25m)

✅ **Recommandé** pour la plupart des cas
- Détecte correctement les vrais allers-retours
- Évite les faux positifs sur les virages serrés

### Valeur trop élevée (> 50m)

❌ **Risque** : Faux positifs
- Des virages serrés ou lacets peuvent être détectés comme des superpositions
- Affichage incorrect de la pente sur des segments distincts

---

## 🎨 Valeurs recommandées par type de trace

| Type de trace | Seuil recommandé | Justification |
|---------------|------------------|---------------|
| **Randonnée pédestre** | 15-20m | Précision GPS moyenne, chemins étroits |
| **VTT / Cyclisme** | 20-30m | Vitesse plus élevée, imprécision GPS accrue |
| **Vol (parapente, drone)** | 30-50m | Altitude variable, grande imprécision GPS |
| **Course à pied** | 15-20m | Similaire à la randonnée |

---

## ⚠️ Paramètre critique

Ce paramètre est marqué comme **CRITIQUE** car il affecte :

1. **La détection des zones de superposition**
   - Modifie le nombre de zones détectées
   - Impacte la précision de la segmentation aller/retour

2. **L'affichage de la pente**
   - Détermine quels segments sont considérés comme superposés
   - Influence la couleur affichée pendant la visualisation

3. **Les performances**
   - Valeur trop élevée = plus de calculs
   - Valeur trop faible = détection incomplète

### 🔄 Modification du paramètre

> [!CAUTION]
> Si vous modifiez ce paramètre, vous devrez **ré-importer** vos traces GPX existantes pour régénérer les métadonnées de superposition.

Les fichiers `segments_metadata.json` existants ne seront **pas** automatiquement mis à jour.

---

## 📊 Exemple concret

### Trace avec aller-retour

Imaginons une randonnée en montagne avec un aller-retour sur le même sentier :

- **Km 0-5** : Montée (aller) - pente +8%
- **Km 5-10** : Suite du parcours
- **Km 10-15** : Retour sur le même sentier - pente -8%

#### Avec seuil = 20m ✅

L'application détecte que les km 0-5 et km 10-15 se superposent :
- Au km 2 (aller) : affiche la pente +8% (montée)
- Au km 12 (retour) : affiche la pente -8% (descente)

#### Avec seuil = 5m ❌

L'application ne détecte pas la superposition (le sentier fait 10m de large) :
- Au km 2 (aller) : affiche la pente -8% (celle du retour, incorrect !)
- Au km 12 (retour) : affiche la pente -8% (correct par hasard)

---

## 🔗 Voir aussi

- [Longueur du segment](LongueurSegment.md) - Paramètre lié au tracking
- [Lissage du cap](LissageCap.md) - Autre paramètre critique du tracking
- [Documentation complète des paramètres](../../parametres.md)

---

## 🛠️ Détails techniques

L'algorithme de détection utilise la **formule de Haversine** pour calculer la distance entre deux points GPS :

```
distance = 2 * R * arcsin(√(sin²(Δφ/2) + cos(φ1) * cos(φ2) * sin²(Δλ/2)))
```

Où :
- R = rayon de la Terre (6371 km)
- φ = latitude
- λ = longitude

Cette formule prend en compte la courbure de la Terre pour un calcul précis.
