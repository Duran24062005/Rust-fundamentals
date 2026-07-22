// 1. Definición de la estructura (Atributos)
pub struct Dog {
    name: String, // Privado por defecto
    pub age: u32, // Público
}


// 2. Implementación de metodos(Comportamiento)
impl Dog {
    // Constructor (Abstracción)
    pub fn new(name: &str, age: u32) -> Self {
        Dog {
            name: name.to_string(),
            age
        }
    }

    // Metodo de instacia
    pub fn bark(&self) {
        println!("{} say Woof - woof", self.name)
    }
}