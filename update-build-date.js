import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const settingsPath = path.join(__dirname, 'src-tauri', 'settingsDefault.json');
const indexPath = path.join(__dirname, 'docs', 'DocUtilisateur', 'index.md');

try {
    // 1. Read Settings
    if (!fs.existsSync(settingsPath)) {
        console.error(`Settings file not found at ${settingsPath}`);
        process.exit(1);
    }

    const settingsRaw = fs.readFileSync(settingsPath, 'utf8');
    const settings = JSON.parse(settingsRaw);

    // Extract versions (handling potential "référence" key with accent)
    // Based on file check: "référence" key exists at root.
    const ref = settings['référence'] || {};

    const versionApp = ref.version || 'Unknown';
    const versionArchivage = ref.version_export_circuit || 'Unknown';
    const versionExport = ref.version_export_context || 'Unknown';

    console.log(`Versions found: App=${versionApp}, Arc=${versionArchivage}, Exp=${versionExport}`);

    // 2. Read Index.md
    if (!fs.existsSync(indexPath)) {
        console.error(`Index file not found at ${indexPath}`);
        process.exit(1);
    }

    let indexContent = fs.readFileSync(indexPath, 'utf8');

    // 3. Update Versions (using Regex to allow updating already replaced values)
    // Pattern: capture the key part, replacement updates the value part

    // Update Application Version
    indexContent = indexContent.replace(
        /(\*   \*\*Version de l'application\*\* : ).*/,
        `$1${versionApp}`
    );
    // Fallback if placeholder still exists (first run)
    indexContent = indexContent.replace('{{versionApplication}}', versionApp);


    // Update Archivage Version
    indexContent = indexContent.replace(
        /(\*   \*\*Version d'export des circuits\*\* : ).*/,
        `$1${versionArchivage}`
    );
    indexContent = indexContent.replace('{{versionArchivage}}', versionArchivage);

    // Update Export Version
    indexContent = indexContent.replace(
        /(\*   \*\*Version d'export des contextes\*\* : ).*/,
        `$1${versionExport}`
    );
    indexContent = indexContent.replace('{{versionExport}}', versionExport);


    // 4. Update/Append Build Date Footer
    const now = new Date();
    // Format: JJ/MM/AAAA à HH:MM
    const dateStr = now.toLocaleDateString('fr-FR', { day: '2-digit', month: '2-digit', year: 'numeric' });
    const timeStr = now.toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit' });

    // Discreet HTML footer
    const footerLine = `<div style="text-align: right; font-size: 0.7em; opacity: 0.7; margin-top: 20px;"><em>Généré le ${dateStr} à ${timeStr}</em></div>`;

    // Regex to find existing footer line (any of the formats we've used)
    const footerRegex = /(> \*\*Application générée le .*|<div style="text-align: right;.*Généré le .*<\/div>)/;

    if (footerRegex.test(indexContent)) {
        indexContent = indexContent.replace(footerRegex, footerLine);
    } else {
        // Append to end if not found
        if (!indexContent.endsWith('\n')) indexContent += '\n';
        indexContent += `\n${footerLine}\n`;
    }

    // 5. Write back
    fs.writeFileSync(indexPath, indexContent, 'utf8');
    console.log(`Updated ${indexPath} successfully.`);

} catch (err) {
    console.error("Error updating build date/versions:", err);
    process.exit(1);
}
