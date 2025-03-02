/*
 * Curso: Domina Rust: Genéricos
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

fn valor_mas_alto<T: PartialOrd + Clone>(a: T, b: T) -> T { // PartialOrd
    if a > b {
        a.clone()
    } else {
        b.clone()
    }
}

fn main(){
    let a = 4;
    let b = 8;
    let mayor = valor_mas_alto(a, b);
    println!("El número más alto es: {}", mayor);
}