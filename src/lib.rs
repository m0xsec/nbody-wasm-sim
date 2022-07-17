mod dom;
mod gpu_primitives;
mod render;
mod runtime;
mod sim;

use gloo_console::log;
use render::WgpuContext;
use wasm_bindgen::prelude::*;
use winit::dpi::PhysicalSize;
use winit::platform::web::WindowBuilderExtWebSys;
use winit::{event_loop::EventLoop, window::WindowBuilder};

use crate::dom::Dom;
use crate::runtime::Runtime;

#[wasm_bindgen(start)]
pub async fn run() {
    // Redirect panics to the console (debugging)
    console_error_panic_hook::set_once();

    let dom = Dom::new();
    let canvas = dom::get_canvas();
    log!("Acquired DOM elements");

    // Create window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_canvas(Some(canvas))
        .build(&event_loop)
        .and_then(|w| {
            // Set attributes
            w.set_inner_size(PhysicalSize::new(600.0, 400.0));
            Ok(w)
        })
        .expect("Could not build window");
    log!("Created window");

    // Connect graphics card to window
    let mut context = WgpuContext::new(&window).await;
    log!("Acquired graphics context");

    // Load shaders
    context.add_shader("vert", "../assets/shaders/vert.wgsl");
    context.add_shader("frag", "../assets/shaders/frag.wgsl");
    log!("Loaded shaders");

    context.add_texture("cookie", "../assets/textures/cookie.png");

    // Run program
    let mut runtime = Runtime::new(context, window, dom);
    log!("Starting...");
    event_loop.run(move |event, target, control_flow| {
        runtime.main_loop(event, target, control_flow)
    });
}
