/*
 * Curso: Domina Rust: Genéricos
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

struct Punto<T>{
    x: T,
    y: T,
}

impl<T> Punto<T> {
    fn nuevo(x: T, y: T) -> Self {
        Punto {x, y}
    }
}

fn main(){

}