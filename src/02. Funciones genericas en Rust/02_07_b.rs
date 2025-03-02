/*
 * Curso: Domina Rust: Genéricos
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

 fn localiza_el_minimo<T: PartialOrd>(elementos: &[T]) -> Option<&T> {
    if elementos.is_empty() {
        return None;
    }

    let mut minimo = &elementos[0];

    for elemento in elementos.iter() {
        if elemento < minimo {
            minimo = elemento;
        }
    }

    Some(minimo)
}

fn main() {
    let numeros = vec![42, 15, 8, 108, 23, 4, 16];

    match localiza_el_minimo(&numeros) {
        Some(minimo) => println!("El número más pequeño es {}", minimo),
        None => println!("La lista de números está vacía"),
    }

    let precios_que_aparecen_en_series = vec![8.99, 0.99, 1.50];

    match localiza_el_minimo(&precios_que_aparecen_en_series) {
        Some(minimo) => println!("El precio más pequeño es {}", minimo),
        None => println!("La lista de precios está vacía"),
    }

    let nombres = vec!["Juan", "Esther", "Marcos"];

    match localiza_el_minimo(&nombres) {
        Some(minimo) => println!("El nombre menor en orden alfabético es {}", minimo),
        None => println!("La lista de nombres está vacía"),
    }
}
