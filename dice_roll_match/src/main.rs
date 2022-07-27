fn add_fancy_hat() {}
fn remove_fancy_hat() {}
// fn move_player(_: u8) {}
// fn reroll() {}

fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}
