/*Q3 ) Créer une constante Pi et l'utiliser dans les fonctions suivante puis citer les avantages
    de cette nouvelle approche par rapport à l'ancienne :
*/

// const PI: f64 = 3.141592653589793;
const PI: f64 = std::f64::consts::PI;

fn main() {
    fn circle_area(radius: f64) -> f64 {
        PI * radius * radius
    }

    fn sphere_volume(radius: f64) -> f64 {
        (4.0 / 3.0) * PI * radius * radius * radius
    }

    println!("circle area = {}", circle_area(45.0));
    println!("sphere volume = {}", sphere_volume(45.0));
}
