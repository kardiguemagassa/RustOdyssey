struct Dog { name: String }
impl Dog {
    fn noise(&self) -> String {
        format!("{} says Woof!", self.name)
    }
}

struct Cat { name: String }
impl Cat {
    fn noise(&self) -> String {
        format!("{} says Meow!", self.name)
    }
}

// 1 ) Peut-on écrire une seule fonction make_noise qui accepte à la fois &Dog et &Cat sans rien
// changer d'autre au code ? Justifier.

/*
Non. &Dog et &Cat sont deux types distincts sans aucune relation déclarée entre eux. Une fonction typée fn make_noise(animal: &Dog) refusera catégoriquement un &Cat à la compilation, et inversement. Pour accepter les deux types de façon interchangeable, il faut un point commun explicite, c'est le rôle d'un trait.

2 ) Créer un trait Animal avec une méthode noise(&self) -> String, et l'implémenter pour Dog et Cat.
 */
trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Dog {
    fn noise(&self) -> String {
        format!("{} Says Woof!", self.name)
    }
}

impl Animal for Cat {
    fn noise(&self) -> String {
        format!("{} Says Meow!", self.name)
    }
}

// 3 ) Écrire une fonction générique make_noise qui accepte n'importe quel type implémentant Animal.
fn make_noise <T: Animal>(animal: &T) {
    println!("{}", animal.noise());
}
// Le compilateur applique la monomorphisation : pour chaque type concret utilisé lors d'un appel,
// il génère une version spécialisée de la fonction, sans coût d'indirection à l'exécution.


// 4 ) Ajouter une struct Cow (avec un cri "Moo!") et implémenter Animal pour elle, sans modifier make_noise.
struct Cow { name: String }
impl Animal for Cow {
    fn noise(&self) -> String {
        format!("{} says Moo!", self.name)
    }
}
// make_noise n'a besoin d'aucune modification : dès qu'un type implémente Animal,
// il fonctionne automatiquement avec cette fonction. C'est l'intérêt principal du trait :
// permettre d'ajouter du code sans avoir à modifier ce qui existe déjà.


// 5 ) Ajouter au trait Animal une méthode par défaut introduce(&self) -> String qui utilise noise().
trait Animals {
    fn noise(&self) -> String;

    fn introduce(&self) -> String {
        format!("I make this sound: {}", self.noise())
    }
}
// Une méthode par défaut est fournie directement dans la définition du trait ;
// les types qui implémentent Animal en héritent automatiquement sans avoir à la réécrire,
// mais peuvent la redéfinir explicitement s'ils le souhaitent.


// 6 ) Quel symptôme dans le code de départ indique qu'un trait est nécessaire ?

/*
Deux structs différentes (Dog, Cat) définissaient une méthode de même nom et de même signature
(noise(&self) -> String), sans aucun lien formel entre elles. Dès que plusieurs types indépendants
dupliquent la même signature de méthode, ou que l'on anticipe le besoin d'une fonction générique
capable de les traiter de façon interchangeable, c'est le signal qu'il faut extraire un trait plutôt
que de continuer à dupliquer du code type par type.
 */

fn main() {
    println!("Hello, world!");
}
