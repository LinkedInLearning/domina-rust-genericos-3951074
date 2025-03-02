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
    let p = Punto::nuevo(4, 8);
    println!("Punto: ({},{})", p.x, p.y);
    
    let q = Punto::nuevo(15.16, 23.42);
    println!("Punto: ({},{})", q.x, q.y);
}