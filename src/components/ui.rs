use leptos::*;

#[component]
pub fn Container(children: Children, class: Option<String>) -> impl IntoView {
    let class = format!(
        "px-3 w-full lg:max-w-[700px] mx-auto {}",
        class.unwrap_or("".to_string())
    );
    view! { <div class=class>{children()}</div> }
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
pub fn Stack(class: Option<String>, children: Children) -> impl IntoView {
    let class = format!(
        "{} {}",
        "flex flex-col gap-3",
        class.unwrap_or("".to_string())
    );
    view! { <div class=class>{children()}</div> }
}
