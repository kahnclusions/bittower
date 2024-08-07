use fnord_ui::components::{Text, View};
use human_bytes::human_bytes;
use leptos::prelude::*;

use crate::signals::syncstate::ServerState;
use qbittorrent_rs_proto::transfer::ServerStateFull;

#[component]
pub fn StatusBar(server_state: ServerState) -> impl IntoView {
    let dl_speed = move || human_bytes(server_state.dl_info_speed.get());
    let up_speed = move || human_bytes(server_state.up_info_speed.get());
    let dl_data = move || human_bytes(server_state.dl_info_data.get());
    let up_data = move || human_bytes(server_state.up_info_data.get());

    view! {
        <View class="flex-row bg-background-highlight justify-between fixed bottom-0 left-0 right-0 h-10 text-sm">
            <View class="gap-0">
                <div>{move || dl_speed()}"/s"</div>
                <div>{move || up_speed()}"/s"</div>
            </View>
            <View class="gap-0">
                <div>"Downloaded: "{move || dl_data()}</div>
                <div>"Uploaded: "{move || up_data()}</div>
            </View>
        </View>
    }
}
