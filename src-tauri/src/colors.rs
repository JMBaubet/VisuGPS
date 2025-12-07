use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static VUETIFY_COLOR_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("red-lighten-5", "FFEBEE"); map.insert("red-lighten-4", "FFCDD2"); map.insert("red-lighten-3", "EF9A9A"); map.insert("red-lighten-2", "E57373"); map.insert("red-lighten-1", "EF5350"); map.insert("red", "F44336"); map.insert("red-darken-1", "E53935"); map.insert("red-darken-2", "D32F2F"); map.insert("red-darken-3", "C62828"); map.insert("red-darken-4", "B71C1C");
    map.insert("pink-lighten-5", "FCE4EC"); map.insert("pink-lighten-4", "F8BBD0"); map.insert("pink-lighten-3", "F48FB1"); map.insert("pink-lighten-2", "F06292"); map.insert("pink-lighten-1", "EC407A"); map.insert("pink", "E91E63"); map.insert("pink-darken-1", "D81B60"); map.insert("pink-darken-2", "C2185B"); map.insert("pink-darken-3", "AD1457"); map.insert("pink-darken-4", "880E4F");
    map.insert("purple-lighten-5", "F3E5F5"); map.insert("purple-lighten-4", "E1BEE7"); map.insert("purple-lighten-3", "CE93D8"); map.insert("purple-lighten-2", "BA68C8"); map.insert("purple-lighten-1", "AB47BC"); map.insert("purple", "9C27B0"); map.insert("purple-darken-1", "8E24AA"); map.insert("purple-darken-2", "7B1FA2"); map.insert("purple-darken-3", "6A1B9A"); map.insert("purple-darken-4", "4A148C");
    map.insert("deep-purple-lighten-5", "EDE7F6"); map.insert("deep-purple-lighten-4", "D1C4E9"); map.insert("deep-purple-lighten-3", "B39DDB"); map.insert("deep-purple-lighten-2", "9575CD"); map.insert("deep-purple-lighten-1", "7E57C2"); map.insert("deep-purple", "673AB7"); map.insert("deep-purple-darken-1", "5E35B1"); map.insert("deep-purple-darken-2", "512DA8"); map.insert("deep-purple-darken-3", "4527A0"); map.insert("deep-purple-darken-4", "311B92");
    map.insert("indigo-lighten-5", "E8EAF6"); map.insert("indigo-lighten-4", "C5CAE9"); map.insert("indigo-lighten-3", "9FA8DA"); map.insert("indigo-lighten-2", "7986CB"); map.insert("indigo-lighten-1", "5C6BC0"); map.insert("indigo", "3F51B5"); map.insert("indigo-darken-1", "3949AB"); map.insert("indigo-darken-2", "303F9F"); map.insert("indigo-darken-3", "283593"); map.insert("indigo-darken-4", "1A237E");
    map.insert("blue-lighten-5", "E3F2FD"); map.insert("blue-lighten-4", "BBDEFB"); map.insert("blue-lighten-3", "90CAF9"); map.insert("blue-lighten-2", "64B5F6"); map.insert("blue-lighten-1", "42A5F5"); map.insert("blue", "2196F3"); map.insert("blue-darken-1", "1E88E5"); map.insert("blue-darken-2", "1976D2"); map.insert("blue-darken-3", "1565C0"); map.insert("blue-darken-4", "0D47A1");
    map.insert("light-blue-lighten-5", "E1F5FE"); map.insert("light-blue-lighten-4", "B3E5FC"); map.insert("light-blue-lighten-3", "81D4FA"); map.insert("light-blue-lighten-2", "4FC3F7"); map.insert("light-blue-lighten-1", "29B6F6"); map.insert("light-blue", "03A9F4"); map.insert("light-blue-darken-1", "039BE5"); map.insert("light-blue-darken-2", "0288D1"); map.insert("light-blue-darken-3", "0277BD"); map.insert("light-blue-darken-4", "01579B");
    map.insert("cyan-lighten-5", "E0F7FA"); map.insert("cyan-lighten-4", "B2EBF2"); map.insert("cyan-lighten-3", "80DEEA"); map.insert("cyan-lighten-2", "4DD0E1"); map.insert("cyan-lighten-1", "26C6DA"); map.insert("cyan", "00BCD4"); map.insert("cyan-darken-1", "00ACC1"); map.insert("cyan-darken-2", "0097A7"); map.insert("cyan-darken-3", "00838F"); map.insert("cyan-darken-4", "006064");
    map.insert("teal-lighten-5", "E0F2F1"); map.insert("teal-lighten-4", "B2DFDB"); map.insert("teal-lighten-3", "80CBC4"); map.insert("teal-lighten-2", "4DB6AC"); map.insert("teal-lighten-1", "26A69A"); map.insert("teal", "009688"); map.insert("teal-darken-1", "00897B"); map.insert("teal-darken-2", "00796B"); map.insert("teal-darken-3", "00695C"); map.insert("teal-darken-4", "004D40");
    map.insert("green-lighten-5", "E8F5E9"); map.insert("green-lighten-4", "C8E6C9"); map.insert("green-lighten-3", "A5D6A7"); map.insert("green-lighten-2", "81C784"); map.insert("green-lighten-1", "66BB6A"); map.insert("green", "4CAF50"); map.insert("green-darken-1", "43A047"); map.insert("green-darken-2", "388E3C"); map.insert("green-darken-3", "2E7D32"); map.insert("green-darken-4", "1B5E20");
    map.insert("light-green-lighten-5", "F1F8E9"); map.insert("light-green-lighten-4", "DCEDC8"); map.insert("light-green-lighten-3", "C5E1A5"); map.insert("light-green-lighten-2", "AED581"); map.insert("light-green-lighten-1", "9CCC65"); map.insert("light-green", "8BC34A"); map.insert("light-green-darken-1", "7CB342"); map.insert("light-green-darken-2", "689F38"); map.insert("light-green-darken-3", "558B2F"); map.insert("light-green-darken-4", "33691E");
    map.insert("lime-lighten-5", "F9FBE7"); map.insert("lime-lighten-4", "F0F4C3"); map.insert("lime-lighten-3", "E6EE9C"); map.insert("lime-lighten-2", "DCE775"); map.insert("lime-lighten-1", "D4E157"); map.insert("lime", "CDDC39"); map.insert("lime-darken-1", "C0CA33"); map.insert("lime-darken-2", "AFB42B"); map.insert("lime-darken-3", "9E9D24"); map.insert("lime-darken-4", "827717");
    map.insert("yellow-lighten-5", "FFFDE7"); map.insert("yellow-lighten-4", "FFF9C4"); map.insert("yellow-lighten-3", "FFF59D"); map.insert("yellow-lighten-2", "FFF176"); map.insert("yellow-lighten-1", "FFEE58"); map.insert("yellow", "FFEB3B"); map.insert("yellow-darken-1", "FDD835"); map.insert("yellow-darken-2", "FBC02D"); map.insert("yellow-darken-3", "F9A825"); map.insert("yellow-darken-4", "F57F17");
    map.insert("amber-lighten-5", "FFF8E1"); map.insert("amber-lighten-4", "FFECB3"); map.insert("amber-lighten-3", "FFE082"); map.insert("amber-lighten-2", "FFD54F"); map.insert("amber-lighten-1", "FFCA28"); map.insert("amber", "FFC107"); map.insert("amber-darken-1", "FFB300"); map.insert("amber-darken-2", "FFA000"); map.insert("amber-darken-3", "FF8F00"); map.insert("amber-darken-4", "FF6F00");
    map.insert("orange-lighten-5", "FFF3E0"); map.insert("orange-lighten-4", "FFE0B2"); map.insert("orange-lighten-3", "FFCC80"); map.insert("orange-lighten-2", "FFB74D"); map.insert("orange-lighten-1", "FFA726"); map.insert("orange", "FF9800"); map.insert("orange-darken-1", "FB8C00"); map.insert("orange-darken-2", "F57C00"); map.insert("orange-darken-3", "EF6C00"); map.insert("orange-darken-4", "E65100");
    map.insert("deep-orange-lighten-5", "FBE9E7"); map.insert("deep-orange-lighten-4", "FFCCBC"); map.insert("deep-orange-lighten-3", "FFAB91"); map.insert("deep-orange-lighten-2", "FF8A65"); map.insert("deep-orange-lighten-1", "FF7043"); map.insert("deep-orange", "FF5722"); map.insert("deep-orange-darken-1", "F4511E"); map.insert("deep-orange-darken-2", "E64A19"); map.insert("deep-orange-darken-3", "D84315"); map.insert("deep-orange-darken-4", "BF360C");
    map.insert("brown-lighten-5", "EFEBE9"); map.insert("brown-lighten-4", "D7CCC8"); map.insert("brown-lighten-3", "BCAAA4"); map.insert("brown-lighten-2", "A1887F"); map.insert("brown-lighten-1", "8D6E63"); map.insert("brown", "795548"); map.insert("brown-darken-1", "6D4C41"); map.insert("brown-darken-2", "5D4037"); map.insert("brown-darken-3", "4E342E"); map.insert("brown-darken-4", "3E2723");
    map.insert("blue-grey-lighten-5", "ECEFF1"); map.insert("blue-grey-lighten-4", "CFD8DC"); map.insert("blue-grey-lighten-3", "B0BEC5"); map.insert("blue-grey-lighten-2", "90A4AE"); map.insert("blue-grey-lighten-1", "78909C"); map.insert("blue-grey", "607D8B"); map.insert("blue-grey-darken-1", "546E7A"); map.insert("blue-grey-darken-2", "455A64"); map.insert("blue-grey-darken-3", "37474F"); map.insert("blue-grey-darken-4", "263238");
    map.insert("grey-lighten-5", "FAFAFA"); map.insert("grey-lighten-4", "F5F5F5"); map.insert("grey-lighten-3", "EEEEEE"); map.insert("grey-lighten-2", "E0E0E0"); map.insert("grey-lighten-1", "BDBDBD"); map.insert("grey", "9E9E9E"); map.insert("grey-darken-1", "757575"); map.insert("grey-darken-2", "616161"); map.insert("grey-darken-3", "424242"); map.insert("grey-darken-4", "212121");
    map.insert("black", "000000");
    map.insert("white", "FFFFFF");
    map.insert("transparent", "FFFFFF00");
    map
});

pub fn convert_vuetify_color_to_hex(color_name: &str) -> String {
    let hex = VUETIFY_COLOR_MAP.get(color_name).unwrap_or(&"000000"); // Default to black if not found

    // Only strip the suffix "00" if the hex code is 8 characters long (has an alpha channel)
    if hex.len() == 8 && hex.ends_with("00") {
        hex[..6].to_string() // Take only the first 6 characters
    } else {
        hex.to_string() // Otherwise, return the hex code as is
    }
}

pub fn get_contrasting_text_color(hex_color: &str) -> String {
    let hex_color = hex_color.trim_start_matches('#');
    if hex_color.len() != 6 {
        return "#000000".to_string(); // Default to black for invalid hex
    }

    let r = u8::from_str_radix(&hex_color[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex_color[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex_color[4..6], 16).unwrap_or(0);

    // Calculate luminance using the standard formula
    let luminance = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0;

    if luminance > 0.6 {
        "#000000".to_string() // Bright background, use black text
    } else {
        "#FFFFFF".to_string() // Dark background, use white text
    }
}
