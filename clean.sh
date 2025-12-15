#!/bin/bash

echo "--- Lancement du nettoyage complet ---"

# Supprimer le dossier de build de Vite
echo "1/5 : Suppression du dossier dist..."
rm -rf dist

# Supprimer le cache de build de Tauri
echo "2/5 : Suppression de src-tauri/target..."
rm -rf src-tauri/target

# Supprimer les dépendances et le cache de Vite
echo "3/5 : Suppression de node_modules..."
rm -rf node_modules

# Nettoyer le cache npm (recommandé)
echo "4/5 : Nettoyage du cache npm..."
npm cache clean --force

# Réinstallation des modules
echo "5/5 : Réinstallation des dépendances (npm install)..."
npm install

echo "✅ Nettoyage terminé."
echo "Vous pouvez maintenant lancer l'application avec : npm run tauri dev"