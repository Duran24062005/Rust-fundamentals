<div align="center">
    <img src="https://miro.medium.com/v2/resize:fit:4800/format:webp/0*Eqqrv9zVpH99X726.png" />
    <h1>Rust Fundamentals</h1>
</div>

## ¿Qué es Rust?

**Rust** es un lenguaje de programación de sistemas creado por Mozilla e impulsado actualmente por la **Rust Foundation**. Fue diseñado para desarrollar software **rápido, seguro y confiable**, evitando muchos de los errores comunes de lenguajes como C y C++, especialmente los relacionados con la gestión de memoria.

Es uno de los lenguajes más apreciados por desarrolladores y se utiliza en sistemas operativos, navegadores, videojuegos, herramientas CLI, servidores, blockchain y sistemas embebidos.

---

## ¿Por qué se creó Rust?

Los lenguajes de bajo nivel como **C** y **C++** ofrecen un rendimiento excelente, pero presentan problemas frecuentes como:

* Fugas de memoria (Memory Leaks)
* Punteros inválidos (Dangling Pointers)
* Buffer Overflows
* Errores de concurrencia (Race Conditions)

Rust busca mantener el rendimiento de C/C++ eliminando la mayoría de estos problemas **desde la compilación**, sin utilizar un recolector de basura (Garbage Collector).

---

## Características principales

### 🚀 Alto rendimiento

Rust compila directamente a código máquina, por lo que su velocidad es comparable con C y C++.

Ideal para:

* Sistemas operativos
* Motores de videojuegos
* Bases de datos
* Navegadores
* Redes

---

### 🔒 Seguridad de memoria

Su característica más famosa.

Rust garantiza que:

* No accedes a memoria liberada.
* No existen referencias inválidas.
* No hay dobles liberaciones de memoria.

Todo esto sin usar un Garbage Collector.

---

### ⚡ Concurrencia segura

Crear aplicaciones con múltiples hilos suele ser complicado.

Rust detecta muchos errores de concurrencia **antes de ejecutar el programa**.

---

### 📦 Cargo

Rust incluye uno de los mejores gestores de paquetes del ecosistema.

Con Cargo puedes:

* Crear proyectos
* Instalar dependencias
* Compilar
* Ejecutar pruebas
* Publicar librerías

Ejemplo:

```bash
cargo new mi_proyecto
cd mi_proyecto
cargo run
```

---

## ¿Cómo administra la memoria?

En lugar de usar:

* Garbage Collector (Java, Go)
* `malloc` y `free` (C)

Rust utiliza un sistema llamado **Ownership**.

Existen tres reglas básicas:

1. Cada dato tiene un único dueño.
2. Cuando el dueño desaparece, la memoria se libera automáticamente.
3. Solo puede existir un dueño a la vez.

Gracias a estas reglas, Rust evita una gran cantidad de errores.

---

## Primer programa

```rust
fn main() {
    println!("Hola Mundo");
}
```

Salida:

```
Hola Mundo
```

---

## Variables

Por defecto son inmutables.

```rust
fn main() {
    let nombre = "Alex";
    println!("{}", nombre);
}
```

Variables mutables:

```rust
let mut edad = 20;
edad += 1;
```

---

## Tipos de datos

### Enteros

```rust
let x: i32 = 10;
let y: u64 = 200;
```

### Decimales

```rust
let precio: f64 = 19.99;
```

### Booleanos

```rust
let activo = true;
```

### Caracteres

```rust
let letra = 'A';
```

### Strings

```rust
let nombre = String::from("Rust");
```

---

## Funciones

```rust
fn sumar(a:i32, b:i32) -> i32 {
    a + b
}

fn main() {
    let resultado = sumar(5,3);
    println!("{}", resultado);
}
```

---

## Condicionales

```rust
let edad = 20;

if edad >= 18 {
    println!("Mayor");
} else {
    println!("Menor");
}
```

---

