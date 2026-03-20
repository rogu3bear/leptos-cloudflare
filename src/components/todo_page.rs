use leptos::{ev::SubmitEvent, prelude::*};

use crate::api::{list_todos, CreateTodo, DeleteTodo, TodoItem, TodosResponse, ToggleTodo};

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
            local_error.set(Some("Give the todo a short title first.".to_string()));
            return;
        }

        local_error.set(None);
        create_action.dispatch(CreateTodo { title });
    };

    view! {
        <main class="page-shell">
            <section class="hero">
                <p class="eyebrow">"Leptos + Cloudflare Workers + D1"</p>
                <div class="hero-grid">
                    <div class="hero-copy">
                        <h1>"Ship a real full-stack starter, not a demo button."</h1>
                        <p class="hero-lede">
                            "This template keeps the official Workers deployment model, but adds a
                            public-ready todo domain, D1 persistence, server-function wiring, and
                            an opinionated default UI that is meant to be modified."
                        </p>
                    </div>

                    <form class="composer-card" on:submit=on_submit>
                        <label class="composer-label" for="todo-title">
                            "Create a task"
                        </label>
                        <div class="composer-row">
                            <input
                                id="todo-title"
                                class="composer-input"
                                type="text"
                                name="title"
                                placeholder="Ship the login page"
                                autocomplete="off"
                                prop:value=move || draft.get()
                                on:input=move |ev| draft.set(event_target_value(&ev))
                            />
                            <button class="composer-button" type="submit" disabled=submit_disabled>
                                {move || {
                                    if create_action.pending().get() {
                                        "Saving..."
                                    } else {
                                        "Add Todo"
                                    }
                                }}
                            </button>
                        </div>
                        <p class="composer-hint">
                            "Mutations go through Leptos server functions and write directly to D1."
                        </p>
                    </form>
                </div>
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

            {move || match todos.get() {
                None => view! { <LoadingState/> }.into_any(),
                Some(Err(error)) => view! {
                    <section class="panel error-panel">
                        <h2>"Couldn’t load todos"</h2>
                        <p>{error.to_string()}</p>
                        <button
                            class="ghost-button"
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
        </main>
    }
}

#[component]
fn TodoBoard(
    data: TodosResponse,
    toggle_action: ServerAction<ToggleTodo>,
    delete_action: ServerAction<DeleteTodo>,
) -> impl IntoView {
    let TodosResponse { items, stats } = data;
    let has_items = !items.is_empty();
    let items = std::sync::Arc::new(items);
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
                    "Create your first todo to verify the D1 migration, server functions,
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
        </section>

        <section class="panel">
            <div class="panel-head">
                <div>
                    <h2>"Todo Flow"</h2>
                    <p>"Server-rendered on first load, hydrated in the browser after that."</p>
                </div>
                <span class="pill">
                    {if has_items { "Live D1 Data" } else { "Ready for your first task" }}
                </span>
            </div>

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
                class="todo-toggle"
                type="button"
                disabled=move || is_toggling() || is_deleting()
                on:click=move |_| {
                    toggle_action.dispatch(ToggleTodo { id });
                }
            >
                {move || if optimistic_completed() { "Done" } else { "Open" }}
            </button>

            <div class="todo-copy">
                <h3>{title.clone()}</h3>
                <p>
                    {move || {
                        if is_toggling() {
                            "Saving status change...".to_string()
                        } else if is_deleting() {
                            "Removing todo...".to_string()
                        } else {
                            created_at.clone()
                        }
                    }}
                </p>
            </div>

            <button
                class="todo-delete"
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
            <div class="skeleton skeleton--title"></div>
            <div class="skeleton skeleton--row"></div>
            <div class="skeleton skeleton--row"></div>
            <div class="skeleton skeleton--row"></div>
        </section>
    }
}
