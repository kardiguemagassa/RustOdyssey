fn main() {

    //## Exercice 2

    /*
    Q1 ) On souhaite créer une variable `p1` pour représenter les coordonnées d'un point dans un
    espace en 3 dimensions, sa valeur en x est `3.5`, en y `4.0` et en z `8.0`. Expliquer quel
    type choisir parmi les types que l'on a déjà vu.

    Q2 ) Une fois `p1` créé, on souhaite récupérer sa coordonnée en x dans une variable `x` et
    sa cordonnée en y dans une variable `y` le tout sur une seule ligne.
    */

    // Coordonnées d'un point en 3 dimensions
    // On peut utiliser un **array** contenant trois flottants car ils sont tous de même type :
    let p1 = [3.5, 4.0, 8.0];

    // Récupération des coordonnées x et y
    let [x, y, _] = p1;

    println!("x = {}", x);
    println!("y = {}", y);



    /*
    Q3 ) On souhaite stocker les informations d'un pantalon dans une variable `pant`, on veut stocker
    le prix `34.00` euros, la taille `38` (entier ) et si c'est un short ou non `true` ou `false` dans
    une seule variable. Expliquer quel type choisir parmi les types que l'on a déjà vu.

    **Réponse :**
    On peut utiliser un tuple car ils ne sont tous de même type:

    Q4 ) Une fois `pant` créé, on souhaite récupérer chacune de ses infos dans une variable dédiée,
    le prix dans `price`, la taille dans `size`, si c'est un short dans `is_short`

    */

    let pant = (34.0, 38, true);
    //Le type est: (f64, u8, bool)
    let (price, size, is_short) = pant;
    println!("pant = {:?}", pant);
    println!("price = {}, size = {}, is_short = {}", price, size, is_short);
}
