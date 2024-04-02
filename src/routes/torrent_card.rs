use leptos::*;

use crate::components::ui::Card;

#[derive(Debug, Clone, PartialEq)]
pub struct Torrent {
    pub hash: String,
    pub name: RwSignal<String>,
    pub progress: RwSignal<f64>,
}

#[component]
pub fn TorrentCard(torrent: Torrent) -> impl IntoView {
    view! {
        <Card>
            <div class="font-bold">{move || torrent.name.get()}</div>
            <div>Progress: {move || format!("{:.2}", torrent.progress.get() * 100.0)} %</div>
        </Card>
    }
}