## Bucles

### loop

```rust
loop {
    println!("Hola");
}
```

---

### while

```rust
let mut i = 0;

while i < 5 {
    println!("{}", i);
    i += 1;
}
```

---

### for

```rust
for i in 1..6 {
    println!("{}", i);
}
```

---

## Vectores

```rust
let numeros = vec![1,2,3,4];

println!("{}", numeros[0]);
```

Agregar elementos:

```rust
let mut numeros = Vec::new();

numeros.push(10);
numeros.push(20);
```

---

## Structs

Parecidos a clases pero sin herencia.

```rust
struct Persona {
    nombre: String,
    edad: u8,
}

fn main() {
    let p = Persona{
        nombre:String::from("Ana"),
        edad:22,
    };

    println!("{}", p.nombre);
}
```

---

## Enums

```rust
enum Estado {
    Activo,
    Inactivo,
}
```

Muy usados junto con `match`.

---

## Match

Similar a `switch`, pero más potente.

```rust
let numero = 2;

match numero {
    1 => println!("Uno"),
    2 => println!("Dos"),
    _ => println!("Otro"),
}
```

---

## Manejo de errores

Rust evita las excepciones tradicionales.

Utiliza principalmente:

### Option

Representa un valor que puede existir o no.

```rust
let x = Some(5);
let y: Option<i32> = None;
```

---

### Result

Representa éxito o error.

```rust
fn dividir(a:f64,b:f64)->Result<f64,String>{
    if b==0.0{
        Err("No se puede dividir por cero".to_string())
    }else{
        Ok(a/b)
    }
}
```

---

## ¿Dónde se utiliza Rust?

Actualmente Rust se utiliza en proyectos como:

* Sistemas operativos
* Motores de videojuegos
* Compiladores
* Herramientas CLI
* Servicios backend
* Blockchain (Solana, Polkadot)
* WebAssembly
* Sistemas embebidos
* Ciberseguridad
* Redes de alta velocidad
* Bases de datos

Grandes empresas como Microsoft, Amazon, Google, Cloudflare y Discord lo utilizan en partes de su infraestructura para obtener un mejor equilibrio entre rendimiento y seguridad.

---

## Ventajas

* Muy rápido.
* Excelente seguridad de memoria.
* Sin Garbage Collector.
* Muy buen sistema de paquetes (Cargo).
* Gran documentación.
* Excelente compilador con mensajes de error detallados.
* Muy adecuado para programación concurrente.

---

## Desventajas

* Curva de aprendizaje más pronunciada que Python o JavaScript, especialmente por conceptos como *ownership*, *borrowing* y *lifetimes*.
* Los tiempos de compilación pueden ser mayores en proyectos grandes.
* Menor cantidad de bibliotecas en algunos nichos comparado con lenguajes más antiguos.

---

## ¿Vale la pena aprender Rust?

Depende de tus objetivos:

* **Sí**, si te interesa el desarrollo de sistemas, herramientas de alto rendimiento, backend eficiente, WebAssembly, ciberseguridad o programación embebida.
* **No es la primera opción** si tu enfoque principal es ciencia de datos o aprendizaje automático. En esos campos, **Python** sigue siendo el lenguaje dominante gracias a su enorme ecosistema (NumPy, Pandas, PyTorch, TensorFlow, OpenCV, etc.).

Dado que actualmente que estoy retomando **Python para Machine Learning y Visión por Computadora**, debo **priorizar Python hasta dominarlo**. Una vez tenga una base sólida, aprender Rust puede ser una excelente inversión para desarrollar herramientas de alto rendimiento, integrar código con Python mediante extensiones o trabajar en sistemas donde el rendimiento y la seguridad sean críticos.

- [Documentation](https://rust-lang.org/)
- [Video Tutorial by MOure Dev](https://www.youtube.com/watch?v=GWprpnIG-w4)

```
19/06/2024 - By Alexi Dg
```