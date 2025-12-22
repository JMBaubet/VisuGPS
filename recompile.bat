@echo off
setlocal

:: Enregistrer l'heure de début
echo --- Debut de la re-compilation a %DATE% %TIME% ---

:: Exécuter le script de nettoyage
echo Lancement du nettoyage du projet...
call clean.bat

:: Lancer la compilation en mode build pour générer l'installeur
echo Lancement de la compilation Tauri (build)...
:: La commande standard pour générer un build Tauri est 'npm run tauri build'
call npm run tauri build

echo --- Re-compilation terminee ---
echo Fin a %DATE% %TIME%

endlocal
