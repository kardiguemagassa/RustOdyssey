enum Config {
    Timeout (u32),
    Verbose,
    Name(String),
}

// 1 ) Implémenter une fonction print_timeout(config: &Config) qui affiche "Timeout: X" si config
// est la variante Timeout, et "No timeout set" sinon, en utilisant if let.
fn print_timeout(config: &Config) {
    if let Config::Timeout(t) = config {
        println!("Timeout: {}", t);
    } else {
        println!("No timeout set");
    }
}

// 2 ) Dans quel cas préfère-t-on if let à un match complet ?

/*
if let est préférable quand on ne s'intéresse réellement qu'à un seul cas parmi plusieurs, et que tous 
les autres cas partagent le même traitement générique. Écrire un match avec une branche par 
variante juste pour regrouper toutes les autres dans un comportement identique ajoute du bruit sans 
apporter d'information supplémentaire. Ici, seule la variante Timeout a un traitement spécifique ; 
Verbose et Name tombent tous deux dans le même "sinon", donc if let ... else exprime directement 
cette intention.
 */

fn main() {
    println!("Hello, world!");
}
