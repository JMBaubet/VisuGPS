# 🔄 Synchronisation Multi-plateforme (macOS / Windows)

Ce guide explique comment maintenir la cohérence des versions de **VisuGPS** lorsque vous travaillez alternativement sur macOS et Windows.

---

## ⚡ Le concept de cohérence Tauri

Dans une application Tauri, il est impératif que les versions des bibliothèques JavaScript correspondent aux versions des composants Rust. Si une différence existe, le `tauri build` échouera (souvent avec une erreur de type "mismatched versions").

### 📦 Les fichiers de verrouillage (Lock Files)

Pour garantir cette cohérence, deux fichiers sont essentiels et **doivent être suivis par Git** :

1.  **`package-lock.json`** : Verrouille les versions des dépendances NPM (JavaScript).
2.  **`src-tauri/Cargo.lock`** : Verrouille les versions des crates Rust (Backend).

---

## 🚀 Procédure de synchronisation

Suivez ces étapes lorsque vous passez d'un système à l'autre :

### 1. Sur la machine source (où vous avez fait des modifs) :
1. Assurez-vous que vos fichiers de configuration sont à jour.
2. Ajoutez et committez les fichiers lock :
   git add package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock
3. Poussez vers GitHub :
   git push origin <votre-branche>

### 2. Sur la machine cible (où vous voulez récupérer le travail) :
1. Récupérez les changements :
   git pull origin <votre-branche>
2. Mettez à jour les dépendances JavaScript :
   npm install
3. La partie Rust se mettra à jour automatiquement lors du prochain `npm run tauri dev` ou `build`.

---

## 🛠️ En cas d'erreur de build
Si `npm run tauri build` affiche une erreur de versions incohérentes :
1. Ouvrez `package.json` et `src-tauri/Cargo.toml`.
2. Vérifiez que les versions de `@tauri-apps/api` et des plugins correspondent aux versions des crates `tauri` et `tauri-plugin-*`.
3. Après modification, lancez `npm install`.

---

> [!IMPORTANT]
> Ne jamais ajouter `Cargo.lock` ou `package-lock.json` au fichier `.gitignore`. Ils sont vos meilleurs alliés pour un développement multi-plateforme sans erreur.
