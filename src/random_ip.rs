use rand::Rng;
use std::net::Ipv4Addr;

pub fn generate_random_ipv4() -> Ipv4Addr {
    let mut rng = rand::thread_rng();
    let octets: [u8; 4] = [rng.gen(), rng.gen(), rng.gen(), rng.gen()];
    Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3])
}
