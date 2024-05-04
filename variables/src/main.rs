const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;
fn main() {
    // let mut missiles = STARTING_MISSILES;
    // let ready = READY_AMOUNT;
    let (mut missiles, ready): (i32, _) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    // missiles = missiles - ready;
    let wololo:i32;
    println!("{} missiles left", missiles - ready);
}
