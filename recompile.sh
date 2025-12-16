#!/bin/bash

# Enregistrer l'heure de début
START_TIME=$(date +%s)
echo "--- Début de la re-compilation à $(date) ---"

# Exécuter le script de nettoyage
echo "Lancement du nettoyage du projet..."
./clean.sh

# Lancer la compilation en mode build pour générer le .dmg
echo "Lancement de la compilation Tauri (build)..."
# La commande standard pour générer un build Tauri est 'npm run tauri build'
# Tauri détectera automatiquement le système d'exploitation cible (.dmg pour macOS)
npm run tauri build

# Enregistrer l'heure de fin
END_TIME=$(date +%s)

# Calculer le temps écoulé
ELAPSED_TIME=$((END_TIME - START_TIME))

echo "--- Re-compilation terminée ---"
echo "Temps total écoulé : ${ELAPSED_TIME} secondes."
