struct Excerpt<'a> {
    text: &'a str,
}

// 1 ) Pourquoi ce struct a-t-il besoin d'une annotation de lifetime 'a ?

// Le champ text est une référence (&str). Une référence n'est valide que tant que la donnée qu'elle
// pointe existe encore. Sans annotation, le compilateur ne pourrait pas garantir qu'au moment où l'on utilise Excerpt, la string référencée par text soit toujours vivante. Le lifetime 'a rend cette contrainte explicite : « une instance de Excerpt ne peut pas vivre plus longtemps que la donnée pointée par son champ text ».


// 2 ) Implémenter une fonction associée new(source: &'a str) -> Excerpt<'a>.
impl<'a> Excerpt<'a> {
    fn new(source: &'a str) -> Excerpt<'a> {
        Excerpt { text: source }
    }
}

// 3 ) Implémenter une méthode first_word(&self) -> &str qui retourne le premier mot de text.
fn first_word(&self) -> &str {
    self.text.split_whitespace().next().unwrap_or("")
}

// Ici, le lifetime de retour est élidé (les règles d'élision de lifetime permettent d'omettre
// 'a explicitement) mais il correspond bien au lifetime de &self, donc indirectement à celui de text.

// 4 ) Le code suivant ne compile pas. Expliquer pourquoi.
fn create_excerpt() -> Excerpt<'static> {
    let sentence = String::from("temporary");
    Excerpt::new(&sentence)
}

// 5 ) Implémenter une fonction longest<'a>(a: &'a str, b: &'a str) -> &'a str qui retourne
// la plus longue des deux chaînes.
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// 6 ) Si on appelle longest(x, y) où x a un lifetime plus long que y, quel est le lifetime de la valeur
// retournée, du point de vue du compilateur ?
//
// La signature fn longest<'a>(a: &'a str, b: &'a str) -> &'a str unifie les lifetimes de a, b et du retour
// sous un seul paramètre 'a. Le compilateur choisit alors pour 'a le plus court des deux
// lifetimes fournis en entrée, car c'est la seule durée compatible avec les deux paramètres à la fois.
// Résultat : même si la valeur effectivement retournée est x (qui vit plus longtemps),
// le compilateur considère la référence retournée comme valide seulement aussi longtemps que y
// (la plus courte des deux) car il ne raisonne que sur la signature, pas sur le corps de la fonction.

fn main() {
    println!("Hello, world!");
}
