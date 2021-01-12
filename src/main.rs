mod chaos;
use chaos::Config;
extern crate confy;
#[macro_use]
extern crate serde_derive;

// #[macro_use]
// extern crate serde_derive;



fn main() {
    // let config = Config {height: 1024, width: 1024, iterations: 60000, edges: 3};
    let cfg: Config = confy::load_path("chaos_rust.toml").unwrap();
    dbg!(cfg);
    chaos::run(cfg);

}
