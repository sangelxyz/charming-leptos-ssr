pub mod app;

#[cfg(feature = "hydrate")]
pub mod client;

#[cfg(feature = "hydrate")]
pub mod ChartPrelude {
    pub use super::client::charts::auto_chart_resize;
    pub use charming::component::Axis;
    pub use charming::component::Title;
    pub use charming::element::AxisType;
    pub use charming::series::Line;
    pub use charming::Chart;
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
