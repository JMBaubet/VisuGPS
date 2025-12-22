@echo off

echo --- Lancement du nettoyage complet ---

:: Supprimer le dossier de build de Vite
echo 1/5 : Suppression du dossier dist...
if exist dist rd /s /q dist

:: Supprimer le cache de build de Tauri
echo 2/5 : Suppression de src-tauri\target...
if exist src-tauri\target rd /s /q src-tauri\target

:: Supprimer les dépendances et le cache de Vite
echo 3/5 : Suppression de node_modules...
if exist node_modules rd /s /q node_modules

:: Nettoyer le cache npm (recommandé)
echo 4/5 : Nettoyage du cache npm...
call npm cache clean --force

:: Réinstallation des modules
echo 5/5 : Reinstallation des dependances (npm install)...
call npm install

echo Nettoyage termine.
echo Vous pouvez maintenant lancer l'application avec : npm run tauri dev
