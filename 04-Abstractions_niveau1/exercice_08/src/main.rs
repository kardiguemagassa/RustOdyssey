struct Square { side: f64 }
struct Circle { radius: f64 }

fn square_perimeter(s: &Square) -> f64 {
    4.0 * s.side
}

fn circle_perimeter(c: &Circle) -> f64 {
    2.0 * std::f64::consts::PI * c.radius
}

// 1 ) Créer un trait Perimeter avec une méthode perimeter(&self) -> f64, et l'implémenter pour Square et Circle.
trait Perimeter {
    fn perimeter(&self) -> f64;
}

impl Perimeter for Square {
    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }
}

impl Perimeter for Circle {
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

// 2 ) Remplacer square_perimeter et circle_perimeter par une seule fonction générique print_perimeter
// qui affiche le périmètre de n'importe quelle forme implémentant Perimeter.
fn print_perimeter(shape: &impl Perimeter) {
    println!("Perimeter: {:.2}", shape.perimeter());
}
// Les quatre fonctions du départ (square_perimeter, circle_perimeter, et leurs équivalents print_*)
// se réduisent à une seule fonction générique : le symptôme initial était que plusieurs fonctions
// presque identiques ne différaient que par le type de leur paramètre : un signe classique qu'un
// trait doit remplacer la duplication.


// 3 ) Ajouter une struct Triangle (équilatéral) et implémenter Perimeter pour elle. Que dois-tu changer dans print_perimeter ?
struct Triangle { side: f64 }
impl Perimeter for Triangle {
    fn perimeter(&self) -> f64 {
        3.0 * self.side
    }
}
// Rien à changer dans print_perimeter : elle accepte déjà n'importe quel type qui implémente
// Perimeter, Triangle y compris, dès lors que l'impl Perimeter for Triangle existe.


fn main() {
    println!("Hello, world!");
}
