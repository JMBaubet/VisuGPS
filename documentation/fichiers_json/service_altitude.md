# Documentation Service : Altitude et Profil Altimétrique

Ce document décrit comment VisuGPS récupère et traite les informations d'altitude pour les variantes 3D.

## 🏔️ Fournisseurs d'Altitude

L'application sélectionne intelligemment le fournisseur en fonction de la zone géographique :

1.  **IGN (France)** : 
    *   Utilisé systématiquement pour les points situés en **France Métropolitaine et Corse**.
    *   Fournit une précision RGE ALTI (très haute résolution).
2.  **Open-Meteo (Monde)** :
    *   Utilisé comme solution de repli pour les tracés situés en dehors de la France.

---

**L'appel API se produit dès qu'un segment est finalisé (ancré ou validé)**.

Cela permet de lisser la charge sur les serveurs (IGN/Open-Meteo) au lieu de tout demander en masse lors du bouton "Enregistrer".

---

## 🔧 Traitement et Nettoyage (Post-Processing)

Une fois les altitudes reçues du service externe, le backend Rust applique une chaîne de nettoyage :

1.  **Filtrage Médian** : Supprime les "pics" aberrants (ex: un point à 2000m au milieu d'une plaine).
2.  **Moyenne Mobile** : Lisse les petites variations pour éviter les tremblements lors de l'animation 3D.
3.  **Correction de Pente** : Vérifie que la pente entre deux points ne dépasse pas des seuils physiquement impossibles (configurable dans les paramètres).
4.  **Lissage de Jonction** : Pour les segments, assure une transition invisible de l'altitude entre la trace principale (Master) et la variante (pas de "marche d'escalier").

---

## 📊 Cas d'Erreur et Re-calcul

*   **Mode Hors-Ligne / Échec API** : Si le service d'altitude est indisponible au moment du save, le système utilise `0.0` comme valeur par défaut et affiche un avertissement.
*   **Modulo 100m** : L'altitude est interpolée sur chaque point de tracking généré tous les 100m pour garantir que la caméra de l'animation suit toujours le relief.

---

## 🛠️ Paramètres Reliés
`Importation > Altitude Smoothing Median Window`
`Importation > Altitude Smoothing Avg Window`
`Importation > Max Gradient Percent`
