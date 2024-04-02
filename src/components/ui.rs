use leptos::*;

#[component]
pub fn Container(children: Children) -> impl IntoView {
    view! { <div class="p-3 w-full lg:max-w-[700px] mx-auto">{children()}</div> }
}

#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="p-3 border border-4 border-slate-500 bg-slate-50 dark:bg-slate-950">
            {children()}
        </div>
    }
}

#[component]
pub fn Stack(children: Children) -> impl IntoView {
    view! { <div class="flex flex-col gap-3">{children()}</div> }
}
