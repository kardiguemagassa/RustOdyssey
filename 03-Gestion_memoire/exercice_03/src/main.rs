fn main() {

    // cas 1
    // let a: u8 = 14;
    // let b: &u8 = &a;
    // dbg!(b)

    // //cas:2
    // À retenir : a stocke la valeur 14, tandis que b stocke l'adresse mémoire de a.
    // fn double(x: u8) -> usize {
    //     //...
    // }
    //
    // fn double_with_ref(x: &u8) -> usize {
    //     //...
    // }

    // // cas 3
    // Cette version est préférable parce que :
    // - Un char occupe seulement 4 octets.
    // - Un char implémente Copy, donc sa copie est peu coûteuse.
    // - Une référence &char occupe 8 octets sur une architecture 64 bits.
    // Conclusion : pour un petit type comme char, il est généralement
    // préférable de passer directement la valeur.

    // fn is_vowel(c: char) -> bool {
    //     //...
    // }
    //
    // fn is_vowel_ref(c: &char) -> bool {
    //     //...
    // }

    // cas 4
    fn mean_with_ref(x: &[usize]) -> usize {
        //...
    }

    fn mean(x: [usize; 4]) -> usize {
        //...
    }

}
