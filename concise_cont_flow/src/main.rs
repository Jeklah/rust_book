fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("the maximum is configured to be {}", max),
        _ => (),
    }
    // this can be written shorter way as such:

    let config2 = Some(3u8);
    if let Some(max) = config2 {
        println!("the maximum is configured to be {}", max);
    }

    // if else can include an else block. see matches example for more
    // detail + code.
}
