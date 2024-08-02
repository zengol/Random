// Importa la biblioteca rand, que proporciona funcionalidades para generar números aleatorios.
extern crate rand;

// Importa el trait Rng de la biblioteca rand, que permite generar números aleatorios.
use rand::Rng;

// La función principal del programa, donde comienza la ejecución.
fn main() {
    // Crea un generador de números aleatorios utilizando el generador de hilo actual.
    // `thread_rng()` devuelve un generador de números aleatorios local al hilo.
    let random_number = rand::thread_rng().gen_range(1..101);
    
    // Imprime el número aleatorio generado en la consola.
    // `println!` es una macro que imprime texto en la salida estándar.
    println!("Random number: {}", random_number);
}
