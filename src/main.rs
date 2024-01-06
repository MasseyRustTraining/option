fn option_fn() -> Option<u32> {
    None
}

fn main() {
    let x: Option<u32> = Some(7);
    let y: Option<u32> = option_fn();
    if x.is_some() {
        println!("{}", x.unwrap());
    }
    assert_eq!(None, y);
}
