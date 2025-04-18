use crate::data::monitor_repo::MonitorRepo;
use crate::data::monitor_repo::MonitorRepo;
use crate::ui::monitor_list::MonitorList;
use dioxus::prelude::*;

#[component]
pub fn App(cx: Scope) -> Element {
    let repo = use_future(cx, (), |_| async move {
        MonitorRepo::new("active_monitors.csv").await
    });

    cx.render(rsx!(match repo.value() {
        None => rsx!( div { "Loading..." } ),
        Some(Err(e)) => rsx!( div { "Error: {e}" } ),
        Some(Ok(repo)) => rsx!(MonitorList { repo: repo.clone() }),
    }))
}
