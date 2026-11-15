mod detail;
mod editor;
mod query;

pub use detail::ResourceDetailPage;

use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::{
    components::{Form, A},
    hooks::use_query_map,
};

use super::ui::{ActionLink, ControlTone};
use crate::api::{search_todos, CreateTodo, TodoItem};
use query::{filter_suffix, status_filter, BASE};

/// A runnable recipe: URL-owned filters, server-owned records, local drafts.
#[component]
pub fn ResourceExplorerPage() -> impl IntoView {
    let query = use_query_map();
    let create = ServerAction::<CreateTodo>::new();
    let refresh = RwSignal::new(0usize);
    let draft = RwSignal::new(String::new());
    let hydrated = RwSignal::new(false);
    Effect::new(move |_| hydrated.set(true));
    Effect::new(move |_| {
        if matches!(create.value().get(), Some(Ok(_))) {
            draft.set(String::new());
            refresh.update(|value| *value += 1);
        }
    });
    let records = Resource::new(
        move || {
            (
                query.get().get("q").unwrap_or_default(),
                status_filter(&query.get()),
                refresh.get(),
            )
        },
        |(q, status, _)| search_todos(q, status.to_string()),
    );

    view! {
        <Title text="Resource explorer — Leptos CF"/>
        <Meta name="description" content="A working collection, detail, and review workflow using session-scoped records."/>
        <div class="page-shell explorer section-stack section-stack--related">
            <header class="explorer-heading section-stack section-stack--related">
                <A href="/patterns">"← All patterns"</A>
                <p class="eyebrow">"Workflow 01 · resource explorer"</p>
                <h1>"Resource explorer"</h1>
                <p class="page-lede">"Find a record, inspect its context, and review a change before saving. These are your browser session’s lab records."</p>
            </header>

            <Form method="get" action=BASE attr:class="explorer-filters" attr:role="search">
                <label class="explorer-field" for="resource-search">
                    <span class="composer-label">"Search records"</span>
                    <input id="resource-search" class="composer-input" type="search" name="q" maxlength=crate::todos::MAX_SEARCH_CHARS
                        value=move || query.get().get("q").unwrap_or_default()
                        prop:value=move || query.get().get("q").unwrap_or_default()/>
                </label>
                <label class="explorer-field" for="resource-status">
                    <span class="composer-label">"Status"</span>
                    <select id="resource-status" class="composer-input" name="status" prop:value=move || status_filter(&query.get())>
                        <option value="all" selected=move || status_filter(&query.get()) == "all">"All statuses"</option>
                        <option value="open" selected=move || status_filter(&query.get()) == "open">"Open"</option>
                        <option value="completed" selected=move || status_filter(&query.get()) == "completed">"Completed"</option>
                    </select>
                </label>
                <button type="submit" class="control-frame control-frame--primary">"Apply filters"</button>
                <ActionLink href=BASE tone=ControlTone::Secondary>"Clear"</ActionLink>
            </Form>

            <Suspense fallback=|| view! { <p class="feedback" role="status">"Loading records…"</p> }>
                {move || records.get().map(|result| match result {
                    Err(_) => view! { <LoadError retry=Callback::new(move |_| records.refetch())/> }.into_any(),
                    Ok(response) => {
                        let count = response.items.len();
                        view! {
                            <section class="explorer-results section-stack section-stack--related" aria-label="Search results">
                                <p role="status">{format!("{} matching {}", response.stats.total, if response.stats.total == 1 { "record" } else { "records" })}</p>
                                {response.is_truncated.then(|| view! { <p class="feedback">{format!("Showing the first {count} matches. Narrow your search to find the remaining records.")}</p> })}
                                {if response.items.is_empty() {
                                    view! {
                                        <div class="empty-state section-stack section-stack--related">
                                            <h2>"No matching records"</h2>
                                            <p>"Clear the filters or add your first record below."</p>
                                            <A href=BASE>"Show all records"</A>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <table class="resource-table">
                                            <caption class="visually-hidden">"Records matching the current search and status"</caption>
                                            <thead><tr><th scope="col">"Record"</th><th scope="col">"Status"</th><th scope="col" class="resource-table__date">"Created"</th></tr></thead>
                                            <tbody>
                                                {response.items.into_iter().map(|item| view! {
                                                    <ResourceRow item=item suffix=filter_suffix(&query.get())/>
                                                }).collect_view()}
                                            </tbody>
                                        </table>
                                    }.into_any()
                                }}
                            </section>
                        }.into_any()
                    }
                })}
            </Suspense>

            <details class="explorer-create">
                <summary>"Add a record"</summary>
                <ActionForm action=create attr:class="section-stack section-stack--related">
                    <label class="explorer-field" for="new-resource-title">
                        <span class="composer-label">"New record title"</span>
                        <input id="new-resource-title" class="composer-input" name="title" required maxlength=crate::todos::MAX_TITLE_BYTES
                            prop:value=move || draft.get() on:input=move |ev| draft.set(event_target_value(&ev))/>
                    </label>
                    <div class="action-cluster">
                        <button type="submit" class="control-frame control-frame--primary" disabled=move || !hydrated.get() || create.pending().get()>
                            {move || if create.pending().get() { "Adding…" } else { "Add record" }}
                        </button>
                    </div>
                </ActionForm>
                <Show when=move || create.value().get().is_some()>
                    <p class="feedback" role="status">
                        {move || create.value().get().map(|result| match result {
                            Ok(item) => view! { "Record added. " <A href=format!("{BASE}/{}{}", item.id, filter_suffix(&query.get()))>"Open the new record"</A> }.into_any(),
                            Err(error) => view! { <span>{format!("Could not add the record. {error}")}</span> }.into_any(),
                        })}
                    </p>
                </Show>
            </details>
            <noscript><p class="feedback">"Search and record links work without JavaScript. Enable JavaScript to add records or review edits."</p></noscript>
            <details class="explorer-create">
                <summary>"Reuse this workflow"</summary>
                <div class="section-stack section-stack--related">
                    <p>"Open these files in your checkout. The gallery renders the same components you can adapt."</p>
                    <code>"patterns/resource-explorer/README.md"</code>
                    <code>"src/components/resource_explorer/"</code>
                    <p>"Keep the URL filters and draft/review flow. Replace the record fields, related-record rule, and server commands for your domain."</p>
                    <p>"Run the complete local checks after adapting:"</p>
                    <code>"./scripts/verify.sh"</code>
                </div>
            </details>
            <p class="composer-hint">"Saved records also appear in the lab. "<A href="/lab">"Open the lab"</A></p>
        </div>
    }
}

#[component]
fn ResourceRow(item: TodoItem, suffix: String) -> impl IntoView {
    view! {
        <tr>
            <th scope="row"><A href=format!("{BASE}/{}{suffix}", item.id)>{item.title}</A><span class="resource-id">{format!("Record #{}", item.id)}</span></th>
            <td><span class="resource-status" class:resource-status--completed=item.completed>{query::status_label(item.completed)}</span></td>
            <td class="resource-table__date">{item.created_at}</td>
        </tr>
    }
}

#[component]
fn LoadError(retry: Callback<()>) -> impl IntoView {
    view! {
        <div class="feedback feedback--error section-stack section-stack--related" role="alert">
            <p>"Records could not be loaded. Your filters are still in the address bar."</p>
            <div><button type="button" class="control-frame control-frame--secondary" on:click=move |_| retry.run(())>"Retry loading"</button></div>
        </div>
    }
}
