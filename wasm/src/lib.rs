use rchip8::cpu::Cpu;
use std::sync::Mutex;

use wasm_bindgen::prelude::*;


#[macro_use]
extern crate lazy_static;


lazy_static! {
	/**
	 * Our runtime will instantiate a global CPU instance
	 */
	static ref CPU: Mutex<Cpu> = Mutex::new(Cpu::new());
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

/* Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    Ok(())
}*/


#[wasm_bindgen]
pub fn load_program(prog: &[u8]) -> Result<(), JsValue>  {
	let mut cpu = CPU.lock().unwrap();
	match cpu.load_from_bytes(&prog) {
		Ok(_) => {},
		Err(e) => {
			console_log!("{:?}", e);
			return Err(format!("{:?}",e).into());
		}
	}
    console_log!("[+] loaded ROM");
	Ok(())
}


#[wasm_bindgen]
pub fn execute_cycle() -> Result<(), JsValue> {
    let mut cpu = CPU.lock().unwrap();
    let opcode = cpu.fetch_instruction();
    cpu.execute_instruction(opcode);
    Ok(())
}


#[wasm_bindgen]
pub fn handle_key_event(code: u32, event_type: &str) {
    match code {
        _ => console_log!("got key {:?}, type: {:?}", code, event_type),
    }
}