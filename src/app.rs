// Shared
use crate::client::charts::auto_chart_resize;
use crate::error_template::{AppError, ErrorTemplate};
use charming::renderer::wasm_renderer::Echarts;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::rc::Rc;

// Browser Charming Echarts
#[cfg(feature = "hydrate")]
use charming::{
    component::{Axis, Title},
    element::AxisType,
    renderer::wasm_renderer::WasmRenderer,
    series::Line,
    Chart,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/charming-leptos.css"/>

        // ECharts inject
        <Script src="https://cdn.jsdelivr.net/npm/echarts@5.4.2/dist/echarts.min.js" />

        // sets the document title
        <Title text="Charming Example in Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

pub type EchartsWrite = WriteSignal<Option<Rc<Echarts>>>;
pub type EchartsRead = ReadSignal<Option<Rc<Echarts>>>;

fn update_title(view_update: EchartsRead) {
    #[cfg(feature = "hydrate")]
    WasmRenderer::update(
        &view_update.get().unwrap(),
        &Chart::new().title(Title::new().text("Hello Charming Leptos")),
    );
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (view_update, set_option): (EchartsRead, EchartsWrite) = create_signal(None);

    // AutoSize Chart onMount.
    create_effect(move |_| {
        if view_update.get().is_some() {
            auto_chart_resize(view_update);
        }
    });

    // Resize Chart on resize
    window_event_listener(ev::resize, move |_| auto_chart_resize(view_update));

    view! {
        <div class="container">
            <h1>"Welcome to Leptos!"</h1>
            <div class="clk-obj" on:click=move|_| update_title(view_update)>Click Me</div>
            <div class="chart">
                <Chart set_option=set_option id=1/>
            </div>
        </div>
        // <Chart set_option=set_option id=1/>
    }
}

#[component]
fn Chart(id: i32, set_option: EchartsWrite) -> impl IntoView {
    // Render on client Only, Feature flag is needed becouse our dependencies are also gated.
    #[cfg(feature = "hydrate")]
    spawn_local(async move {
        let chart = Chart::new()
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
            )
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Line::new().data(vec![150, 230, 224, 218, 135, 147, 260]));

        // Charming, Requires we set a width & Height
        let renderer = WasmRenderer::new(600, 600);
        let result = renderer.render(&id.to_string(), &chart);

        // Store echarts instance for updates.
        if let Ok(echarts) = result {
            set_option.update(move |v| *v = Some(Rc::new(echarts)));
        }
    });

    // Chart Container, it's rendered on both server and client.
    view! {<div id={id} class="chart"></div>}
}
