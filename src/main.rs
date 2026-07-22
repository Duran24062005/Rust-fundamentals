use std::io;

// This is a main file and entry point.
fn main() {
    let mut program: bool = true;

    println!("=== Hi, wolcome to my Pelis Program ===");

    while program {
        println!("
==============================
=== Please select a option ===
==============================
 1. View all movies.
 2. Search movie by ID.
 3. Search movies by name.
 4. Create a new movie.
 5. Update movie by ID.
 6. Delete movie by ID.
 7. Exit.
        ");

        let mut input: String = String::new();

        io::stdin().read_line(&mut input).expect("Error to read the text.");

        let option: i32 = match input.trim().parse() {
            Ok(num)=>num,
            Err(_)=>{
                println!("❌ Por favor, introduce un número válido.");
                continue; // Reinicia el ciclo para pedir la opción otra vez
            }
        };

        match option {
            1 => println!("Mostrando todas las películas..."),
            2 => println!("Buscando película por ID..."),
            3 => println!("Buscando películas por nombre..."),
            4 => println!("Creando una nueva película..."),
            5 => println!("Actualizando película..."),
            6 => println!("Eliminando película..."),
            7 => {
                println!("Saliendo del programa. ¡Adiós!");
                program = false; // Detiene el ciclo while
            }
            _ => println!("⚠️ Opción no válida. Intenta del 1 al 7."),
        }
    }
}