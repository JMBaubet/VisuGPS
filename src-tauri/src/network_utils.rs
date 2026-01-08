use local_ip_address::list_afinet_netifas;
use log::{debug, info};

pub async fn get_best_ip() -> String {
    let network_interfaces = match list_afinet_netifas() {
        Ok(interfaces) => interfaces,
        Err(e) => {
            info!("Erreur lors du listage des interfaces réseau : {}", e);
            return "127.0.0.1".to_string();
        }
    };

    let mut potential_ips = Vec::new();

    for (name, ip) in network_interfaces {
        let ip_str = ip.to_string();

        // Ignorer le localhost
        if ip_str == "127.0.0.1" || ip_str == "::1" {
            continue;
        }

        // Ignorer les interfaces virtuelles connues (ZeroTier, VPN, etc.)
        let lower_name = name.to_lowercase();
        if lower_name.contains("zt")
            || lower_name.contains("zerotier")
            || lower_name.contains("feth")
            || lower_name.contains("utun")
            || lower_name.contains("vpn")
            || lower_name.contains("tun")
            || lower_name.contains("tap")
            || lower_name.contains("bridge")
            || lower_name.contains("virtualbox")
            || lower_name.contains("vmware")
            || lower_name.contains("docker")
            || lower_name.contains("vethernet")
            || lower_name.contains("hyper-v")
            || lower_name.contains("cyberghost")
            || lower_name.contains("nordvpn")
            || lower_name.contains("wireguard")
            || lower_name.contains("tailscale")
        {
            debug!("Interface ignorée (virtuelle/VPN) : {} ({})", name, ip_str);
            continue;
        }

        debug!("Interface candidate trouvée : {} ({})", name, ip_str);
        potential_ips.push((name, ip_str));
    }

    // Priorisation des plages d'adresses privées LAN classiques
    // 1. 192.168.x.x (Le plus probable pour le WiFi domestique)
    if let Some(ip) = potential_ips
        .iter()
        .find(|(_, ip)| ip.starts_with("192.168."))
        .map(|(_, ip)| ip)
    {
        return ip.clone();
    }

    // 2. 10.x.x.x (LAN d'entreprise ou certaines configurations, mais attention à ZeroTier déjà filtré par nom)
    if let Some(ip) = potential_ips
        .iter()
        .find(|(_, ip)| ip.starts_with("10."))
        .map(|(_, ip)| ip)
    {
        return ip.clone();
    }

    // 3. 172.x.x.x
    if let Some(ip) = potential_ips
        .iter()
        .find(|(_, ip)| ip.starts_with("172."))
        .map(|(_, ip)| ip)
    {
        return ip.clone();
    }

    // 4. N'importe quelle autre IP trouvée qui n'a pas été filtrée
    if let Some((_, ip)) = potential_ips.first() {
        return ip.clone();
    }

    "127.0.0.1".to_string()
}
