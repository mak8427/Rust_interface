use dioxus::prelude::*;
use polars::prelude::*;
use polars_io::prelude::*;
use std::fs::File;
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

fn read_csv_now(file_name: &str) -> PolarsResult<DataFrame> {
    CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(file_name.into()))?
        .finish()
}

#[component]
fn App() -> Element {
    rsx! { "Culoh"};

    let show_title: bool = true;

    rsx! {
        {show_title.then(|| rsx!{"Theme"})}

        ul {
            {(0..5).map(|i| rsx! { "{i}" })}
        }


        ul {

            {{0..5}.map(|i| rsx!{"{i}"})}
        }

    }
}
