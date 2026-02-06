# Configuration Réseau pour la Télécommande

Ce document détaille les vérifications et configurations réseau nécessaires pour que la fonctionnalité de **télécommande smartphone** fonctionne correctement sur un PC Windows 11.

## 📋 Prérequis

Pour que votre smartphone puisse se connecter à VisuGPS sur votre PC :

1. Votre PC et votre Smartphone doivent être connectés au **même réseau Wi-Fi**.
2. Le réseau Wi-Fi sur votre PC doit être configuré en mode **Privé**.
3. Le **Port 9001** doit être autorisé dans le Pare-feu Windows.

---

## 🔍 Vérifications à Effectuer

### 1. Profil Réseau (Privé vs Public)

Windows 11 bloque par défaut les connexions entrantes si votre réseau Wi-Fi est considéré comme "Public".

**Symptôme :** Le smartphone n'arrive pas à se connecter (Timeout), même avec la bonne adresse IP.

**Comment vérifier et corriger :**

1. Ouvrez les **Paramètres Windows** (`Win + I`).
2. Allez dans **Réseau et Internet** > **Wi-Fi**.
3. Cliquez sur le nom de votre réseau Wi-Fi connecté (Propriétés).
4. Sous "Type de profil réseau", assurez-vous que **Réseau privé** est sélectionné.

> **Astuce PowerShell (Administrateur) :**
> ```powershell
> Set-NetConnectionProfile -InterfaceAlias "Wi-Fi" -NetworkCategory Private
> ```

---

### 2. Pare-feu Windows

VisuGPS utilise le **port 9001** pour communiquer avec le smartphone. Ce port doit être ouvert.

**Symptôme :** Le smartphone charge indéfiniment ou affiche "Connexion refusée".

**Comment configurer :**

Ouvrez **PowerShell en tant qu'administrateur** et exécutez la commande suivante pour créer la règle automatiquement :

```powershell
New-NetFirewallRule -DisplayName "VisuGPS Remote Port 9001" -Direction Inbound -Protocol TCP -LocalPort 9001 -Action Allow -Profile Private
```

---

### 3. Navigateur Smartphone (iOS / Android)

Certains navigateurs mobiles (notamment Safari sur iOS) peuvent avoir des difficultés avec les connexions en temps réel (SSE) si le réseau est instable.

**Symptôme :** L'interface de la télécommande s'affiche mais reste bloquée sur "Connexion SSE...".

**Solutions :**
*   Rafraîchissez la page sur votre smartphone.
*   Assurez-vous que votre PC n'est pas en veille.
*   L'application VisuGPS intègre des correctifs automatiques pour maintenir la connexion active, même sur iPhone.

---

## 🛠️ En cas de problème

Si la connexion ne fonctionne toujours pas :

1.  **Vérifiez l'adresse IP :** Assurez-vous d'utiliser l'adresse IP affichée dans VisuGPS (ex: `http://192.168.1.54:9001`).
2.  **Antivirus tiers :** Si vous utilisez un antivirus autre que Windows Defender (Avast, Norton, etc.), vérifiez qu'il ne bloque pas le port 9001 ou l'application `visugps.exe`.
3.  **Isolation Wi-Fi :** Sur certains routeurs (Bbox, Freebox...), une option "Isolation AP" ou "Isolation Client" peut empêcher les appareils de communiquer entre eux. Vérifiez la configuration de votre box internet.
