# ⏱️ Paramètre : Pré affichage arrivée

Ce document détaille le paramètre `Pré affichage arrivée`, qui gère l'anticipation de l'affichage du message de fin.

---

## 🎯 Rôle du Paramètre

Ce paramètre définit combien d'incréments *avant* la fin du parcours le message d'arrivée doit commencer à apparaître.

-   **Type**: Entier
-   **Valeur par défaut**: 10
-   **Plage recommandée**: 1 - 20

---

## 💡 Utilisation

Contrairement au départ (qui reste affiché *après*), l'arrivée doit souvent être annoncée *avant* que la caméra ne s'arrête.
- Une valeur de **10** signifie que le message apparaîtra 10 points avant la fin réelle de la trace.
- Cela permet de voir le panneau "Arrivée" grossir à l'approche de la ligne finale, créant un effet de conclusion naturel.
