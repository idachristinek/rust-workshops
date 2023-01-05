const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (missiles, ready, _ok) = (STARTING_MISSILES, READY_AMOUNT, true);

    println!("Firing {} of my {} missiles...", ready, missiles);

    // missiles = missiles - ready;

    println!("{} missiles left.", missiles - ready);
}
