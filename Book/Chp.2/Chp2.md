# Chp.2

## Strong Values with Variables

```rust
    let mut guess = String::new() 
```

- ```let mut guess``` : introduce mutable variable named ```guess``` (the basic setting is immutable)

- ```::new```: ```::``` syntax indicates that ```new``` is an associated function of the ```String``` type.
(An associated function is a function defined for a specific type. These functions do not belong to a specific instance, but to the type itself.)

```rust 
    io::stdin().read_line(&mut guess).expect("Failed to read line");
```

- ```.read_line(&mut guess)``` calls the ```read_line``` method on the standard input handle to get input from the user. 

- ```&``` indicates that this argument is a reference, which gives you **a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times**. References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references. You don’t need to know a lot of those details to finish this program. For now, all you need to know is that, like variables, references are immutable by default.