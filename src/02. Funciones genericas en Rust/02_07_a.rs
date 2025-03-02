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
    
}
