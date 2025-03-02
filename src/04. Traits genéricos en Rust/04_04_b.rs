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

#[derive(Clone, Debug)]
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
   
   let mut estacion_el_cisne = Contador { valor: 42 };
   println!("Valor inicial del contador: {}", estacion_el_cisne.valor);
   
   estacion_el_cisne.restablecer(108);
   println!("Valor después de restablecer: {}", estacion_el_cisne.valor);
   
   let mut configuracion = Configuracion {
       volumen: 50,
       calidad: "Alta".to_string(),
       modo: "Oscuro".to_string(),
   };
   
   let configuracion_por_defecto = Configuracion {
       volumen: 100,
       calidad: "Media".to_string(),
       modo: "Claro".to_string(),
   };
   
   println!("Configuración inicial: {:?}", configuracion);
   
   configuracion.restablecer(configuracion_por_defecto.clone());
   println!("Configuración después de restablecer: {:?}", configuracion);
}