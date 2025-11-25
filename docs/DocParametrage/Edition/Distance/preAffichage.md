# Pré-affichage des bornes kilométriques

## Description

Nombre d'incréments avant la borne kilométrique où le message commence à s'afficher.

## Valeur par défaut

`10` incréments

## Plage de valeurs

- **Minimum** : 0 incréments
- **Maximum** : 20 incréments

## Utilisation

Ce paramètre contrôle à partir de quel moment le message de distance devient visible avant d'atteindre la borne exacte.

Avec un pré-affichage de 10 incréments (soit environ 1 km avec des segments de 100m), le message "km 20" commencera à s'afficher environ 1 km avant le point exact des 20 km.

## Impact

- **Valeur élevée** : Le message apparaît plus tôt, donnant plus de temps pour anticiper
- **Valeur faible** : Le message apparaît juste avant la borne
- **Valeur 0** : Le message n'apparaît qu'à partir de la borne exacte
