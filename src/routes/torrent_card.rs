use leptos::*;

use crate::{components::ui::Card, qbt::torrents::TorrentStatus};

#[derive(Debug, Clone, PartialEq)]
pub struct Torrent {
    pub hash: String,
    pub name: RwSignal<String>,
    pub progress: RwSignal<f64>,
    pub state: RwSignal<TorrentStatus>,
}

#[component]
pub fn TorrentCard(torrent: Torrent) -> impl IntoView {
    let state = move || torrent.state.with(|v| format!("{:?}", v));
    view! {
        <Card>
            <div class="font-bold">{move || torrent.name.get()}</div>
            <div>{state} : {move || format!("{:.2}", torrent.progress.get() * 100.0)} %</div>
        </Card>
    }
}
