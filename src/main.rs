mod chaos;
use chaos::Config;


fn main() {
    let config = Config {height: 3024, width: 3024, iterations: 600000, edges: 3};
    chaos::run(config);

}
