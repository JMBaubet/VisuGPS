use serde_json::{Value, Map};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ParamChange {
    pub path: String,
    pub details: String,
}

#[derive(Debug)]
pub struct OutOfBoundsParam {
    pub path: String,
    pub value: String,
    pub min: String,
    pub max: String,
}

#[derive(Debug, Default)]
pub struct MigrationReport {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<ParamChange>,
    pub out_of_bounds: Vec<OutOfBoundsParam>,
    pub all_overrides: Vec<(String, String, bool)>, // (path, value, is_out_of_bounds)
}

pub fn compare_and_merge(old_settings: &Value, new_default: &Value) -> (Value, Option<String>) {
    let mut report = MigrationReport::default();
    let merged = merge_recursive(new_default, old_settings, "".to_string(), &mut report);

    // Extract versions
    let existing_version = old_settings.get("référence").and_then(|r| r.get("version")).and_then(|v| v.as_str());
    let default_version = new_default.get("référence").and_then(|r| r.get("version")).and_then(|v| v.as_str());

    let version_info = if existing_version != default_version {
        Some((existing_version.unwrap_or("N/A"), default_version.unwrap_or("N/A")))
    } else {
        None
    };

    let report_string = generate_markdown_report(&report, version_info);
    
    (merged, report_string)
}

fn merge_recursive(
    default: &Value, 
    existing: &Value, 
    path: String, 
    report: &mut MigrationReport
) -> Value {
    match (default, existing) {
        (Value::Object(def_map), Value::Object(exist_map)) => {
            // Check if this is a "Group" or "Parameter" object based on keys
            if def_map.contains_key("groupes") || def_map.contains_key("parametres") {
                // It's a node containing subgroups and/or parameters
                let mut new_map = def_map.clone();
                
                // Handle "groupes" if present
                if def_map.contains_key("groupes") && def_map["groupes"].is_array() {
                    let def_groups = def_map["groupes"].as_array().unwrap();
                    let exist_groups = exist_map.get("groupes").and_then(|v| v.as_array());
                    
                    if let Some(exist_groups_arr) = exist_groups {
                        let merged_groups = merge_arrays(def_groups, exist_groups_arr, path.clone(), report, "groupes");
                        new_map.insert("groupes".to_string(), Value::Array(merged_groups));
                    } else {
                        // All groups are new
                         for item in def_groups {
                            let key = get_item_key(item);
                            let item_path = if path.is_empty() { key.clone() } else { format!("{}/{}", path, key) };
                            report.added.push(item_path);
                         }
                         // new_map already has the default groups
                    }
                }
                
                // Handle "parametres" if present
                if def_map.contains_key("parametres") && def_map["parametres"].is_array() {
                     let def_params = def_map["parametres"].as_array().unwrap();
                     let exist_params = exist_map.get("parametres").and_then(|v| v.as_array());
                     
                     if let Some(exist_params_arr) = exist_params {
                         let merged_params = merge_arrays(def_params, exist_params_arr, path.clone(), report, "parametres");
                         new_map.insert("parametres".to_string(), Value::Array(merged_params));
                     } else {
                        // All parameters are new
                        for item in def_params {
                            let key = get_item_key(item);
                            let item_path = if path.is_empty() { key.clone() } else { format!("{}/{}", path, key) };
                            report.added.push(item_path);
                         }
                         // new_map already has the default params
                     }
                }

                Value::Object(new_map)
            } else if def_map.contains_key("identifiant") && (def_map.contains_key("type") || def_map.contains_key("defaut")) {
                // It's a Parameter definition
                process_parameter(def_map, exist_map, path, report)
            } else {
                // Generic object merge (like "référence")
                let mut new_map = def_map.clone();
                for (k, v) in def_map {
                    let new_path = if path.is_empty() { k.clone() } else { format!("{}/{}", path, k) };
                    if let Some(exist_v) = exist_map.get(k) {
                        new_map.insert(k.clone(), merge_recursive(v, exist_v, new_path, report));
                    }
                }
                Value::Object(new_map)
            }
        },
        _ => default.clone(), // Fallback: use default if types don't match or not objects
    }
}

fn merge_arrays(
    def_arr: &Vec<Value>, 
    exist_arr: &Vec<Value>, 
    path: String, 
    report: &mut MigrationReport,
    item_type: &str // "groupes" or "parametres"
) -> Vec<Value> {
    let mut merged_arr = Vec::new();
    let mut exist_map: HashMap<String, &Value> = HashMap::new();
    
    // Index existing items by identifiant or libelle
    for item in exist_arr {
        let key = get_item_key(item);
        if !key.is_empty() {
            exist_map.insert(key, item);
        }
    }
    
    // Process default items
    for def_item in def_arr {
        let key = get_item_key(def_item);
        let item_path = if path.is_empty() { key.clone() } else { format!("{}/{}", path, key) };
        
        if let Some(exist_item) = exist_map.get(&key) {
            // Item exists, merge it
            if item_type == "parametres" {
                // For parameters, we need specific comparison logic
                if let Some(def_obj) = def_item.as_object() {
                    if let Some(exist_obj) = exist_item.as_object() {
                        merged_arr.push(process_parameter(def_obj, exist_obj, item_path, report));
                    } else {
                        merged_arr.push(def_item.clone());
                    }
                } else {
                    merged_arr.push(def_item.clone());
                }
            } else {
                // For groups, recursive merge
                merged_arr.push(merge_recursive(def_item, exist_item, item_path, report));
            }
            exist_map.remove(&key); // Mark as processed
        } else {
            // Item is new
            report.added.push(item_path);
            merged_arr.push(def_item.clone());
        }
    }
    
    // Remaining items in exist_map are removed (not in default anymore)
    for (key, _) in exist_map {
        let item_path = if path.is_empty() { key.clone() } else { format!("{}/{}", path, key) };
        report.removed.push(item_path);
    }
    
    merged_arr
}

