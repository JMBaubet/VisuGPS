# Coefficient réducteur message

## Rôle
Ce paramètre permet de réduire progressivement la largeur moyenne des caractères au fur et à mesure que le message s'allonge. Il divise le coefficient de base par un facteur dépendant de la longueur du texte.

## Justification
Les messages courts nécessitent souvent une largeur de caractère moyenne plus élevée pour inclure les overheads (marges, polices larges). À l'inverse, pour les messages très longs, l'estimation linéaire peut devenir excessive. Ce coefficient "amortit" la largeur totale pour les textes longs.

## Fonctionnement
- Formule approximative : `Largeur = (NbCaractères * CoefLargeur) / (1 + NbCaractères * CoefRéducteur)`.
- **0** : Aucune réduction (linéaire).
- **Valeur faible (ex: 0.005)** : Réduction légère pour les très longs textes.
- **Valeur élevée** : Réduction forte, le cadre grandit moins vite que le texte.
- Valeur par défaut recommandée : **0**.
