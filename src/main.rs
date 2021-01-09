mod chaos;
use chaos::Config;


fn main() {
    let config = Config {height: 3028, width: 3028, iterations: 600000, edges: 5};
    chaos::run(config);

}
