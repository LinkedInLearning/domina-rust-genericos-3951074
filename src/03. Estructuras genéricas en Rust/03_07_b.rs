/*
 * Curso: Domina Rust: Genéricos
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

struct Paquete<T>{
    destino: String,
    datos: T,
    checksum: u32,
}

fn main(){
    
    let paquete = Paquete {
        destino: String::from("192.168.0.1"),
        datos: String::from("https://rust-lang.es"),
        checksum:162342,
    };
    
    // Interceptamos la variable
    
    let Paquete {destino, datos, checksum } = paquete;
    
    println!("Destino: {}", destino);
    println!("Datos: {}", datos);
    println!("Checksum: {}", checksum);
}