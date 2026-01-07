# 🚴 Paramètre : Vitesse moyenne par défaut (km/h)

Ce document détaille le paramètre `Vitesse moyenne par défaut`, utilisé pour estimer la progression temporelle sur le parcours.

---

## 🎯 Rôle du Paramètre

Ce paramètre définit la vitesse de déplacement théorique utilisée pour calculer l'heure de passage à chaque point du circuit, en l'absence de données réelles.

- **Libellé**: Vitesse moyenne par défaut (km/h)
- **Type**: Réel
- **Valeur par défaut**: 28.0 km/h
- **Unité**: km/h
- **Plage**: 5.0 - 50.0 km/h

## ⚖️ Justification : Pourquoi simuler la vitesse ?

Pour synchroniser la météo avec la position (ex: être au sommet du col à 11h et non à 9h), il faut connaître la vitesse de progression.

### 1. ⏱️ Calcul des Horaires de Passage

-   Connaitre l'heure de départ ne suffit pas.
-   Le système utilise cette vitesse pour projeter votre position dans le temps : `Temps = Distance / Vitesse`.

> [!NOTE]
> La vitesse moyenne est appliquée de manière **linéaire** sur tout le parcours. Elle ne tient pas compte du relief (montées, descentes) qui, dans la réalité, ferait varier la vitesse instantanée.

### 2. 🌤️ Précision des Prévisions

-   Si vous roulez plus lentement que prévu, vous pourriez arriver sous la pluie alors que le système prévoyait du soleil.
-   Une vitesse moyenne réaliste améliore la fiabilité des données météo affichées (vent, température) pour chaque segment.

---

## ⚠️ Recommandations

-   **Soyez réaliste** : Indiquez votre vitesse moyenne réelle sur ce type de parcours. 20 km/h est une bonne moyenne pour du cyclotourisme, 30 km/h pour des cyclosportifs.
-   **Ajustement local** : Comme pour l'heure de départ, ajustez cette valeur spécifique pour chaque circuit si le profil (montagne vs plat) modifie votre performance attendue.
