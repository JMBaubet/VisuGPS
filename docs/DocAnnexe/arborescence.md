# Arborescence de la Documentation Utilisateur

Ce document présente l'organisation réelle des liens de navigation entre les différents manuels de VisuGPS. Ce schéma a été généré après un audit complet de la documentation pour refléter la nouvelle structure de navigation contextuelle.

```mermaid
graph TD
    %% Noeuds principaux
    Index([index.md])
    Toolbar[toolbar.md]
    Filtres[filtres_tris.md]
    Details[circuit_details.md]
    Upload[upload.md]
    Exploit[exploitation.md]
    Modes[modes_fonctionnement.md]
    FAQ[faq.md]
    MajCommunes[maj_communes.md]
    Param[parametres.md]

    %% Navigation depuis l'Index
    Index --> Toolbar
    Index --> Filtres
    Index --> Details
    Index --> Exploit
    Index --> Modes
    Index --> Param
    Index --> FAQ

    %% Retours à l'accueil (Accueil = index.md)
    Toolbar --> Index
    Filtres --> Index
    Details --> Index
    Exploit --> Index
    Modes --> Index
    FAQ --> Index

    %% Liens depuis la Barre d'Outils
    Toolbar --> MajCommunes
    Toolbar --> Modes
    Toolbar --> Upload

    %% Liens depuis l'Importation (Upload)
    Upload --> Exploit
    Upload --> Details

    %% Liens depuis Détails Circuit
    Details --> MajCommunes
    Details --> Editeur[edition_intro.md]
    Details --> Visu[visualisation.md]
    Details --> Suppression[suppression.md]

    %% Liens depuis le Guide d'Exploitation (Exploitation)
    Exploit --> Toolbar
    Exploit --> Upload
    Exploit --> Editeur
    Exploit --> Visu
    Exploit --> RemoteCouplage[telecommande_couplage.md]
    Exploit --> Export[vgps_export.md]
    Exploit --> Suppression

    subgraph Mode_Edition ["Mode Édition"]
        direction TB
        Editeur[edition_intro.md]
        IntroCam[introduction_camera.md]
        EditFly[edition_flyto_pause.md]
        EditKM[edition_marqueurs_km.md]
        EditMsg[edition_messages.md]
        EditCam[edition_camera.md]
        LibMsg[library_messages.md]

        %% Détails de l'Édition interne
        Editeur --> IntroCam
        Editeur --> EditFly
        Editeur --> EditMsg
        
        IntroCam -->|#les-trois-modes-dédition| Editeur
        IntroCam --> EditCam
        
        EditCam -->|#les-trois-modes-dédition| Editeur
        
        EditFly -->|#les-trois-modes-dédition| Editeur
        
        EditMsg -->|#les-trois-modes-dédition| Editeur
        EditMsg --> EditKM
        EditMsg --> LibMsg
        
        EditKM --> EditMsg
        LibMsg --> EditMsg
    end

    subgraph Mode_Visualisation ["Mode Visualisation"]
        direction TB
        Visu[visualisation.md]
        RemoteCouplage[telecommande_couplage.md]
        RemoteUtil[telecommande_utilisation.md]

        %% Liens internes Visualisation
        Visu --> RemoteCouplage
        Visu --> RemoteUtil
        RemoteCouplage --> RemoteUtil
        RemoteUtil --> RemoteCouplage
    end

    %% Liens depuis le Guide d'Exploitation (Exploitation)
    Exploit --> Toolbar
    Exploit --> Upload
    Exploit --> Editeur
    Exploit --> Visu
    Exploit --> RemoteCouplage
    Exploit --> Export[vgps_export.md]
    Exploit --> Suppression

    %% Liens externes depuis le mode Edition
    Editeur --> Exploit
    IntroCam --> Exploit
    EditCam --> Exploit
    EditFly --> RemoteUtil
    EditFly --> Exploit
    EditMsg --> Exploit

    %% Liens externes depuis le mode Visualisation
    Visu --> Exploit
    RemoteCouplage --> Exploit
    RemoteUtil --> Exploit

    %% Retours Techniques
    Export --> Exploit
    Suppression --> Exploit
    MajCommunes --> Details

    %% Styles
    style Index fill:#f9f,stroke:#333,stroke-width:4px
    style Exploit fill:#bbf,stroke:#333,stroke-width:2px
    style Editeur fill:#dfd,stroke:#333
    style Visu fill:#fdd,stroke:#333
    style Param fill:#ff9,stroke:#333
```

> [!IMPORTANT]
> - Ce diagramme représente les liens **explicites** (boutons, liens "voir [X]") présents dans le texte.
> - Le bouton **Maison** (Home) de l'interface permet de revenir à `index.md` depuis n'importe quelle page.
> - Le bouton **Précédent** (Flèche gauche) de l'interface utilise l'historique de navigation de l'utilisateur.
