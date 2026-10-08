fn main() {
    /* Q1
      L'expression x > 23 est une comparaison.
       Comme 7 > 23 est faux, Rust produit la valeur false, de type bool.
       Résultat : y est un bool qui vaut false.
    */
    let x: u8 = 7;
    let y = { x > 23 };
    println!("y = {}", y);

    /*Q2
     1. x vaut 19.
     2. x > 20 est faux, donc le if produit 8.
     3. Le ; après le if fait que cette valeur n'est pas retournée par le bloc.
     4. y est donc de type () et sa valeur est ().
     5. Le deuxième x, qui vaut 12, existe uniquement dans son bloc et ne modifie pas le premier.
     Une précision : si tu exécutes exactement ce code, rien ne s'affichera,
     car il ne contient aucun println!(). Rust pourra simplement signaler des avertissements pour
     les variables inutilisées.
    */
    let x: u16 = 19;
    let y = {
        if x > 20 {
            44
        } else {
            8
        };
    };
    println!("y = {:?}", y);

    {
        let x = 12;
        println!("x = {}", x);
    }

    // Q3
    let x: u8 = 4;

    let y = { if x < 8 && x > 2 { false } else { true } };

    println!("Q3 : y = {}", y);

    // 2 ) Dans les exemples suivant, effacer les `return` quand c'est possible :
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn is_positive(x: i32) -> bool {
        x > 0
    }

    fn compute(x: i32) -> i32 {
        if x > 10 { x * 2 } else { x + 2 }
    }

    println!("add = {}", add(10, 5));
    println!("is_positive = {}", is_positive(-3));
    println!("compute(15) = {}", compute(15));
    println!("compute(5) = {}", compute(5));

    // Version plus propre :
    // fn is_positive(x: i32) -> bool {
    //     x > 0
    // }
    //
    // fn compute(x: i32) -> i32 {
    //     if x > 10 {
    //         x * 2
    //     } else {
    //         x + 2
    //     }
    // }
}
