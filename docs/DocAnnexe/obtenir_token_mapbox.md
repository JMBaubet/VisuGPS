# Obtenir un token Mapbox

## 📝 Étapes pour obtenir un token Mapbox

1. **Créer un compte Mapbox**
   
   - Va sur [https://account.mapbox.com](https://account.mapbox.com)
   - Clique sur **Sign up for Mapbox** si tu n’as pas encore de compte.
   - Sélectionne **Individual**.
   - Remplis le formulaire.
   - Accepte les termes.
   - Clique sur **Continue**.
   - Saisir les informations de paiement et valider. (C'est obligatoire pour obtenir un token, mais l'utilisation normale que nous devons en faire est largement en-dessous du seuil gratuit)
   - Valider votre compte via l'email reçu.
   - Confirmer votre mot de passe.
   

2. **Accéder à la page Tokens**
   
   - Une fois connecté, tu arrives sur ton **Tableau de bord** (Account Dashboard).
   - Dans le menu à gauche, clique sur **Tokens**.

3. **Créer un nouveau token** (optionnel)
   
   - Tu as déjà un **Default public token** créé automatiquement (il commence par `pk.`).
   - Si tu veux en générer un spécifique à ton projet :
     - Clique sur **Create a token**
     - Donne-lui un nom (ex. `VisuGPS-Dev`)
     - Laisse les scopes par défaut si c’est juste pour afficher des cartes.
     - Clique sur **Create token** → tu auras un nouveau token `pk.`.

4. **Copier ton token**
   
   - Clique sur l’icône de copie à côté du token.
   
   - **Stocke-le dans une variable d’environnement** (recommandé) :
     
     - Sur Windows PowerShell :
       
       ```powershell
       setx MAPBOX_TOKEN "pk.xxxxx..."
       ```
     
     - Sur macOS/Linux :
       
       ```bash
       export MAPBOX_TOKEN="pk.xxxxx..."
       ```
   
   - Dans ton projet (Vue/Tauri), charge la variable pour l’utiliser dans le code.

---

## ✅ Bonnes pratiques

- **Ne jamais commiter** ton token directement dans Git (même si c’est un `pk.` public).
- Pour le dev, tu peux le mettre dans un fichier `.env` (ajouté à `.gitignore`).
- Si le token est compromis, retourne sur [account.mapbox.com/tokens](https://account.mapbox.com/tokens) et **révoque-le**.
