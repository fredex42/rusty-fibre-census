mod utils;
mod superxtractor;

fn main() {
    println!("Hello, world!");
    let addresses = utils::get_ip_addresses();
    println!("Found {} local IP addresses", addresses.len());
    for adr in addresses {
        println!("{}",adr);
    }

    let hwinfo = utils::get_hw_info();
    println!("Got hardware info: {:#?}", hwinfo);
}
