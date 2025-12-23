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
    Param --> Index

    %% Liens depuis la Barre d'Outils
    Toolbar --> MajCommunes
    Toolbar --> Modes
    Toolbar --> Upload

    %% Liens depuis l'Importation (Upload)
    Upload --> Exploit
    Upload --> Details
    Upload --> Param

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

    %% Détails de l'Édition
    Editeur --> IntroCam[introduction_camera.md]
    Editeur --> EditFly[edition_flyto_pause.md]
    Editeur --> EditKM[edition_marqueurs_km.md]
    Editeur --> EditMsg[edition_messages.md]
    Editeur --> Exploit

    %% Sous-pages d'Édition vers les sommets
    IntroCam -->|#les-trois-modes-dédition| Editeur
    IntroCam --> EditCam[edition_camera.md]
    IntroCam --> Exploit

    EditCam -->|#les-trois-modes-dédition| Editeur
    EditCam --> Exploit

    EditFly -->|#les-trois-modes-dédition| Editeur
    EditFly --> RemoteUtil[telecommande_utilisation.md]
    EditFly --> Exploit

    EditMsg -->|#les-trois-modes-dédition| Editeur
    EditMsg --> EditKM
    EditMsg --> LibMsg[library_messages.md]
    EditMsg --> Exploit
    
    EditKM --> EditMsg

    LibMsg --> EditMsg

    %% Visualisation et Télécommande
    Visu --> RemoteCouplage
    Visu --> RemoteUtil
    Visu --> Exploit

    RemoteCouplage --> Exploit
    RemoteCouplage --> RemoteUtil

    RemoteUtil --> Exploit
    RemoteUtil --> RemoteCouplage

    %% Retours Techniques
    Export --> Exploit
    Suppression --> Exploit
    MajCommunes --> Details
    Modes --> Index

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
