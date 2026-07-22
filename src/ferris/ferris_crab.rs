fn main() {
    // Definición del cangrejo como array de strings
    let crab = vec![
        "                R  RR  RR           ",
        "            R  RRRRRRRRRR  R        ",
        "        R  RR        R  RRRRRRRRRRRRR  R       R       RR     ",
        "      R  RRR       R  RRRRRRRRRRRRRRR  R    RRR  R    ",
        "    RRRRR    RRRRRRRRRRRRRRRRRRRRRRRRRR   RRRR       ",
        "  RRR  RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR RRRR        ",
        "    R   RRRRRRRRRR=  RR  =  RRRRRRRRRR               ",
        "      RRRRRRRRRRR=    RR    =  RRRRRRRRRR             ",
        "        RRRRRRRRRR=  RR  =  RRRRRRRRRR              ",
        "          RR=RRRRRRRRRRRRRRRRRRRRR==RR              ",
        "          RR  =  RRRRRRRR    RRRRRR=  =  RR         ",
        "            RR  =             ==========      =  RR  ",
        "              R                                    R  ",
        "              R                                    R  ",
        "              R                                      ",
        "              R                                      ",
    ];
    
    print_crab(&crab);
}

/// Imprime el cangrejo (Ferris mascota de Rust)
fn print_crab(lines: &[&str]) {
    println!("🦀 Ferris - Mascota de Rust 🦀\n");
    for line in lines {
        println!("{}", line);
    }
}

/// Versión con colores (códigos ANSI)
#[allow(dead_code)]
fn print_crab_colored(lines: &[&str]) {
    const RED: &str = "\x1b[31m";    // Rojo para 'R'
    const YELLOW: &str = "\x1b[33m"; // Amarillo para '='
    const RESET: &str = "\x1b[0m";
    
    println!("{}🦀 Ferris - Mascota de Rust 🦀{}\n", RED, RESET);
    
    for line in lines {
        let colored = line
            .replace('R', &format!("{}R{}", RED, RESET))
            .replace('=', &format!("{}={}", YELLOW, RESET));
        println!("{}", colored);
    }
}