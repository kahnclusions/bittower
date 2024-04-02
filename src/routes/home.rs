use std::collections::HashMap;

use crate::components::ui::Container;
use crate::qbt::get_sync_maindata;
use crate::qbt::transfer::ConnectionStatus;
use crate::routes::torrent_card::{Torrent, TorrentCard};

use crate::{components::ui::Stack, qbt::sync::MainData};
use human_bytes::human_bytes;
use leptos::{leptos_dom::logging::console_error, *};
use leptos_use::use_interval_fn;

#[server(SyncMainData, "/api")]
pub async fn sync_main_data(sid: String, rid: u64) -> Result<MainData, ServerFnError> {
    let res = get_sync_maindata(sid, rid).await;

    match res {
        Ok(results) => Ok(results),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ServerState {
    pub connection_status: RwSignal<ConnectionStatus>,
    pub dl_info_speed: RwSignal<f64>,
    pub up_info_speed: RwSignal<f64>,
}

#[component]
pub fn ServerStateBar(state: ServerState) -> impl IntoView {
    let dl_speed = move || human_bytes(state.dl_info_speed.get());
    let ul_speed = move || human_bytes(state.up_info_speed.get());
    let status = move || state.connection_status.with(|s| s.to_string());
    view! {
        <div class="flex flex-row gap-3 justify-between items-center">
            <div>{status}</div>
            <div>DL: {dl_speed}</div>
            <div>UP: {ul_speed}</div>
        </div>
    }
}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let sid = "MT/HpLY7t91t24RWXh9HOvOhhenYwunS".to_string();
    // let (sid, set_sid) = create_signal();
    let (rid, set_rid) = create_signal(0_u64);

    let (torrents, set_torrents) = create_signal::<HashMap<String, Torrent>>(HashMap::new());

    let server_state = ServerState {
        connection_status: create_rw_signal(ConnectionStatus::Disconnected),
        dl_info_speed: create_rw_signal(0.0),
        up_info_speed: create_rw_signal(0.0),
    };

    use_interval_fn(
        move || {
            let sid = sid.to_owned();
            spawn_local(async move {
                let res = sync_main_data(sid, rid.get_untracked()).await;
                match res {
                    Ok(MainData::Full(full_data)) => {
                        set_rid(full_data.rid);
                        let mut torrent_signals: HashMap<String, Torrent> = HashMap::new();
                        for (hash, torrent) in full_data.torrents {
                            torrent_signals.insert(
                                hash.clone(),
                                Torrent {
                                    hash: hash.clone(),
                                    name: create_rw_signal(torrent.name),
                                    progress: create_rw_signal(torrent.progress),
                                    state: create_rw_signal(torrent.state),
                                },
                            );
                        }
                        set_torrents(torrent_signals);

                        server_state
                            .connection_status
                            .set(full_data.server_state.connection_status);
                        server_state
                            .dl_info_speed
                            .set(full_data.server_state.dl_info_speed);
                        server_state
                            .up_info_speed
                            .set(full_data.server_state.up_info_speed);
                    }
                    Ok(MainData::Partial(partial_data)) => {
                        set_rid(partial_data.rid);
                        if let Some(partial_torrents) = partial_data.torrents {
                            for (hash, torrent) in partial_torrents {
                                if let Some(ts) = torrents.get_untracked().get_mut(&hash) {
                                    if let Some(value) = torrent.progress {
                                        ts.progress.set(value);
                                    }
                                    if let Some(value) = torrent.state {
                                        ts.state.set(value);
                                    }
                                }
                            }
                        }

                        if let Some(new_state) = partial_data.server_state {
                            if let Some(new_status) = new_state.connection_status {
                                server_state.connection_status.set(new_status);
                            }
                            if let Some(new_value) = new_state.dl_info_speed {
                                server_state.dl_info_speed.set(new_value);
                            }
                            if let Some(new_value) = new_state.up_info_speed {
                                server_state.up_info_speed.set(new_value);
                            }
                        }
                    }
                    Err(_) => {
                        console_error("Failed to get torrents");
                    }
                }
            })
        },
        1000,
    );

    view! {
        <h1 class="bg-slate-50 dark:bg-slate-900 font-display text-2xl text-center h-10 flex flex-row items-center justify-center">
            "bit-tower"
        </h1>
        <Container>
            <Stack>
                <For
                    each=move || {
                        let torrents: Vec<Torrent> = torrents()
                            .into_iter()
                            .map(|(_, torrent)| torrent)
                            .collect();
                        torrents
                    }

                    key=|t| t.hash.clone()
                    let:child
                >
                    <TorrentCard torrent=child/>
                </For>
            </Stack>
            <ServerStateBar state=server_state/>
        </Container>
    }
}
