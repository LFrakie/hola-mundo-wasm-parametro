use wasm_bindgen::prelude::*;

#[no_mangle]
#[wasm_bindgen(js_name = saludar)]
pub fn saludar(nombre: &str) -> String {
    format!("¡Hola, {}! ¡Bienvenido a Rust y a WebAssembly!", nombre)
}

