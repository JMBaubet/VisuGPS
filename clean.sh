#!/bin/bash

# Vérifier si le script est exécuté en tant qu'administrateur (avec sudo)
if [ "$(id -u)" -ne 0 ]; then
  echo "Ce script nécessite des droits administrateurs. Demande de privilèges..."
  sudo "$0" "$@"
  exit
fi

# Supprimer src-tauri/target
echo "Suppression de src-tauri/target..."
cd scr-tauri 
rm -rf target
cd ..

# Supprimer les dossiers node_modules
echo "Suppression de node_modules..."
rm -rf node_modules

# Nettoyer le cache npm
echo "Nettoyage du cache npm..."
npm cache clean --force

# Réinstallation des modules 
echo "Installation des node_modules"
npm install

echo "Opération terminée."
