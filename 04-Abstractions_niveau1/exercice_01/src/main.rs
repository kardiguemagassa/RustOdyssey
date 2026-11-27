struct Rectangle {
    width: f64,
    height: f64,
}

// 1 ) Implémenter une fonction associée new(width: f64, heigh
impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
}

// 2 ) Implémenter une fonction associée square(side: f64) -> Rectangle qui construit un rectangle
// dont la largeur et la hauteur valent side
fn square(side: f64) -> Rectangle {
    Rectangle { width: side, height: side }
}

// 3 ) Implémenter une méthode area(&self) -> f64.
fn area(&self) -> f64 {
    self.width * self.height
}

// 4 ) Implémenter une méthode perimeter(&self) -> f64.
fn perimeter (&self) -> f64 {
    2.0 * (self.width + self.height)
}

// 5 ) Implémenter une méthode is_square(&self) -> bool.
fn is_square (&self) -> bool {
    self.width * self.height != 0.0
}


// 6 ) Quelle est la différence entre une méthode et une fonction associée en Rust ?
/*
Une méthode prend self (ou &self/&mut self) comme premier paramètre : elle s'appelle sur une instance
existante avec la syntaxe instance.methode(), qui est sucrée en Type::methode(&instance).
Une fonction associée n'a pas de paramètre self : elle appartient au type mais ne s'appelle pas sur
une valeur particulière, elle s'invoque avec Type::fonction(...). Elle sert typiquement à construire
une instance (new, square, etc.), un rôle qu'un constructeur jouerait dans d'autres langages.
 */


struct Counter {
    count: i32,
}

// 7 ) Ajouter une constante associée MAX: i32 = 100.
impl Counter {
    const MAX: i32 = 100;
}

// 8 ) Implémenter une fonction associée new() -> Counter qui démarre à 0.
fn new() -> Counter {
    Counter { count: 0 }
}

// 9 ) Implémenter une méthode increment(&mut self) qui ajoute 1 à value, sans jamais dépasser MAX.
fn increment(&mut self) {
    if self.value < Self::MAX {
        self.value += 1;
    }
}

// Le type doit être &mut self (et non &self) car la méthode modifie un champ de l'instance. Self::MAX
// (avec un S majuscule) désigne la constante associée au type courant, indépendamment du nom réel du type.

// 10 ) Implémenter une méthode value(&self) -> i32.
fn value(&self) -> i32 {
    self.value
}

fn main() {
    println!("Hello, world!");
}
