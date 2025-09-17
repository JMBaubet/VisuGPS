@echo off
:: Vérifier si le script est exécuté en tant qu'administrateur
::openfiles >nul 2>nul
::if %errorlevel% NEQ 0 (
::    echo Ce script nécessite des droits administrateurs. Redémarrage avec les privilèges administratifs...
::    pause
::    powershell -Command "Start-Process cmd -ArgumentList '/c, %~dp0% && pause' -Verb runAs"
::    exit /b
::)

:: Exécution des commandes
:: cd /d "C:\chemin\vers\ton\projet"   :: Remplace avec le chemin réel de ton projet
echo Suppression de node_modules...
rd /s /q node_modules
echo Nettoyage du cache npm...
npm cache clean --force
echo Installation des node_modules
npm install

