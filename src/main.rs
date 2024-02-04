use crate::settings::Settings;

mod settings;

fn main() {
    let settings = Settings::new();

    println!("{:?}", settings);
}