fn get_item_key(item: &Value) -> String {
    item.get("identifiant")
        .or_else(|| item.get("libelle"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

fn process_parameter(
    def_obj: &Map<String, Value>, 
    exist_obj: &Map<String, Value>, 
    path: String, 
    report: &mut MigrationReport
) -> Value {
    let mut new_param = def_obj.clone();
    let mut modified = false;
    let mut changes = Vec::new();

    // Compare attributes
    let attributes_to_check = vec!["min", "max", "defaut", "libelle", "description", "type", "doc"];
    for attr in attributes_to_check {
        let def_val = def_obj.get(attr);
        let exist_val = exist_obj.get(attr);
        
        if def_val != exist_val {
            modified = true;
            let is_secret = def_obj.get("type").and_then(|v| v.as_str()).unwrap_or("") == "secret";
            let def_str = format_value(def_val, is_secret);
            let exist_str = format_value(exist_val, is_secret);
            changes.push(format!("{}: {} -> {}", attr, exist_str, def_str));
        }
    }

    if modified {
        report.modified.push(ParamChange {
            path: path.clone(),
            details: changes.join(", "),
        });
    }

    // Handle surcharge (override)
    if let Some(surcharge) = exist_obj.get("surcharge") {
        if !surcharge.is_null() {
            // Validate surcharge against NEW min/max
            let mut is_out_of_bounds = false;
            
            if let (Some(min), Some(max)) = (def_obj.get("min"), def_obj.get("max")) {
                if let (Some(val_f64), Some(min_f64), Some(max_f64)) = (surcharge.as_f64(), min.as_f64(), max.as_f64()) {
                    if val_f64 < min_f64 || val_f64 > max_f64 {
                        is_out_of_bounds = true;
                        report.out_of_bounds.push(OutOfBoundsParam {
                            path: path.clone(),
                            value: val_f64.to_string(),
                            min: min_f64.to_string(),
                            max: max_f64.to_string(),
                        });
                    }
                }
            }

            let is_secret = def_obj.get("type").and_then(|v| v.as_str()).unwrap_or("") == "secret";
            report.all_overrides.push((path.clone(), format_value(Some(surcharge), is_secret), is_out_of_bounds));
            
            // Keep the surcharge in the new parameter
            new_param.insert("surcharge".to_string(), surcharge.clone());
        }
    }

    Value::Object(new_param)
}

fn format_value(v: Option<&Value>, is_secret: bool) -> String {
    if is_secret {
        return "******".to_string();
    }
    match v {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Bool(b)) => b.to_string(),
        Some(Value::Null) => "null".to_string(),
        None => "none".to_string(),
        _ => "complex".to_string(),
    }
}

fn generate_markdown_report(report: &MigrationReport, version_info: Option<(&str, &str)>) -> Option<String> {
    let content_changed = !report.added.is_empty() || !report.removed.is_empty() || !report.modified.is_empty();

    if !content_changed && version_info.is_none() {
        return None;
    }

    let mut md = String::new();
    md.push_str("# Rapport de mise à jour des paramètres\n\n");

    if let Some((old_v, new_v)) = version_info {
        md.push_str(&format!(
            "**Mise à jour de la version de configuration de `{}` à `{}`.**\n\n",
            old_v, new_v
        ));
    }

    if content_changed {
        md.push_str("Des modifications de paramètres ont été détectées et votre configuration a été mise à jour pour refléter ces changements.\n\n");
    } else if version_info.is_some() {
        md.push_str("Aucun changement de paramètre n'a été détecté, mais la configuration a été synchronisée en raison du changement de version.\n\n");
    }

    if !report.modified.is_empty() {
        md.push_str("## Paramètres Modifiés\n");
        for change in &report.modified {
            md.push_str(&format!("- **{}** : {}\n", change.path, change.details));
        }
        md.push_str("\n");
    }

    if !report.added.is_empty() {
        md.push_str("## Paramètres Ajoutés\n");
        for path in &report.added {
            md.push_str(&format!("- {}\n", path));
        }
        md.push_str("\n");
    }

    if !report.removed.is_empty() {
        md.push_str("## Paramètres Supprimés\n");
        for path in &report.removed {
            md.push_str(&format!("- {}\n", path));
        }
        md.push_str("\n");
    }

    if !report.all_overrides.is_empty() {
        md.push_str("## Annexe : Vos Surcharges\n");
        md.push_str("Voici la liste de vos paramètres personnalisés. Ceux en **gras et rouge** ne respectent plus les nouvelles limites (min/max).\n\n");
        
        for (path, value, is_oob) in &report.all_overrides {
            if *is_oob {
                // Find details for OOB
                if let Some(oob) = report.out_of_bounds.iter().find(|o| o.path == *path) {
                    md.push_str(&format!("- 🔴 **{}** : Valeur `{}` (Hors limites : [{}, {}])\n", path, value, oob.min, oob.max));
                } else {
                    md.push_str(&format!("- 🔴 **{}** : Valeur `{}` (Hors limites)\n", path, value));
                }
            } else {
                md.push_str(&format!("- {} : `{}`\n", path, value));
            }
        }
    }

    Some(md)
}
