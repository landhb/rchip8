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
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

/**
 * Wasm init
 */
#[wasm_bindgen(start)]
pub fn entry() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}

/**
 * Load a binary blob as the Chip8 program
 *
 * This is required because the rust wasm runtime can't
 * yet read files by itself, so we will fetch the file with
 * JS first
 */
#[wasm_bindgen]
pub fn load_program(prog: &[u8]) -> Result<(), JsValue> {
    let mut cpu = CPU.lock().unwrap();
    match cpu.load_from_bytes(&prog) {
        Ok(_) => {}
        Err(e) => {
            console_log!("{:?}", e);
            return Err(format!("{:?}", e).into());
        }
    }
    console_log!("[+] loaded ROM");
    Ok(())
}

/**
 * Complete a full fetch -> execute cycle for the next
 * instruction
 */
#[wasm_bindgen]
pub fn execute_cycle() -> Result<(), JsValue> {
    let mut cpu = CPU.lock().unwrap();
    let opcode = cpu.fetch_instruction();
    console_log!("{:x}", opcode);
    match cpu.execute_instruction(opcode) {
        Ok(_) => {}
        Err(e) => {
            console_log!("{:?}", e);
            return Err(format!("{:?}", e).into());
        }
    }
    Ok(())
}

/**
 * Handle all key events by updating the emulator
 * state appropriately
 */
#[wasm_bindgen]
pub fn handle_key_event(code: u32, event_type: &str) {
    let mut cpu = CPU.lock().unwrap();
    match event_type {
        "keydown" => cpu.key_down(code as usize),
        "keyup" => cpu.key_up(code as usize),
        _ => {}
    }
    console_log!("got key {:?}, type: {:?}", code, event_type)
}

/**
 * Update the display by writing into the provided JS buffer
 *
 * JS reference for the buffer:
 * https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createImageData
 */
#[wasm_bindgen]
pub fn update_display(display: &mut [u8]) {
    let mut cpu = CPU.lock().unwrap();
    let (data, glow) = cpu.get_display();
    for i in 0..data.len() {
        if glow[i] > 0 {
            display[i * 4 + 0] = 0x33;
            display[i * 4 + 1] = 0xff;
            display[i * 4 + 2] = 0x99 + glow[i];
            glow[i] -= 1;
            continue;
        }

        display[i * 4 + 0] = if data[i] == 1 { 0x33 } else { 0x0 };
        display[i * 4 + 1] = if data[i] == 1 { 0xff } else { 0x0 };
        display[i * 4 + 2] = if data[i] == 1 { 0x66 } else { 0x0 };
        display[i * 4 + 3] = 255;
    }
}

/**
 *  Update the timers, should get called at 60Hz
 */
#[wasm_bindgen]
pub fn update_timers() {
    let mut cpu = CPU.lock().unwrap();
    cpu.decrement_timers();
}
