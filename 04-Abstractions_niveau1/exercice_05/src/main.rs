enum Command {
    Move {x: i32, y: i32},
    Stop,
}

fn valid_move(cmd: &Command) -> Option<(i32, i32)> {
    let Command::Move { x, y } = cmd else {
        return None;
    };
    if *x >= 0 && *y >= 0 {
        Some((*x, *y))
    } else {
        None
    }
}

/*
Déroulé pour Command::Stop : le pattern Command::Move { x, y } ne correspond pas, donc la branche
else s'exécute et return None; termine immédiatement la fonction — le reste du corps n'est jamais atteint.
 */

// 2 ) Quel avantage apporte let else par rapport à un match classique dans ce cas précis ?

/*
let else permet de garder le chemin "normal" (le cas où le pattern correspond)
au même niveau d'indentation que le reste de la fonction, au lieu d'imbriquer tout le corps dans
une branche match { Variante => { ... } }. Cela traduit directement l'intention
« je continue normalement si ça correspond, sinon je sors tout de suite » c'est plus lisible
qu'un match dès qu'un seul cas nous intéresse vraiment et que l'autre cas se contente de sortir.
 */

fn main() {
    println!("Hello, world!");
}
