#!/bin/bash

# Récupère le répertoire courant
DIR="$(pwd)"

# Ouvre une nouvelle fenêtre Terminal avec le premier onglet "gemini"
osascript <<EOF
tell application "Terminal"
    activate
    do script "cd '$DIR'; clear; gemini"
    set custom title of front window to "gemini"
    -- Change la taille de la police (ex. 16)
    set font size of front window to 16

    -- Crée un nouvel onglet et le nomme "commandes"
    tell application "System Events" to tell process "Terminal"
        keystroke "t" using command down
    end tell
    delay 0.2
    do script "cd '$DIR'; clear" in front window
    -- Change la taille de la police (ex. 16)
    set font size of front window to 16
    set custom title of selected tab of front window to "commandes"
end tell
EOF
