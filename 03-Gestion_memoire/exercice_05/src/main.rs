// fn show_vec(v: Vec<u8>) {
//     dbg!(v);
// }

fn show_vec(v: &Vec<u8>) {
    dbg!(v);
}

// fn show(z: String) {
//     println!("{}", z);
// }

// fn show(v: &Vec<i32>) {
//     dbg!(v);
// }

fn show(v: Vec<i32>) {
    dbg!(v);
}

fn main() {
    //Question 1
    // En Rust, un Move est un transfert de propriété d'une valeur d'une variable vers une autre.

    // let a = String::from("Bonjour");
    // let b = a; // Move : a → b
    //
    // dbg!(b); // ✅ Compile
    // dbg!(a); // ❌ Ne compile pas
    /*
    Explication : String n'implémente pas Copy. Lorsque b reçoit a, la propriété de la chaîne est
    transférée à b. La variable a ne peut plus être utilisée.
    À retenir : un Move transfère la propriété sans recopier les caractères de la chaîne.
     */


    //Question 2 : quels types sont concernés par le Move ?
    //Quels types sont concernés par le Move ?
    // En Rust, lorsqu'on affecte une valeur à une autre variable, si son type n'implémente pas le trait Copy,
    // sa propriété est transférée : c'est un Move.
    // C'est quoi un trait en Rust ?
    // Un trait est un ensemble de comportements qu'un type peut posséder.
    // Il existe aussi d'autres traits en Rust, comme Clone et Debug. Nous les découvrirons progressivement.

    // let a = String::from("Bonjour");
    // let b = a; // Move : a → b


    // Question 3 : exemple 1
    // Pourquoi ?
    // 1. "Bonsoir" est une chaîne littérale de type &str.
    // 2. Le type &str implémente le trait Copy.
    // 3. let other_text = text; copie la référence.
    // 4. text reste utilisable.
    // À retenir pour tes révisions : ici, il n'y a aucun Move, seulement une Copy.
    // Rust copie la référence, pas les caractères de "Bonsoir".
    // let text = "Bonsoir";
    // let other_text = text; // Copy ✅
    // dbg!(text);            // Fonctionne ✅


    // Question 3 — Code n°2
    // Pourquoi ?
    // 1. String::from("Bonsoir") crée une valeur de type String.
    // 2. String n'implémente pas le trait Copy.
    // 3. let other_text = text; transfère la propriété de text vers other_text.
    // 4. dbg!(text) provoque une erreur, car text n'est plus utilisable.
    // let text = String::from("Bonsoir");
    // let other_text = text; // Move : text → other_text
    // dbg!(text);            // ❌ Erreur !


    // Question 3 — Code n°3
    // Pourquoi ce code compile ?
    // 1. text contient une String.
    // 2. let other_text = text; effectue un Move.
    // 3. other_text devient le nouveau propriétaire.
    // 4. text n'est plus utilisable, mais on ne cherche pas à l'utiliser après le Move.
    // C'est pour cela que le code compile !
    // let text = String::from("Bonsoir");
    // let other_text = text; // Move : text → other_text


    // Question 3 — Code n°4
    // x est un vecteur de type Vec<u8>.
    // Vec<u8> n'implémente pas le trait Copy.
    // Quand on appelle show_vec(x), la propriété du vecteur est transférée de x vers le paramètre v.
    // Après cet appel, x n'est plus utilisable.
    // dbg!(x) provoque donc une erreur de compilation.
    // let x: Vec<u8> = vec![1, 2, 3];
    // show_vec(x); // Move : x → v
    // dbg!(x);     // ❌ Erreur !


    // Question 3 — Correction du code n°5
    // Pourquoi ce code compile ?
    // 1. x est un vecteur de type Vec<u8> (ici, Rust déduit u8 grâce au paramètre de la fonction).
    // 2. &x crée une référence vers le vecteur.
    // 3. La fonction show_vec reçoit cette référence, sans prendre la propriété de x.
    // 4. Après l'appel, x reste utilisable.
    // let x = vec![1, 2, 3];
    // show_vec(&x); // ✅ Emprunt de x
    // dbg!(x);      // ✅ Fonctionne


    // Question 3 — Code n°6
    // À retenir pour tes révisions
    // for i in x consomme le vecteur. Pour parcourir ses éléments sans perdre sa propriété,
    // on peut écrire for i in &x.
    // let x = vec![1, 2, 3];
    //
    // for i in x { //un Move du vecteur
    //     println!(i); // ❌ Erreur de syntaxe
    // }
    //
    // println!(x);         // ❌ Erreur de syntaxe + x déplacé


    // Question 3 — Code n°7
    // Pourquoi ?
    // 1. x est un Vec<i32>, qui n'implémente pas Copy.
    // 2. for i in x prend possession du vecteur.
    // 3. Chaque i reçoit un élément du vecteur.
    // 4. dbg!(i) est valide.
    // 5. dbg!(x) est interdit, car x a été déplacé.
    // let x = vec![1, 2, 3];
    //
    // for i in x { // Move de x vers la boucle
    //     dbg!(i); // ✅ Fonctionne
    //     dbg!(x); // ❌ Erreur : x a été déplacé
    // }


    //Question 3 — Code n°8
    // À retenir pour tes révisions
    // - x : le vecteur lui-même.
    // - &x : une référence vers le vecteur.
    // - dbg!(&x) : affiche le vecteur sans en prendre la propriété.
    // let x = vec![1, 2, 3];
    //
    // for i in &x { // Emprunt de x, pas de Move
    //     dbg!(i);  // ✅ Fonctionne
    //     dbg!(&x);  // ✅ Fonctionne
    // }


    // Question 3 — Code n°9
    // Pourquoi ce code compile ?
    // 1. x est un tableau de trois éléments de type u8.
    // 2. Le type u8 implémente Copy.
    // 3. Le tableau [u8; 3] implémente donc également Copy.
    // 4. for i in x utilise une copie du tableau pour parcourir ses éléments.
    // 5. dbg!(x) utilise également une copie de x.
    // Résultat : x reste utilisable pendant toute la boucle.
    // let x: [u8; 3] = [1, 2, 3];
    //
    // for i in x {  // Copy du tableau
    //     dbg!(i);  // ✅ Fonctionne
    //     dbg!(x);  // ✅ Copy de x
    // }


    // Question 3 — Code n°10
    // Pourquoi ce code compile ?
    // 1. a contient la valeur 10, de type i32.
    // 2. i32 implémente le trait Copy.
    // 3. let b = a; copie la valeur de a dans b.
    // 4. a reste utilisable après la copie.
    // Il n'y a aucun Move dans cet exemple.
    // À retenir pour tes révisions
    // Quand un type implémente Copy, on peut copier sa valeur sans perdre la propriété de la variable d'origine.
    let a = 10;
    let b = a; // ✅ Copy de a vers b

    println!("{}", a); // ✅ Fonctionne


    //Question 3 — Code n°11
    // Pourquoi ce code ne compile pas ?
    // 1. s1 contient une valeur de type String.
    // 2. String n'implémente pas le trait Copy.
    // 3. let s2 = s1; transfère la propriété de s1 vers s2.
    // 4. s1 n'est donc plus utilisable.
    // 5. println!("{}", s1) provoque une erreur de compilation.
    // À retenir pour tes révisions
    // Une String n'implémente pas Copy. Lorsqu'on l'affecte à une autre variable, sa propriété est transférée.
    // On ne peut plus utiliser l'ancienne variable.
    // let s1 = String::from("hello");
    // let s2 = s1; // Move : s1 → s2
    //
    // println!("{}", s1); // ❌ Erreur !


    // Question 3 — Code n°12
    // Pourquoi ce code ne compile pas ?
    // 1. s contient une valeur de type String.
    // 2. La fonction show attend un paramètre z: String.
    // 3. Lorsque nous appelons show(s), Rust transfère la propriété de s vers z.
    // 4. À la fin de la fonction, z est détruit et sa mémoire est libérée.
    // 5. s n'est donc plus utilisable dans main.
    // À retenir pour révisions
    // Passer une String à une fonction qui attend un String provoque un Move.
    // La fonction devient propriétaire de la valeur.
    // Pour éviter ce Move, on peut utiliser une référence &String (ou &str).
    // let s = String::from("hello");
    // show(s); //  // Move : s → z
    // println!("{}", s); //  // ❌ Erreur !


    // Correction du code n°13
    // Pourquoi ce code compile ?
    // 1. s est propriétaire de la String "hello".
    // 2. let r = &s; crée une référence vers s.
    // 3. Il n'y a aucun Move de la String.
    // 4. s reste propriétaire de sa valeur.
    // 5. println!("{}", s) fonctionne donc normalement.
    // À retenir pour tes révisions
    // Une référence & permet d'accéder à une valeur sans en prendre la propriété.
    // Attention : &s ne crée pas une deuxième String. Il crée une référence vers la String existante.
    // let s = String::from("hello");
    // let r = &s;   // Emprunt de s
    //
    // println!("{}", s);  // ✅ Fonctionne


    // Question 3 — Code n°14
    // Pourquoi ce code compile ?
    // 1. a1 est un tableau de type [i32; 3].
    // 2. Le type i32 implémente le trait Copy.
    // 3. Le tableau [i32; 3] implémente donc également Copy.
    // 4. let a2 = a1; copie les trois valeurs.
    // 5. a1 reste utilisable.
    // Résultat : le code compile et affiche [1, 2, 3].
    // À retenir pour tes révisions
    // Un tableau implémente Copy si ses éléments implémentent Copy.
    //
    // Attention : un tableau contenant des String, par exemple, n'implémente pas Copy.
    // let a1 = [1, 2, 3];
    // let a2 = a1; // ✅ Copy de a1 vers a2
    //
    // println!("{:?}", a1);  // ✅ Fonctionne


    // Question 3 — Code n°15
    // Pourquoi ce code compile ?
    // 1. v1 est un vecteur de type Vec<i32>.
    // 2. let v2 = v1; effectue un Move, car Vec<i32> n'implémente pas Copy.
    // 3. v1 devient inutilisable, mais v2 est maintenant propriétaire du vecteur.
    // 4. show(&v2) emprunte le vecteur sans prendre sa propriété.
    // 5. dbg!(v2) fonctionne parce que v2 est toujours propriétaire du vecteur.
    // let v1 = vec![1, 2, 3];
    // let v2 = v1; // Move : v1 → v2
    //
    // show(&v2);   // Emprunt de v2
    // dbg!(v2);    // ✅ Fonctionne


    // Question 3 — Code n°16 (dernier code)
    // Pourquoi ce code ne compile pas ?
    // 1. v est un vecteur de type Vec<i32>.
    // 2. Vec<i32> n'implémente pas le trait Copy.
    // 3. La fonction show attend un Vec<i32> directement, pas une référence.
    // 4. show(v) transfère donc la propriété du vecteur à la fonction.
    // 5. Après cet appel, v n'est plus utilisable dans main.
    // À retenir pour tes révisions
    // Passer un Vec par valeur provoque un Move. Le passer par référence avec & permet de conserver sa propriété.

    // let v = vec![1, 2, 3];
    //
    // show(v);  // Move : v (main) → v (show)
    // dbg!(v);  // ❌ Erreur : v a été déplacé

}