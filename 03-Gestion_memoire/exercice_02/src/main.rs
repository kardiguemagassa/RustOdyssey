fn double(x: usize)-> &usize {
    let y = x*2;
    // À la fin de double, la variable locale y cesse d'exister.
    &y
}

fn main() {

    // 1 ) Pour chaque cas suivant décrire ce qui se passe au niveau de la mémoire :
    // let x: u8 = 7;
    // let y: u8 = x;
    //
    // dbg!("X", x, "Y", y);

    // let x: bool = true;
    // let y: bool = x;
    // dbg!(x, y);

    // let x: [u8; 3] = [1, 2, 3];
    // let y: [u8; 3] = x;
    // dbg!(y);
    // dbg!(y);

    // Un tuple permet de regrouper plusieurs valeurs de types différents :
   // let x: (u8, bool, char) = (1, false, 'c');
   // let y: (u8, bool, char) = x;
   // dbg!((x, y));

   // 2 ) Quel est l’intérêt d’utiliser une référence
    // Pour chaque exemple, donner la taille en mémoire de y et expliquer pourquoi :



    // 3 ) Pour chaque exemple, donner la taille en mémoire de y et expliquer pourquoi :
    // les références (&)
    // x contient la valeur 99.
    // &x crée une référence vers x.
    // y contient cette référence, et non une copie de la valeur 99.
    // let x: u8 = 99;
    // let y: &u8 = &x;
    // println!("x: {}, y: {}", x, y);
    //

    // let x: bool = true;
    // let y: bool = x;
    // dbg!("x: {}, y: {}", x, y);


    // let x: [u8;3] = [23, 17, 8];
    // let y: [u8;3] = x;
    // dbg!("x: {}, y: {}", x, y);


    // La vraie différence, c'est que la copie crée un deuxième tableau,
    // tandis que la référence permet d'accéder au tableau existant.
    // Un seul tableau, accessible par y
    // let x: [u8;3] = [23, 17, 8];
    // let y: &[u8;3] = &x;
    // dbg!("x: {}, y: {}", x, y);


   // 4 ) Pour chaque cas, dire si le code compile ou non et expliquer pourquoi,
    // si le code ne compile pas, on le corrigera :

    // Cas 1
    // let x = 10;  // x est immuable. En Rust, une variable est immuable par défaut.
    // let  r = &mut x; // ❌ On demande une référence mutable
    // *r = 20; // ❌ On essaie de modifier x à travers r
    // dbg!(x);

    // Cas 2
    // let x = 8; //x contient 8 et est immuable.
    // let r = &x; // r est une référence qui permet de lire x.
    // let y = *r + 2; // ✅ On lit une valeur et on calcule
    // dbg!(y);

    // Cas 3 sur 5
    // let mut x = 10;
    // let r1 = &mut x; // &mut x crée une référence mutable vers x.
    // *r1 += 5; // *r1 += 5 ajoute 5 à la valeur de x. Donc x passe de 10 à 15.
    // let r2 = &x;
    // // On ne peut pas utiliser simultanément une référence mutable (&mut)
    // // et une référence immuable (&) vers la même donnée.
    // println!("r1 is {}", r1);


    // Cas 4 sur 5
    // let mut x = 10;
    // let r1 = &mut x;
    // *r1 += 5;
    // let r2 = &x;
    // dbg!(r2);

    // Cas 4 sur 5
    double(13);

}
