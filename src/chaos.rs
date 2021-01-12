extern crate image;
use rand::Rng;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::f32::consts;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
extern crate confy;

// #[macro_use]
// extern crate serde_derive

// use wasm_bindgen::prelude::*;

// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// pub fn generate_image(config: Config){

//     let mut imgbuf = image::ImageBuffer::new(config.height, config.width);
//     // set background color
//     for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
//         *pixel = image::Rgb([0 as u8, 0 as u8, 0 as u8]);
//     }
// }

// pub fn run(config: Config) -> [u8; 1024*1024*4]{
pub fn run(config: Config){
   let mut imgbuf = image::ImageBuffer::new(config.height, config.width);

    // set background color
    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([0 as u8, 0 as u8, 0 as u8]);
    }

    let edges = edges_of_polygon(config.edges,
        config.width as f32);

    // default color
    let color = image::Rgb([255, 255, 255]);

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();
    let workload = config.height * config.width / num_cpus::get() as u32;

    for _i in 0..num_cpus::get(){
        let tx = tx.clone();
        let config = config.clone();
        let edges = edges.clone();
        let mut point = Point{x: (config.width / 2) as i32, y: (config.height /2) as i32};
        pool.execute(move ||{
            let mut current_edge = 0;
            let current_edge = Rc::new(RefCell::new(0));

            // Closures should ideally be defined outside each thread. 

            let regular_pattern = |previous_point: &Point| -> Point {
                let mut rng = rand::thread_rng();
                previous_point.middle_point(&edges[rng.gen_range(0..config.edges as i32) as usize], config.factor)
            };

            // The current vertex cannot be chosen in the next iteration
            let mut same_edge_cannot_be_chosen_next = |previous_point: &Point| -> Point {
                let mut rng = rand::thread_rng();
                let mut new_edge = rng.gen_range(0..config.edges as i32);
                // while new_edge == current_edge {
                while new_edge == *current_edge.borrow(){
                    new_edge = rng.gen_range(0..config.edges as i32);
                };
                // current_edge = new_edge;
                *current_edge.borrow_mut() = new_edge;
                previous_point.middle_point(&edges[new_edge as usize], config.factor)
            };

            // The current vertex cannot be one place away (anti-clockwise) from the previous vertex
            let mut edge_one_away_cannot_be_chosen_next = |previous_point: &Point| -> Point {
                let mut rng = rand::thread_rng();
                let mut new_edge = rng.gen_range(0..config.edges as i32);
                let config = config.clone();
                let mut prohibited_new_edge: i32 = *current_edge.borrow() - 1;
                prohibited_new_edge = prohibited_new_edge.rem_euclid(config.edges as i32);
                // while new_edge == prohibited_new_edge {
                while new_edge == *current_edge.borrow(){
                    new_edge = rng.gen_range(0..config.edges as i32);
                };
                // current_edge = new_edge;
                *current_edge.borrow_mut() = new_edge;
                previous_point.middle_point(&edges[new_edge as usize], config.factor)
            };


            for _ in 0..workload {
                point = regular_pattern(&point);
                // point = same_edge_cannot_be_chosen_next(&point);
                tx.send(point).expect("Threading failed");
        }})}

    for _i in 0..config.iterations{
        let point = rx.recv().unwrap();
        imgbuf.put_pixel(point.x as u32, point.y as u32, color)
    }

    imgbuf.save("test.png").unwrap();

}

// #[wasm_bindgen]
// pub fn run_wasm() -> [u8; 1024*1024*4]{
//     let config = Config {height: 3028, width: 3028, iterations: 600000, edges: 5};
//     run(config)

// }


fn edges_of_polygon(edges: u32, diameter: f32) -> Vec<Point>{
    let mut v: Vec<Point> = Vec::new();
    let radius: f32 = diameter / 2.0;
    let angle: f32 = 2.0 * consts::PI / edges as f32;
    let origin = diameter / 2.0;

    for i in 0..edges{
        let internal = angle * i as f32;
        let x = origin + radius * internal.sin();
        let y = origin + radius * internal.cos();
        v.push(Point{x: x as i32, y: y as i32});
    }

    v
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Copy)]
pub struct Config {
    pub height: u32,
    pub width: u32,
    pub iterations: u32,
    pub edges: u32,
    pub factor: f32
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn middle_point(&self, to: &Self, factor: f32) -> Self {
        Self {
            x: ((self.x + to.x) as f32 * factor) as i32,
            y: ((self.y + to.y) as f32 * factor) as i32,
        }
    }

    fn convert_to_index(&self, point: &Self) -> usize {
        // fix hardcoded
        let width = 1024;
        let height = 1024;
        ((height * point.y + point.x) * 4) as usize
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
