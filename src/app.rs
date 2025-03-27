use leptos::{ev, prelude::*, task::spawn_local};

use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use charming::Echarts;
use charming::WasmRenderer;
use std::rc::Rc;

#[cfg(feature = "hydrate")]
use crate::ChartPrelude::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/charming-leptos.css"/>
            // ECharts inject
        <script src="https://cdn.jsdelivr.net/npm/echarts@5.4.2/dist/echarts.min.js" />
        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

// Echarts needs to be wrapped in an Rc to clone. It's only used client-side in a wasm context
// which is single threaded so Rc is sufficient.
pub type EchartsWrite = Option<Rc<Echarts>>;
pub type EchartsRead = Option<Rc<Echarts>>;

fn update_title(view_update: ArcReadSignal<EchartsRead>) {
    #[cfg(feature = "hydrate")]
    spawn_local(async move {
        use charming::component::Title;
        use charming::Chart;
        WasmRenderer::update(
            &view_update.get().unwrap(),
            &Chart::new().title(Title::new().text("Hello Charming Leptos")),
        );
    });
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (view_update, set_option): (ArcReadSignal<EchartsRead>, ArcWriteSignal<EchartsWrite>) =
        arc_signal(None);

    // Client - Resize Chart on resize
    #[cfg(feature = "hydrate")]
    {
        window_event_listener(ev::resize, move |_| {
            auto_chart_resize(&view_update.get_untracked())
        });
    }

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <div style="width: 100%; height: 800px;">
            <Chart set_option=set_option id=1/>
        </div>
    }
}

#[component]
fn Chart(id: i32, set_option: ArcWriteSignal<EchartsWrite>) -> impl IntoView {
    use leptos::task::spawn_local;
    // Render on client Only, Feature flag is needed becouse our dependencies are also gated.
    #[cfg(feature = "hydrate")]
    spawn_local(async move {
        // echarts instance
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
            let echart_rc = Rc::new(echarts);
            auto_chart_resize(&Some(echart_rc.clone()));
            set_option.update(move |v| *v = Some(echart_rc.clone()));
        }
    });

    // Chart Container, it's rendered on both server and client.
    view! {<div id={id} class="chart"></div>}
}
