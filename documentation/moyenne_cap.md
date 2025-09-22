# Calcul de la moyenne vectorielle du cap — Note technique

## Contexte
Le cap (ou *bearing*) est un angle mesuré par rapport au nord géographique, exprimé en degrés dans l’intervalle `[0°, 360°)`.  
Lorsqu’on veut calculer une **direction moyenne** à partir de plusieurs segments consécutifs d’une ligne (*LineString*), on ne peut pas utiliser une simple moyenne arithmétique des angles, car les angles sont cycliques.

Exemple : la moyenne de `350°` et `10°` ne doit pas donner `180°`, mais bien `0°`.  
→ On utilise une **moyenne vectorielle**.

---

## Algorithme (formulation compacte)

1. **Sélection des segments**  
   À partir d’un point `P[i]`, on considère les `y` prochains segments :  
   ```
   (P[i] → P[i+1]), (P[i+1] → P[i+2]), …, (P[i+y-1] → P[i+y])
   ```

2. **Calcul des caps élémentaires**  
   Pour chaque segment, calculer le cap géodésique `θ_k` (en radians) via la formule :
   ```
   Δλ = lon₂ - lon₁
   θ_k = atan2( sin(Δλ) · cos(lat₂),
                cos(lat₁) · sin(lat₂) − sin(lat₁) · cos(lat₂) · cos(Δλ) )
   ```
   (lat/lon en radians)

3. **Passage en vecteurs unitaires**
   ```
   x_k = cos(θ_k)
   y_k = sin(θ_k)
   ```

4. **Somme et moyenne**
   ```
   X = Σ x_k
   Y = Σ y_k
   ```

5. **Cap moyen**
   ```
   θ_moy = atan2(Y, X)        // radians
   cap_moy = (θ_moy * 180/π + 360) mod 360  // degrés normalisés
   ```

> Remarque : si `X ≈ 0` et `Y ≈ 0` (vecteur résultat quasi nul), l'angle moyen est mal défini — renvoyer `None` ou gérer ce cas.

---

## Exemple (Rust)

```rust
use std::f64::consts::PI;

/// Coordonnée en degrés
#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

fn to_rad(deg: f64) -> f64 { deg * PI / 180.0 }
fn to_deg_norm(rad: f64) -> f64 {
    let mut deg = rad * 180.0 / PI;
    if deg < 0.0 { deg += 360.0 }
    deg
}

fn bearing(from: Coord, to: Coord) -> f64 {
    let lon1 = to_rad(from.lon);
    let lon2 = to_rad(to.lon);
    let lat1 = to_rad(from.lat);
    let lat2 = to_rad(to.lat);
    let dlon = lon2 - lon1;
    let y = dlon.sin() * lat2.cos();
    let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * dlon.cos();
    to_deg_norm(y.atan2(x))
}

pub fn cap_moyen(coords: &[Coord], start: usize, nb_points: usize) -> Option<f64> {
    if start + nb_points >= coords.len() { return None; }
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    for i in 0..nb_points {
        let b = bearing(coords[start + i], coords[start + i + 1]);
        let rad = to_rad(b);
        sum_x += rad.cos();
        sum_y += rad.sin();
    }
    if sum_x.abs() < 1e-12 && sum_y.abs() < 1e-12 { return None; }
    Some(to_deg_norm(sum_y.atan2(sum_x)))
}
```

---

## Exemple (JS avec Turf.js)

```js
import * as turf from "@turf/turf"

function capMoyen(coords, startIndex, nbPoints) {
  if (startIndex + nbPoints >= coords.length) {
    nbPoints = coords.length - startIndex - 1;
  }
  if (nbPoints <= 0) return null;

  let sumX = 0, sumY = 0;
  for (let i = 0; i < nbPoints; i++) {
    const from = turf.point(coords[startIndex + i]);
    const to = turf.point(coords[startIndex + i + 1]);
    let b = turf.bearing(from, to); // -180..180
    if (b < 0) b += 360;
    const rad = (b * Math.PI) / 180;
    sumX += Math.cos(rad);
    sumY += Math.sin(rad);
  }

  const meanRad = Math.atan2(sumY, sumX);
  let meanDeg = (meanRad * 180) / Math.PI;
  if (meanDeg < 0) meanDeg += 360;
  return meanDeg;
}
```

---

## Schéma (voir fichier SVG joint)
Le fichier `moyenne_cap.svg` illustre :
- la conversion d'angles en vecteurs unitaires sur le cercle trigonométrique,
- la somme vectorielle et l'angle résultant.

---

## Remarques / bonnes pratiques
- Normaliser les angles (0..360°) avant conversion si nécessaire.  
- Utiliser la moyenne vectorielle plutôt que la moyenne arithmétique pour éviter les effets de bord 359°↔0°.  
- Gérer le cas du vecteur résultant quasi nul (distribution isotrope des directions).  
- Si tu travailles sur de longues distances, calcule les bearings avec une formule géodésique (WGS84) et préfère des interpolations géodésiques pour l'échantillonnage.

---

## Licence
Tu peux intégrer cette note dans ton dépôt sous la licence souhaitée (ex : MIT).
