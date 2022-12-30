use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Document;

mod graphics;

static mut GRAPHICS: Option<graphics::Graphics> = None;
static mut EXPLOTION_SOUNDS: Vec<web_sys::HtmlAudioElement> = vec![];
static mut LAUNCH_SOUNDS: Vec<web_sys::HtmlAudioElement> = vec![];

fn get_audio_element_by_id(document: &mut Document, id: &str) -> web_sys::HtmlAudioElement {
    document
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<web_sys::HtmlAudioElement>()
        .map_err(|_| ())
        .unwrap()
}

/* Initialize the simulation. */
#[wasm_bindgen(start)]
pub unsafe fn start() {
    let window = web_sys::window().unwrap();
    let mut document = window.document().unwrap();

    let canvas = document.get_element_by_id("fireworks").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    EXPLOTION_SOUNDS.push(get_audio_element_by_id(&mut document, "explosion"));
    EXPLOTION_SOUNDS.push(get_audio_element_by_id(&mut document, "explosion2"));
    EXPLOTION_SOUNDS.push(get_audio_element_by_id(&mut document, "explosion3"));

    LAUNCH_SOUNDS.push(get_audio_element_by_id(&mut document, "launch"));
    LAUNCH_SOUNDS.push(get_audio_element_by_id(&mut document, "launch2"));
    LAUNCH_SOUNDS.push(get_audio_element_by_id(&mut document, "launch3"));

    /* Resize the canvas to take up the entire screen. */
    canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
    canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);

    /* Create a new simulation and initialize it. */
    let mut graphics = graphics::Graphics::new(canvas);
    graphics.init();

    unsafe {
        GRAPHICS = Some(graphics);
    }
}

/* Draw the current state of the simulation, and simulate one step. */
#[wasm_bindgen]
pub unsafe fn draw() {
    /* This is only unsafe when aquiring multiple muteable references,
     * but the reference is immediately dropped, so this is safe. */
    if let Some(graphics) = unsafe { GRAPHICS.as_mut() } {
        graphics.step();

        graphics.draw();
    }
}

/* Spawn a new firework. */
#[wasm_bindgen]
pub unsafe fn spawn_firework(name: String) {
    /* This is only unsafe when aquiring multiple muteable references,
     * but the reference is immediately dropped, so this is safe. */
    if let Some(graphics) = unsafe { GRAPHICS.as_mut() } {
        graphics.spawn_firework(name);
    }
}

#[wasm_bindgen]
pub fn resize_canvas() {
    /* This is only unsafe when aquiring multiple muteable references,
     * but the reference is immediately dropped, so this is safe. */
    if let Some(graphics) = unsafe { GRAPHICS.as_mut() } {
        let window = web_sys::window().unwrap();

        graphics.resize(
            window.inner_width().unwrap().as_f64().unwrap() as u32,
            window.inner_height().unwrap().as_f64().unwrap() as u32,
        );
    }
}
