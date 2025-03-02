/*
 * Curso: Domina Rust: Genéricos
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

use std::fmt;

enum TipoProducto {
    Fisico,
    Digital,
}

impl fmt::Display for TipoProducto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TipoProducto::Fisico => write!(f, "Físico"),
            TipoProducto::Digital => write!(f, "Digital"),
        }
    }
}

struct DetallesFisicos {
    color: String,
    peso: f32,
}

impl fmt::Display for DetallesFisicos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color: {}, Peso: {:.2} kg", self.color, self.peso)
    }
}

struct DetallesDigitales {
    formato: String,
    autor: String,
}

impl fmt::Display for DetallesDigitales {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Formato: {}, Autor: {}", self.formato, self.autor)
    }
}

struct Producto<T> {
    nombre: String,
    precio: f32,
    tipo: TipoProducto,
    detalles: T,
}

impl<T: fmt::Display> fmt::Display for Producto<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Producto: {}\nTipo: {}\nPrecio: ${:.2}\nDetalles: {}",
            self.nombre, self.tipo, self.precio, self.detalles
        )
    }
}

impl<T: fmt::Display> Producto<T> {
    fn nuevo(nombre: &str, precio: f32, tipo: TipoProducto, detalles: T) -> Producto<T> {
        Producto {
            nombre: nombre.to_string(),
            precio,
            tipo,
            detalles,
        }
    }

    fn mostrar(&self) {
        println!("Nombre: {}", self.nombre);
        println!("Precio: {}€", self.precio);
        match &self.tipo {
            TipoProducto::Fisico => println!("Tipo: Producto físico"),
            TipoProducto::Digital => println!("Tipo: Producto digital"),
        }
        println!("Detalles: {}", self.detalles);
    }
}

fn main() {

    
}
