//Q1
fn square(x: u64) -> u64 {
    x * x
}

//Q2
fn mean(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

fn main() {
    //## Exercice 4

    /* 1 ) On a une fonction `square` qui calcul le carré d'un nombre, on souhaite appeler cette
    fonction  avec `n`, compléter le code pour y parvenir.*/

    /*Ici, tu convertis la valeur de n en u64 pour la passer à square().
    Point important : cette conversion ne modifie pas n. La variable reste de type u16.

    1. n contient la valeur 100, de type u16.
    2. n as u64 produit la valeur 100, mais de type u64.
    3. Cette valeur u64 est transmise à square().
    4. La variable n, elle, reste de type u16.
    Avant et pendant l'appel
    Variable originale
    100
    u16
    n
    Valeur convertie
    100
    u64
    n as u64
    Seule la valeur convertie est envoyée à square()
     */
    let n: u16 = 100;
    let result = square(n as u64);
    println!("The square of {} is {}", n, result);

    // 2 ) Même chose ici avec `x` et `y` et la fonction `mean` qui calcul la moyenne de 2 nombres flottants.
    // Même principe
    let x: i32 = 10;
    let y: i32 = 20;
    let result = mean(x as f64, y as f64);
    println!("Mean is {}", result);
}
