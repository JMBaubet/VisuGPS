# Post-affichage des bornes kilométriques

## Description

Nombre d'incréments après la borne kilométrique où le message cesse de s'afficher.

## Valeur par défaut

`10` incréments

## Plage de valeurs

- **Minimum** : 0 incréments
- **Maximum** : 20 incréments

## Utilisation

Ce paramètre contrôle combien de temps le message de distance reste visible après avoir dépassé la borne exacte.

Avec un post-affichage de 10 incréments (soit environ 1 km avec des segments de 100m), le message "km 20" restera visible environ 1 km après avoir dépassé le point exact des 20 km.

## Impact

- **Valeur élevée** : Le message reste visible plus longtemps après la borne
- **Valeur faible** : Le message disparaît rapidement après la borne
- **Valeur 0** : Le message disparaît immédiatement après la borne
