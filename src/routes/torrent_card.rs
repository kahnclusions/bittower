use leptos::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Torrent {
    pub hash: String,
    pub name: RwSignal<String>,
    pub progress: RwSignal<f64>,
}

#[component]
pub fn TorrentCard(torrent: Torrent) -> impl IntoView {
    view! {
        <div class="p-3 border border-4 border-slate-500">
            <span>
                {move || torrent.name.get()} :
                {move || format!("{:.2}", torrent.progress.get() * 100.0)} %
            </span>
        </div>
    }
}

#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! { <div class="p-3 border border-4 border-slate-500">{children()}</div> }
}
