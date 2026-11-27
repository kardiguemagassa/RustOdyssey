enum Shape {
    Circle {radius: f64},
    Rectangle {width: f64, height: f64},
    Triangle {base: f64, height: f64},
}

//1 ) Implémenter une fonction area(shape: &Shape) -> f64 qui calcule l'aire de la forme avec match.
fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rectangle { width, height } => width * height,
        Shape::Triangle { base, height } => 0.5 * base * height,
    }
}

// 2 ) Pourquoi ce match doit-il être exhaustif ?
/*
Rust exige que chaque match couvre tous les cas possibles de la valeur analysée, pour garantir
qu'aucune variante ne soit oubliée, c'est une vérification faite à la compilation, pas à l'exécution.
Comme Shape n'a que trois variantes et que les trois sont traitées, le compilateur accepte le match
sans avoir besoin d'une branche _ ("tous les autres cas"). Si une nouvelle variante était ajoutée
à Shape sans mettre à jour ce match, la compilation échouerait, c'est en réalité un avantage
car ça évite d'oublier de gérer un nouveau cas.
 */

// 3 ) Que retourne area(&Shape::Circle { radius: 2.0 }) ?
//area = π * radius * radius = 3.14159... * 2.0 * 2.0 ≈ 12.57


fn main() {
    println!("Hello, world!");
}
