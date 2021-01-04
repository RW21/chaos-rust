extern crate image;
use rand::Rng;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::f32::consts;
use std::fmt;

pub fn run(config: Config){
    let mut imgbuf = image::ImageBuffer::new(config.height, config.width);

    // set background color
    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([0 as u8, 0 as u8, 0 as u8]);
    }

    let edges = edges_of_polygon(config.edges,
        config.width as f32);
    println!("{:?}", edges);

    // default color
    let color = image::Rgb([255, 255, 255]);

    let pool = ThreadPool::new(num_cpus::get());

    let (tx, rx) = channel();
    let workload = config.height * config.width / num_cpus::get() as u32;

    for _i in 0..num_cpus::get(){
        let tx = tx.clone();
        let config = config.clone();
        let edges = edges.clone();
        pool.execute(move ||{
            let mut current_point = Point{x: config.width / 2, y: config.height /2};
            let mut rng = rand::thread_rng();
            for _j in 0..workload {
                current_point = current_point.middle_point(&edges[rng.gen_range(0..config.edges as i32) as usize]);
                tx.send(current_point).expect("Threading failed");
        }})}

    for _i in 0..config.iterations{
        let point = rx.recv().unwrap();
        imgbuf.put_pixel(point.x, point.y, color)
    }

    imgbuf.save("test.png").unwrap();

}

fn edges_of_polygon(edges: u32, diameter: f32) -> Vec<Point>{
    let mut v: Vec<Point> = Vec::new();
    let radius: f32 = diameter / 2.0;
    let angle: f32 = 2.0 * consts::PI / edges as f32;
    let origin = diameter / 2.0;

    for i in 0..edges{
        let internal = angle * i as f32;
        let x = origin + radius * internal.sin();
        let y = origin + radius * internal.cos();
        v.push(Point{x: x as u32, y: y as u32});
    }

    v
}

#[derive(Clone)]
pub struct Config {
    pub height: u32,
    pub width: u32,
    pub iterations: u32,
    pub edges: u32,
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn middle_point(&self, to: &Self) -> Self {
        Self {
            x: (self.x + to.x) / 2,
            y: (self.y + to.y) / 2,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
