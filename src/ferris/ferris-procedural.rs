fn main() {
    println!("🦀 Ferris ASCII Art Generator 🦀\n");
    
    // Generar y mostrar el cangrejo
    let crab = generate_crab();
    display_crab(&crab);
    
    // Mostrar versión con colores
    println!("\n=== Versión Coloreada ===\n");
    display_crab_colored(&crab);
}

/// Estructura que representa una línea del cangrejo
#[derive(Debug, Clone)]
struct CrabLine {
    indent: usize,
    content: String,
}

impl CrabLine {
    fn new(indent: usize, content: &str) -> Self {
        CrabLine {
            indent,
            content: content.to_string(),
        }
    }
    
    fn render(&self) -> String {
        format!("{}{}", " ".repeat(self.indent), self.content)
    }
}

/// Genera el cangrejo usando lógica pura
fn generate_crab() -> Vec<CrabLine> {
    vec![
        CrabLine::new(16, "R  RR  RR"),
        CrabLine::new(12, "R  RRRRRRRRRR  R"),
        CrabLine::new(8, "R  RR        R  RRRRRRRRRRRRR  R       R       RR"),
        CrabLine::new(6, "R  RRR       R  RRRRRRRRRRRRRRR  R    RRR  R"),
        CrabLine::new(4, "RRRRR    RRRRRRRRRRRRRRRRRRRRRRRRRR   RRRR"),
        CrabLine::new(2, "RRR  RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR RRRR"),
        CrabLine::new(4, "R   RRRRRRRRRR=  RR  =  RRRRRRRRRR"),
        CrabLine::new(6, "RRRRRRRRRRR=    RR    =  RRRRRRRRRR"),
        CrabLine::new(8, "RRRRRRRRRR=  RR  =  RRRRRRRRRR"),
        CrabLine::new(10, "RR=RRRRRRRRRRRRRRRRRRRRR==RR"),
        CrabLine::new(10, "RR  =  RRRRRRRR    RRRRRR=  =  RR"),
        CrabLine::new(12, "RR  =             ==========      =  RR"),
        CrabLine::new(14, "R                                    R"),
        CrabLine::new(14, "R                                    R"),
        CrabLine::new(14, "R"),
        CrabLine::new(14, "R"),
    ]
}

/// Muestra el cangrejo
fn display_crab(crab: &[CrabLine]) {
    for line in crab {
        println!("{}", line.render());
    }
}

/// Muestra el cangrejo con colores ANSI
fn display_crab_colored(crab: &[CrabLine]) {
    const RED: &str = "\x1b[31m";      // Rojo para 'R'
    const YELLOW: &str = "\x1b[33m";   // Amarillo para '='
    const CYAN: &str = "\x1b[36m";     // Cyan para espacios especiales
    const RESET: &str = "\x1b[0m";
    
    for line in crab {
        let colored_content = line.content
            .replace('R', &format!("{}R{}", RED, RESET))
            .replace('=', &format!("{}={}", YELLOW, RESET));
        
        println!("{}{}", " ".repeat(line.indent), colored_content);
    }
}

/// Función alternativa: Construir cangrejo de forma más dinámica
#[allow(dead_code)]
fn generate_crab_dynamic() -> Vec<String> {
    let mut lines = Vec::new();
    
    // Usar un enfoque más generativo
    let pattern_specs = vec![
        (16, "R  RR  RR"),
        (12, "R  RRRRRRRRRR  R"),
        (8, "R  RR        R  RRRRRRRRRRRRR  R       R       RR"),
        (6, "R  RRR       R  RRRRRRRRRRRRRRR  R    RRR  R"),
        (4, "RRRRR    RRRRRRRRRRRRRRRRRRRRRRRRRR   RRRR"),
        (2, "RRR  RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR RRRR"),
        (4, "R   RRRRRRRRRR=  RR  =  RRRRRRRRRR"),
        (6, "RRRRRRRRRRR=    RR    =  RRRRRRRRRR"),
        (8, "RRRRRRRRRR=  RR  =  RRRRRRRRRR"),
        (10, "RR=RRRRRRRRRRRRRRRRRRRRR==RR"),
        (10, "RR  =  RRRRRRRR    RRRRRR=  =  RR"),
        (12, "RR  =             ==========      =  RR"),
        (14, "R                                    R"),
        (14, "R                                    R"),
        (14, "R"),
        (14, "R"),
    ];
    
    for (indent, content) in pattern_specs {
        lines.push(format!("{}{}", " ".repeat(indent), content));
    }
    
    lines
}

/// Función para modificar el cangrejo dinámicamente
#[allow(dead_code)]
fn transform_crab(crab: &[CrabLine], transform: fn(&str) -> String) -> Vec<String> {
    crab.iter()
        .map(|line| {
            let rendered = line.render();
            transform(&rendered)
        })
        .collect()
}