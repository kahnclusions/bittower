use std::collections::HashMap;

use crate::{error_template::{AppError, ErrorTemplate}, qbt::{get_sync_maindata, get_torrents_info, torrents::{MainData, TorrentSummary}}};
use leptos::{leptos_dom::logging::{console_error, console_log}, *};
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{use_interval_fn, utils::Pausable};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/bittower.css" />
        <Title text="BitTower"/>
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main class="h-full w-full text-slate-950 dark:text-slate-50 dark:bg-slate-950 bg-slate-100 text-slate-950 dark:text-slate-50">
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[server(SyncMainData, "/api")]
pub async fn sync_main_data(sid: String, rid: u64) -> Result<MainData, ServerFnError> {
    let res = get_sync_maindata(sid, rid).await;

    match res {
        Ok(results) => Ok(results),
        Err(e) => Err(ServerFnError::ServerError(e.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Torrent {
    pub hash: String,
    pub name: RwSignal<String>,
    pub progress: RwSignal<f64>
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let sid = "MT/HpLY7t91t24RWXh9HOvOhhenYwunS".to_string();
    // let (sid, set_sid) = create_signal();
    let (rid, set_rid) = create_signal(0_u64);
    
    let (torrents, set_torrents) = create_signal::<HashMap<String, Torrent>>(HashMap::new());

    let Pausable { pause, resume, is_active } = use_interval_fn(move || {
        let sid = sid.to_owned();
        spawn_local(async move {
            let res = sync_main_data(sid, rid.get_untracked()).await;
           match res {
               Ok(MainData::Full(full_data)) => {
                   set_rid(full_data.rid);
                   let mut torrent_signals: HashMap<String, Torrent> = HashMap::new();
                   for (hash, torrent) in full_data.torrents {
                      torrent_signals.insert(hash.clone(), Torrent {
                          hash: hash.clone(),
                          name: create_rw_signal(torrent.name),
                          progress: create_rw_signal(torrent.progress)
                      });
                   }
                   set_torrents(torrent_signals);
               },
               Ok(MainData::Partial(partial_data)) => {
                   set_rid(partial_data.rid);
                   if let Some(partial_torrents) = partial_data.torrents {
                       for (hash, torrent) in partial_torrents {
                           if let Some(ts) = torrents.get_untracked().get_mut(&hash) {
                               if let Some(progress) = torrent.progress {
                                   ts.progress.set(progress);
                               }
                           }
                       }
                   }
               },
               Err(_) => {console_error("Failed to get torrents");}
           }
        })
    }, 1000);
    

    view! {
        <h1 class="bg-slate-50 dark:bg-slate-900 font-display text-2xl text-center h-10 flex flex-row items-center justify-center">"Hello, world!"</h1>
            <div>
                  <div>
                    <For 
                        each=move || {
                            let torrents: Vec<Torrent> = torrents.get().into_iter().map(|(_, torrent)| torrent).collect();
                            torrents
                        }
                        key=|t| t.hash.clone()
                        let:child
                > 
                                <TorrentCard torrent={child} />
                    </For>
                  </div>
        </div>
    }
}

#[component]
fn TorrentCard(torrent: Torrent) -> impl IntoView {
    view! {
        <div class="p-3">
            <span>{move || torrent.name.get()}: {move || format!("{:.2}", torrent.progress.get() * 100.0)}%</span>
        </div>
    }
}

