/*
 * Curso: Domina Rust: Genéricos
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */
 
fn sumar(a: T, b: U){
    // ... 
}

fn sumar_valores(valores: Vec<T>)-> T {
    let mut suma = 0;
    for valor in valores {
        suma += valor;
    }
    
    suma
}

fn main() {
    
    let valores = vec![4, 8, 15, 16, 23, 42];
    let resultado = sumar_valores(valores);
    println!("La suma es: {}", resultado);
}