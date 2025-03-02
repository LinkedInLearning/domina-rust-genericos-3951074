/*
 * Curso: Domina Rust: Genéricos
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

fn valor_mas_alto<T: std::cmp::PartialOrd>(cadena1: &T, cadena2: &T) -> &T {
    if cadena1 > cadena2 {
        cadena1
    } else {
        cadena2
    }
}

fn main(){
    let cadena1 =  "LinkedIn";
    let cadena2 = "Learning";
    
    let es_mayor = valor_mas_alto(&cadena1, &cadena2);
    
    println!("La cadena más alta es: {}", es_mayor);
}