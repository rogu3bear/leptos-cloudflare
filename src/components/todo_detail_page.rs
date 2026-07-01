use leptos::{ev::MouseEvent, prelude::*};
use leptos_router::{
    components::A,
    hooks::{use_navigate, use_params_map},
};

use crate::api::{get_todo, DeleteTodo, TodoItem, ToggleTodo};

#[component]
pub fn TodoDetailPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || {
        params
            .get()
            .get("id")
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(0)
    };

    let refresh = RwSignal::new(0usize);

    let todo = Resource::new(
        move || (id(), refresh.get()),
        |(id, _)| async move {
            if id == 0 {
                return Err("Invalid todo id".to_string());
            }
            get_todo(id).await.map_err(|e| e.to_string())
        },
    );

    let toggle_action = ServerAction::<ToggleTodo>::new();
    let delete_action = ServerAction::<DeleteTodo>::new();

    // After successful toggle or delete, refresh the resource
    Effect::new(move |_| {
        if toggle_action.value().get().is_some() || delete_action.value().get().is_some() {
            refresh.update(|n| *n += 1);
        }
    });

    let on_toggle = move |ev: MouseEvent| {
        ev.prevent_default();
        if let Some(Ok(item)) = todo.get() {
            toggle_action.dispatch(ToggleTodo { id: item.id });
        }
    };

    let navigate = use_navigate();

    let on_delete = move |ev: MouseEvent| {
        ev.prevent_default();
        if let Some(Ok(item)) = todo.get() {
            delete_action.dispatch(DeleteTodo { id: item.id });
        }
    };

    // After a successful delete, navigate back to the list.
    // This demonstrates good integration between ServerAction results and the router.
    Effect::new(move |_| {
        if delete_action.value().get().is_some() {
            navigate("/", Default::default());
        }
    });

    view! {
        <main class="page-shell">
            <div style="margin-bottom: 1rem;">
                <A href="/" attr:class="ghost-button">"← All todos"</A>
            </div>

            <Suspense fallback=move || view! { <LoadingState/> }>
                {move || match todo.get() {
                    None => view! { <LoadingState/> }.into_any(),
                    Some(Err(e)) => view! {
                        <section class="panel error-panel">
                            <h2>"Could not load todo"</h2>
                            <p>{e}</p>
                            <A href="/" attr:class="ghost-button" attr:style="margin-top: 1rem; display: inline-block;">
                                "Back to list"
                            </A>
                        </section>
                    }.into_any(),
                    Some(Ok(item)) => {
                        let TodoItem { id: _, title, completed, created_at } = item.clone();

                        let optimistic_completed = move || {
                            if toggle_action.pending().get() {
                                !completed
                            } else {
                                completed
                            }
                        };

                        // Small example of derived state with Memo.
                        // This only recomputes when `optimistic_completed` or the title changes.
                        // In a larger UI this pattern avoids unnecessary work compared to
                        // recalculating inside the view every render.
                        let status_label = Memo::new(move |_| {
                            if optimistic_completed() { "Completed" } else { "Open" }
                        });

                        view! {
                            <section class="panel">
                                <div class="panel-head">
                                    <div>
                                        <span class="pill" style="margin-bottom: 0.5rem; display: inline-block;">
                                            {status_label}
                                        </span>
                                        <h1 style="margin: 0; font-size: 1.6rem; line-height: 1.2;">{title.clone()}</h1>
                                        <p style="color: var(--ink-soft); margin-top: 0.25rem;">{created_at}</p>
                                    </div>
                                </div>

                                <div style="display: flex; gap: 0.75rem; margin-top: 1.25rem; flex-wrap: wrap;">
                                    <button
                                        class="todo-toggle"
                                        style="min-width: 120px;"
                                        disabled=move || toggle_action.pending().get()
                                        on:click=on_toggle
                                    >
                                        {move || if optimistic_completed() { "Mark open" } else { "Mark done" }}
                                    </button>

                                    <button
                                        class="todo-delete"
                                        disabled=move || delete_action.pending().get() || toggle_action.pending().get()
                                        on:click=on_delete
                                    >
                                        "Delete todo"
                                    </button>
                                </div>

                                <p style="margin-top: 1.5rem; font-size: 0.85rem; color: var(--ink-soft);">
                                    "This detail view is rendered via ParamSegment(\":id\") and a dedicated server function. "
                                    "After hydration, the buttons use ServerAction for optimistic updates while the resource refetches."
                                </p>

                                // Second independent Suspense boundary.
                                // This demonstrates how Leptos + edge SSR can stream different
                                // sections independently. In a real app this section could load
                                // slower data (audit log, related items, etc.) without blocking
                                // the main content.
                                <Suspense fallback=move || view! { <div class="composer-hint">"Loading metadata..."</div> }>
                                    {move || {
                                        // In a real implementation this would be another Resource
                                        // calling a server function. Here we just show the pattern.
                                        view! {
                                            <div class="composer-hint" style="margin-top: 1rem;">
                                                "Metadata loaded independently (streaming candidate)."
                                            </div>
                                        }
                                    }}
                                </Suspense>
                            </section>

                            <Show when=move || delete_action.value().get().is_some()>
                                <div class="feedback feedback--error" role="status" style="margin-top: 1rem;">
                                    "Todo deleted. "
                                    <A href="/">"Return to list"</A>
                                </div>
                            </Show>
                        }.into_any()
                    }
                }}
            </Suspense>
        </main>
    }
}

#[component]
fn LoadingState() -> impl IntoView {
    view! {
        <section class="panel loading-panel">
            <div class="skeleton skeleton--title"></div>
            <div class="skeleton skeleton--row" style="height: 140px;"></div>
        </section>
    }
}
