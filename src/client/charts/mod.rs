use crate::app::EchartsRead;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_wasm_bindgen::to_value;

#[derive(Serialize, Deserialize)]
struct ResizeOpts {
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    animation: Option<AnimationOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    silent: Option<bool>,
}

impl ResizeOpts {
    fn new(width: Option<Value>, height: Option<Value>) -> Self {
        ResizeOpts {
            width,
            height,
            animation: None,
            silent: None,
        }
    }

    /// From method to create ResizeOpts from i32 values
    pub fn from(width: i32, height: i32) -> Self {
        ResizeOpts {
            width: Some(width.into()),
            height: Some(height.into()),
            animation: None,
            silent: None,
        }
    }

    pub fn width(mut self, width: i32) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height(mut self, height: i32) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn animation(mut self, animation: AnimationOption) -> Self {
        self.animation = Some(animation.into());
        self
    }

    pub fn silent(mut self, silent: bool) -> Self {
        self.silent = Some(silent).into();
        self
    }
}

#[derive(Serialize, Deserialize)]
struct AnimationOption {
    duration: Option<i32>,
    easing: Option<AnimationEasing>,
    delay: Option<i32>,
    // Echarts: has this commented out.
    // additive?: boolean
}

pub fn auto_chart_resize(view_update: EchartsRead) {
    if let Some(echart) = view_update.get_untracked() {
        if let Some(_client_body) = document().body() {
            // Client Height & Width by Leptos
            //let width = client_body.client_width();
            //let height = client_body.client_height();

            // Resize Chart
            // Echarts resize method takes the following options
            //  width: number | 'auto',
            //  height: number | 'auto', // Can be 'auto' (the same as null/undefined)
            //  animation: AnimationOption
            //  silent: boolean // by default false.

            let _ = echart.resize(
                to_value(&ResizeOpts::new(
                    // ECharts defaults to 100% width and auto height for elements. If the height is not set,
                    // the chart may not render correctly because a div with 100% width and 0 height will cause it to disappear.
                    // Make sure to specify a height for your chart element or set it in ECharts configuration.
                    Some("auto".into()),
                    Some(300.into()),
                ))
                .unwrap(),
            );
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum AnimationEasing {
    Linear,
    QuadraticIn,
    QuadraticOut,
    QuadraticInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    QuarticIn,
    QuarticOut,
    QuarticInOut,
    QuinticIn,
    QuinticOut,
    QuinticInOut,
    SinusoidalIn,
    SinusoidalOut,
    SinusoidalInOut,
    ExponentialIn,
    ExponentialOut,
    ExponentialInOut,
    CircularIn,
    CircularOut,
    CircularInOut,
    ElasticIn,
    ElasticOut,
    ElasticInOut,
    BackIn,
    BackOut,
    BackInOut,
    BounceIn,
    BounceOut,
    BounceInOut,
}