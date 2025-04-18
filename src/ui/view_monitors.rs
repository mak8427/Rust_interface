// src/ui/view_monitors.rs
use dioxus::prelude::*;
use polars::prelude::DataFrame;

#[component]
pub fn ViewMonitors<'a>(cx: Scope<'a>, df: &'a DataFrame) -> Element {
    cx.render(rsx!(
        ul {
            // wrap the iterator in braces so RSX knows it’s code
            { (0..df.height().min(5)).map(|i| {
                // pull out the "Label" column as &str
                let label = df
                .column("Label")       // Series
                .and_then(|s| s.utf8())// note the () on utf8()
            .and_then(|ca| ca.get(i))
            .unwrap_or_default();  // "" if missing

              rsx!( li { "{label}" } )
            }) }
        }
    ))
}
