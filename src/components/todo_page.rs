use leptos::{ev::SubmitEvent, prelude::*};
use leptos_meta::{Meta, Title};
use leptos_router::components::A;

use crate::api::{list_todos, CreateTodo, DeleteTodo, TodoItem, TodosResponse, ToggleTodo};

use super::ui::{EvidenceKind, EvidenceTag};

#[component]
pub fn TodoPage() -> impl IntoView {
    let draft = RwSignal::new(String::new());
    let local_error = RwSignal::new(None::<String>);
    let refresh_nonce = RwSignal::new(0usize);

    let create_action = ServerAction::<CreateTodo>::new();
    let toggle_action = ServerAction::<ToggleTodo>::new();
    let delete_action = ServerAction::<DeleteTodo>::new();

    let todos = Resource::new(
        move || {
            (
                refresh_nonce.get(),
                create_action.version().get(),
                toggle_action.version().get(),
                delete_action.version().get(),
            )
        },
        |_| async move { list_todos().await },
    );

    Effect::new(move |_| {
        if let Some(Ok(_)) = create_action.value().get() {
            draft.set(String::new());
            local_error.set(None);
        }
    });

    let server_error = move || {
        create_action
            .value()
            .get()
            .and_then(|result| result.err().map(|error| error.to_string()))
            .or_else(|| {
                toggle_action
                    .value()
                    .get()
                    .and_then(|result| result.err().map(|error| error.to_string()))
            })
            .or_else(|| {
                delete_action
                    .value()
                    .get()
                    .and_then(|result| result.err().map(|error| error.to_string()))
            })
    };

    let submit_disabled =
        move || create_action.pending().get() || draft.with(|value| value.trim().is_empty());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let title = draft.get_untracked().trim().to_string();
        if title.is_empty() {
            local_error.set(Some("Give the task a short title first.".to_string()));
            return;
        }

        local_error.set(None);
        create_action.dispatch(CreateTodo { title });
    };

    view! {
        <Title text="Local lab — Leptos CF"/>
        <Meta name="description" content="A bounded D1-backed mutation lab for the leptos-cf starter."/>

        <div class="page-shell section-stack section-stack--section">
            <section class="lab-intro">
                <div class="section-stack section-stack--related">
                    <p class="eyebrow">"Field note 005 · local mutation lab"</p>
                    <h1>"Real server actions. Honest limits."</h1>
                    <p class="page-lede">
                        "Create, toggle, inspect, and delete session-scoped records through Leptos server functions and local D1. This is an inspectable implementation specimen—not a hosted task product or live trace."
                    </p>
                    <div class="lab-proof"><EvidenceTag kind=EvidenceKind::Browser/><span>"State is observable in this browser session after hydration."</span></div>
                </div>

                <form class="composer-card" on:submit=on_submit>
                    <div class="section-stack section-stack--tight">
                        <label class="composer-label" for="todo-title">
                            "Create a task record"
                        </label>
                        <p class="composer-hint">"The action is disabled until the title is non-empty."</p>
                    </div>
                    <div class="composer-row">
                        <input
                            id="todo-title"
                            class="composer-input"
                            type="text"
                            name="title"
                            placeholder="Map the deployment boundary"
                            autocomplete="off"
                            prop:value=move || draft.get()
                            on:input=move |ev| draft.set(event_target_value(&ev))
                        />
                        <button class="composer-button control-frame" type="submit" disabled=submit_disabled>
                            {move || {
                                if create_action.pending().get() {
                                    "Saving…"
                                } else {
                                    "Add task"
                                }
                            }}
                        </button>
                    </div>
                    <p class="composer-hint">
                        "Records stay isolated to the current browser session. Server-side validation and caps remain authoritative."
                    </p>
                </form>
            </section>

            <Show when=move || local_error.get().is_some() || server_error().is_some()>
                <div class="feedback feedback--error" role="status">
                    {move || {
                        local_error
                            .get()
                            .or_else(server_error)
                            .unwrap_or_else(String::new)
                    }}
                </div>
            </Show>

            <Suspense fallback=move || view! { <LoadingState/> }>
                {move || match todos.get() {
                    None => view! { <LoadingState/> }.into_any(),
                    Some(Err(error)) => view! {
                        <section class="panel error-panel">
                            <h2>"Couldn’t load task records"</h2>
                            <p>{error.to_string()}</p>
                            <button
                                class="ghost-button control-frame"
                                type="button"
                                on:click=move |_| refresh_nonce.update(|value| *value += 1)
                            >
                                "Try again"
                            </button>
                        </section>
                    }
                    .into_any(),
                    Some(Ok(data)) => view! {
                        <TodoBoard
                            data=data
                            toggle_action=toggle_action
                            delete_action=delete_action
                        />
                    }
                    .into_any(),
                }}
            </Suspense>
        </div>
    }
}

