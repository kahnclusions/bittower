use std::collections::HashMap;

use crate::{
    error_template::{AppError, ErrorTemplate},
    qbt::{get_sync_maindata, get_torrents_info, sync::MainData, torrents::TorrentSummary},
};
use leptos::{
    leptos_dom::logging::{console_error, console_log},
    *,
};
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{use_interval_fn, utils::Pausable};

use crate::routes::home::HomePage;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/bittower.css"/>
        <Title text="BitTower"/>
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main class="h-full w-full text-slate-950 dark:text-slate-50 dark:bg-slate-950 bg-slate-100 text-slate-950 dark:text-slate-50">
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}
