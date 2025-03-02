/*
 * Curso: Domina Rust: Genéricos
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

fn imprimir_valor<T: std::fmt::Debug>(valor: T){
    println!("{:?}", valor);
}

fn main(){
    imprimir_valor(30);
    imprimir_valor("@EliezerLopez");
}