mod geometry;

fn main() {
    geometry::draw::draw_triangle(4);

    let area = geometry::math::triangle_area(4.0);

    println!("Area = {}", area);
}
