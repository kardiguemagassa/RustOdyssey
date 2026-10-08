## Exercice 5

1 ) Dans les exemples suivants donner le type et la valeur de retour de `y` :

- ```rust
  fn main() {
    let x: u8 = 7;
    let y = {
      x > 23
    };
  }
  ```

  **Type :** `bool`

  **Valeur :** `false`

- ```rust
  fn main() {
    let x: u16 = 19;
    let y = {
      if x > 20 {44} else {8};
    };
  
    {
      let x = 12;
    }
  }
  ```

  **Type :** `()`

  **Valeur :** `()`

  Le point-virgule après le `if` transforme l'expression en instruction.

- ```rust
  fn main() {
    let x: u8 = 4;
    let y = {
      if x<8 && x>2 {false} else {true}
    };
  }
  ```

  **Type :** `bool`

  **Valeur :** `false`



2 ) Dans les exemples suivant, effacer les `return` quand c'est possible :

- ```rust
  fn add(a: i32, b: i32) -> i32 {
      a + b
  }
  ```

- ```rust
  fn is_positive(x: i32) -> bool {
      if x > 0 {
          true
      } else {
          false
      }
  }
  ```

  Version plus propre :

  ```rust
  fn is_positive(x: i32) -> bool {
      x > 0
  }
  ```

- ```rust
  fn compute(x: i32) -> i32 {
      if x > 10 {
          x * 2
      } else {
          x + 2
      }
  }
  ```