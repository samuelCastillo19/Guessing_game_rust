use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // Se genera el numero secreto y se guarda en memoria
    let secret_number = rand::thread_rng().gen_range(1..11);


    // Se le pide al usuario la entrada del dato a comparar
    println!("Please, input your guess.");                                      
    let mut guess = String::new();                                              //Instancia del numero adivinado
    io::stdin()                                                                 //Lectura del dato ingresado
        .read_line(&mut guess)                                                  //Se asigna el dato ingresado a la variable guess
        .expect("Failed to read line");                                         //Manejo de un posibe error por ingresar un dato invalido
    let guess: u32 = guess.trim().parse().expect("Please type a number!");      //Se convierte a int el dato ingresado, siempre y cuando sea un numero


    // Se imprime el numero ingresado y ya convertido a entero
    println!("You guessed: {}", guess);

    // Se hace la comparacion del numero ingresado (guess) con el numero secreto (secret_number)
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
