/*
 * Curso: Domina Rust: Genéricos
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

trait Restablecer<T: Clone> {
    fn restablecer(&mut self, valor_por_defecto: T);
}

struct Contador {
    valor: i32,
}

impl Restablecer<i32> for Contador {
    fn restablecer(&mut self, valor_por_defecto: i32) {
        self.valor = valor_por_defecto;
    }
}

#[derive(Clone)]
struct Configuracion {
    volumen: u8,
    calidad: String,
    modo: String,
}

impl Restablecer<Configuracion> for Configuracion {
    fn restablecer(&mut self, valor_por_defecto: Configuracion) {
        self.volumen = valor_por_defecto.volumen;
        self.calidad = valor_por_defecto.calidad;
        self.modo = valor_por_defecto.modo;
    }
}

fn main() {
   
   // Dar cera, pulir cera. Dar cera, pulir cera...
   
}