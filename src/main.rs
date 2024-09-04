use vec3::Vec3;

fn main() {
    let image_height = 256;
    let image_width = 256;

    println!("P3");
    println!("{} {} {}", image_width, image_height, 255);

    for j in 0..image_height {
        for i in 0..image_width {
            let r = (i as f32) / ((image_width - 1) as f32);
            let g = (j as f32) / ((image_height - 1) as f32);
            let b = 0.0;

            write_color(Vec3::new_vector(r, g, b));
        }
    }
}

fn write_color(v: Vec3<f32>) {
    let vv = v * 255.999;
    println!("{} {} {}", *vv.x() as i32, *vv.y() as i32, *vv.z() as i32);
}
