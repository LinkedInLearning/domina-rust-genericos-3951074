/*
 * Curso: Domina Rust: Genéricos
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * Más recursos sobre Rust: https://eliezerlopez.rs
 * */

struct MensajeSecreto<'a, T> {
    mensaje: &'a T,
}

impl<'a, T> MensajeSecreto<'a, T> {
    fn obtener_mensaje(&self) -> &'a T {
        self.mensaje
    }
    
    fn nuevo(mensaje: &'a T) -> Self {
        MensajeSecreto { mensaje }
    }
}

fn main() {
   
    let mensaje_secreto = "0100000101110000011100100110010101101110011001000110010100100000011011011100001110100001011100110010000001110011011011110110001001110010011001010010000001010010011101010111001101110100001000000110010101101110001000000110100001110100011101000111000001110011001110100010111100101111011100100111010101110011011101000010110101101100011000010110111001100111001011100110010101110011";
    let mensaje_secreto = MensajeSecreto::nuevo(&mensaje_secreto);
    
    println!("{}", mensaje_secreto.obtener_mensaje());
}