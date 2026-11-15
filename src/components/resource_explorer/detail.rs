use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::{
    components::A,
    hooks::{use_params_map, use_query_map},
};

use super::{
    editor::ResourceEditor,
    query::{filter_suffix, BASE},
};
use crate::api::{get_todo, search_todos, TodoItem, TodosResponse};

type RelatedResource = Resource<Result<TodosResponse, ServerFnError>>;

#[component]
pub fn ResourceDetailPage() -> impl IntoView {
    let params = use_params_map();
    let query = use_query_map();
    let record = Resource::new(
        move || {
            params
                .get()
                .get("id")
                .and_then(|value| value.parse::<i64>().ok())
        },
        |id| async move {
            match id.filter(|id| *id > 0) {
                Some(id) => get_todo(id).await,
                None => Err(ServerFnError::ServerError("Invalid record link.".into())),
            }
        },
    );

    // Resource construction belongs to the route, before any reactive view.
    // Async SSR may evaluate a resolved branch more than once. Constructing a
    // resource in that branch leaves extra serialized values that can seed an
    // unrelated resource on later client navigation.
    let saved_status = RwSignal::new(None::<(i64, bool)>);
    Effect::new(move |_| {
        // A saved override belongs to this visit. Revisiting a record must use
        // its fresh server status, including changes made in another tab.
        params.get();
        saved_status.set(None);
    });
    let related = Resource::new(
        move || (params.get().get("id"), saved_status.get()),
        move |(_, saved_status)| async move {
            // Await the parent value rather than starting an unresolved future
            // from its initial None state. This also preserves SSR completion.
            let item = record.await?;
            let completed = saved_status
                .filter(|(saved_id, _)| *saved_id == item.id)
                .map(|(_, completed)| completed)
                .unwrap_or(item.completed);
            search_todos(
                String::new(),
                if completed { "completed" } else { "open" }.into(),
            )
            .await
        },
    );
    let on_saved =
        Callback::new(move |item: TodoItem| saved_status.set(Some((item.id, item.completed))));

    view! {
        <Title text="Record details — Resource explorer"/>
        <div class="page-shell explorer section-stack section-stack--related">
            <nav aria-label="Record navigation"><A href=move || format!("{BASE}{}", filter_suffix(&query.get()))>"← Back to results"</A></nav>
            <Suspense fallback=|| view! { <p class="feedback" role="status">"Loading record…"</p> }>
                {move || record.get().map(|result| match result {
                    Ok(item) => view! { <RecordWorkspace item=item related=related on_saved=on_saved/> }.into_any(),
                    Err(_) => view! {
                        <section class="feedback feedback--error section-stack section-stack--related" role="alert">
                            <h1 class="detail-title">"Record unavailable"</h1>
                            <p>"This link may be invalid, the record may have been removed, or it belongs to another browser session. A connection problem can also prevent loading."</p>
                            <div><button class="control-frame control-frame--secondary" on:click=move |_| { record.refetch(); related.refetch(); }>"Retry loading"</button></div>
                        </section>
                    }.into_any(),
                })}
            </Suspense>
            <noscript><p class="feedback">"Record details and related links work without JavaScript. Enable JavaScript to review and save edits."</p></noscript>
        </div>
    }
}

#[component]
fn RecordWorkspace(
    item: TodoItem,
    related: RelatedResource,
    on_saved: Callback<TodoItem>,
) -> impl IntoView {
    let saved = RwSignal::new(item);
    view! {
        <div class="explorer-workspace">
            <ResourceEditor saved=saved on_saved=on_saved/>
            <RelatedRecords saved=saved related=related/>
        </div>
    }
}

#[component]
fn RelatedRecords(saved: RwSignal<TodoItem>, related: RelatedResource) -> impl IntoView {
    let query = use_query_map();
    view! {
        <aside class="explorer-related section-stack section-stack--related" aria-labelledby="related-title">
            <h2 id="related-title">"Related records"</h2>
            <p class="composer-hint">"Other records in this session with the same saved status. Showing up to five."</p>
            <Suspense fallback=|| view! { <p role="status">"Loading related records…"</p> }>
                {move || related.get().map(|result| match result {
                    Ok(response) => {
                        let records = response.items.into_iter().filter(|item| item.id != saved.with(|item| item.id)).take(5).collect::<Vec<_>>();
                        if records.is_empty() {
                            view! { <p>"No other records share this status yet."</p> }.into_any()
                        } else {
                            view! {
                                <ul class="related-list">
                                    {records.into_iter().map(|item| view! { <li><A href=format!("{BASE}/{}{}", item.id, filter_suffix(&query.get()))>{item.title}</A></li> }).collect_view()}
                                </ul>
                            }.into_any()
                        }
                    }
                    Err(_) => view! {
                        <div class="section-stack section-stack--related">
                            <p>"Related records could not be loaded. You can still work on this record."</p>
                            <button class="control-frame control-frame--secondary" on:click=move |_| related.refetch()>"Retry related records"</button>
                        </div>
                    }.into_any(),
                })}
            </Suspense>
        </aside>
    }
}