#[component]
fn TodoBoard(
    data: TodosResponse,
    toggle_action: ServerAction<ToggleTodo>,
    delete_action: ServerAction<DeleteTodo>,
) -> impl IntoView {
    let TodosResponse {
        items,
        stats,
        visible_limit,
        is_truncated,
    } = data;
    let has_items = !items.is_empty();
    let items = std::sync::Arc::new(items);

    // Demonstrates fine-grained reactivity with Memo.
    // This derived value only recomputes when `stats` changes,
    // and only the nodes that read it will update.
    let completion_rate = Memo::new(move |_| {
        if stats.total == 0 {
            0
        } else {
            (stats.completed as f32 / stats.total as f32 * 100.0) as i32
        }
    });
    let list_or_empty = if has_items {
        view! {
            <ul class="todo-list">
                <For
                    each=move || items.as_ref().clone().into_iter()
                    key=|todo| todo.id
                    children=move |todo| {
                        view! {
                            <TodoRow
                                todo=todo
                                toggle_action=toggle_action
                                delete_action=delete_action
                            />
                        }
                    }
                />
            </ul>
        }
        .into_any()
    } else {
        view! {
            <div class="empty-state">
                <h3>"Nothing in the queue yet"</h3>
                <p>
                    "Create your first task record to inspect the D1 migration, server functions,
                    and hydration path end to end."
                </p>
            </div>
        }
        .into_any()
    };

    view! {
        <section class="stats-grid">
            <article class="stat-card">
                <span class="stat-label">"Total"</span>
                <strong class="stat-value">{stats.total}</strong>
            </article>
            <article class="stat-card">
                <span class="stat-label">"Open"</span>
                <strong class="stat-value">{stats.open}</strong>
            </article>
            <article class="stat-card">
                <span class="stat-label">"Completed"</span>
                <strong class="stat-value">{stats.completed}</strong>
            </article>
            <article class="stat-card">
                <span class="stat-label">"Done"</span>
                <strong class="stat-value">{completion_rate} "%"</strong>
            </article>
        </section>

        <section class="panel">
            <div class="panel-head">
                <div>
                    <h2>"Task-board specimen"</h2>
                    <p>
                        "Server-rendered on first load, hydrated after that, and scoped to this browser session."
                    </p>
                </div>
                <span class="pill">
                    {if has_items {
                        "Session-scoped D1 data"
                    } else {
                        "Empty local queue"
                    }}
                </span>
            </div>

            <Show when=move || is_truncated>
                <p class="composer-hint">
                    {format!(
                        "Showing the newest {visible_limit} task records for this browser session so the lab stays bounded."
                    )}
                </p>
            </Show>

            {list_or_empty}
        </section>
    }
}

#[component]
fn TodoRow(
    todo: TodoItem,
    toggle_action: ServerAction<ToggleTodo>,
    delete_action: ServerAction<DeleteTodo>,
) -> impl IntoView {
    let TodoItem {
        id,
        title,
        completed,
        created_at,
    } = todo;

    let is_toggling = move || {
        toggle_action.pending().get()
            && toggle_action
                .input()
                .get()
                .as_ref()
                .map(|input| input.id == id)
                .unwrap_or(false)
    };

    let is_deleting = move || {
        delete_action.pending().get()
            && delete_action
                .input()
                .get()
                .as_ref()
                .map(|input| input.id == id)
                .unwrap_or(false)
    };

    let optimistic_completed = move || {
        if is_toggling() {
            !completed
        } else {
            completed
        }
    };

    view! {
        <li
            class="todo-row"
            class:todo-row--done=optimistic_completed
            class:todo-row--mutating=move || is_toggling() || is_deleting()
        >
            <button
                class="todo-toggle control-frame control-frame--compact"
                type="button"
                disabled=move || is_toggling() || is_deleting()
                on:click=move |_| {
                    toggle_action.dispatch(ToggleTodo { id });
                }
            >
                {move || if optimistic_completed() { "Done" } else { "Open" }}
            </button>

            <div class="todo-copy">
                <h3>
                    <A
                        href=format!("/lab/{}", id)
                        attr:class="todo-title-link"
                    >
                        {title.clone()}
                    </A>
                </h3>
                <p>
                    {move || {
                        if is_toggling() {
                            "Saving status change...".to_string()
                        } else if is_deleting() {
                            "Removing task…".to_string()
                        } else {
                            created_at.clone()
                        }
                    }}
                </p>
            </div>

            <button
                class="todo-delete control-frame control-frame--compact"
                type="button"
                disabled=move || is_deleting() || is_toggling()
                on:click=move |_| {
                    delete_action.dispatch(DeleteTodo { id });
                }
            >
                "Delete"
            </button>
        </li>
    }
}

#[component]
fn LoadingState() -> impl IntoView {
    view! {
        <section class="panel loading-panel">
            <div class="skeleton skeleton--title" aria-hidden="true"></div>
            <div class="skeleton skeleton--row" aria-hidden="true"></div>
            <div class="skeleton skeleton--row" aria-hidden="true"></div>
            <div class="skeleton skeleton--row" aria-hidden="true"></div>
            <span class="visually-hidden">"Loading task records"</span>
        </section>
    }
}
