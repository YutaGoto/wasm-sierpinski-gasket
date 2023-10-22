use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn draw_triangle(
    context: &web_sys::CanvasRenderingContext2d,
    points: [(f64, f64); 3],
    color: &str,
) {
    let [top, left, right] = points;
    context.move_to(top.0, top.1);
    context.begin_path();
    context.line_to(left.0, left.1);
    context.line_to(right.0, right.1);
    context.line_to(top.0, top.1);
    context.close_path();
    context.stroke();
    context.set_fill_style(&JsValue::from_str(color));
    context.fill();
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("drawing a triangle"));

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    draw_triangle(
        &context,
        [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)],
        "#ffffff",
    );
    draw_triangle(
        &context,
        [(300.0, 0.0), (150.0, 300.0), (450.0, 300.0)],
        "#e5be6c",
    );
    draw_triangle(
        &context,
        [(150.0, 300.0), (0.0, 600.0), (300.0, 600.0)],
        "#e5be6c",
    );
    draw_triangle(
        &context,
        [(450.0, 300.0), (300.0, 600.0), (600.0, 600.0)],
        "#e5be6c",
    );

    // context.move_to(300.0, 0.0);
    // context.begin_path();
    // context.line_to(0.0, 600.0);
    // context.line_to(600.0, 600.0);
    // context.line_to(300.0, 0.0);
    // context.close_path();
    // context.stroke();
    // context.set_fill_style(&JsValue::from_str("#e5be6c"));
    // context.fill();

    Ok(())
}
