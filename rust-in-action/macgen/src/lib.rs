use crate::mac_addr::MacAddress;

pub mod mac_addr;

pub fn mac_add() {
    let mac = MacAddress::new();
    assert!(mac.is_local());
    assert!(mac.is_unicast());
    println!("mac: {}", mac);
}
