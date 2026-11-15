use leptos::{ev::SubmitEvent, prelude::*};

use super::query::status_label;
use crate::api::{normalize_todo_title, TodoItem, TodoUpdateOutcome, UpdateTodo};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Step {
    View,
    Edit,
    Review,
}

/// Drafts belong to this mounted record. Only an acknowledged server result
/// changes `saved`; failures and conflicts keep the draft available.
#[component]
pub fn ResourceEditor(saved: RwSignal<TodoItem>, on_saved: Callback<TodoItem>) -> impl IntoView {
    let step = RwSignal::new(Step::View);
    let title = RwSignal::new(saved.with_untracked(|item| item.title.clone()));
    let completed = RwSignal::new(saved.with_untracked(|item| item.completed));
    let error = RwSignal::new(None::<String>);
    let conflict = RwSignal::new(None::<TodoItem>);
    let saved_notice = RwSignal::new(false);
    let hydrated = RwSignal::new(false);
    let heading = NodeRef::<leptos::html::H2>::new();
    let save = ServerAction::<UpdateTodo>::new();
    Effect::new(move |_| hydrated.set(true));

    Effect::new(move |previous: Option<Step>| {
        let current = step.get();
        if previous.is_some_and(|previous| previous != current) {
            if let Some(heading) = heading.get_untracked() {
                let _ = heading.focus();
            }
        }
        current
    });
    Effect::new(move |_| {
        if let Some(result) = save.value().get() {
            match result {
                Ok(TodoUpdateOutcome::Saved(item)) => {
                    saved.set(item.clone());
                    on_saved.run(item);
                    step.set(Step::View);
                    saved_notice.set(true);
                    error.set(None);
                    conflict.set(None);
                }
                Ok(TodoUpdateOutcome::Conflict(current)) => {
                    conflict.set(Some(current));
                    error.set(Some("This record changed elsewhere. Your draft is preserved. Review the latest saved values before trying again.".into()));
                }
                Ok(TodoUpdateOutcome::Unavailable) => {
                    error.set(Some("This record is no longer available in this browser session. Your draft is preserved here until you leave.".into()));
                }
                Err(_) => error.set(Some("The save could not be confirmed. Your draft is preserved. Retry saving; an already applied change will be recognized.".into())),
            }
        }
    });

    let start_edit = move |_| {
        title.set(saved.with_untracked(|item| item.title.clone()));
        completed.set(saved.with_untracked(|item| item.completed));
        error.set(None);
        conflict.set(None);
        saved_notice.set(false);
        save.value().set(None);
        step.set(Step::Edit);
    };
    let review = move |event: SubmitEvent| {
        event.prevent_default();
        if !hydrated.get_untracked() || save.pending().get_untracked() {
            return;
        }
        match normalize_todo_title(&title.get_untracked()) {
            Ok(normalized) => {
                title.set(normalized);
                if conflict.get_untracked().is_none() {
                    error.set(None);
                }
                step.set(Step::Review);
            }
            Err(message) => error.set(Some(message.to_string())),
        }
    };
    let submit = move |_| {
        if step.get_untracked() != Step::Review
            || save.pending().get_untracked()
            || conflict.get_untracked().is_some()
        {
            return;
        }
        let original = saved.get_untracked();
        error.set(None);
        save.value().set(None);
        save.dispatch(UpdateTodo {
            id: original.id,
            title: title.get_untracked(),
            completed: completed.get_untracked(),
            expected_title: original.title,
            expected_completed: original.completed,
        });
    };

    view! {
        <section class="explorer-record section-stack section-stack--related" aria-busy=move || save.pending().get()>
            <header class="explorer-heading section-stack section-stack--related">
                <p class="eyebrow">{move || saved.with(|item| format!("Record #{}", item.id))}</p>
                <h1 class="detail-title">{move || saved.with(|item| item.title.clone())}</h1>
                <p><span class="resource-status" class:resource-status--completed=move || saved.with(|item| item.completed)>{move || status_label(saved.with(|item| item.completed))}</span></p>
                <p class="composer-hint">{move || saved.with(|item| format!("Created {} · Saved to this browser session", item.created_at))}</p>
            </header>

            <h2 class="explorer-step-heading" tabindex="-1" node_ref=heading>
                {move || match step.get() { Step::View => "Record details", Step::Edit => "Edit draft", Step::Review => "Review changes" }}
            </h2>
            <Show when=move || saved_notice.get()><p class="feedback" role="status">"Changes saved. The record above shows the saved values."</p></Show>
            <Show when=move || step.get() == Step::View>
                <div><button type="button" class="control-frame control-frame--primary" disabled=move || !hydrated.get() on:click=start_edit>"Edit record"</button></div>
            </Show>

            <form on:submit=review hidden=move || step.get() != Step::Edit class="section-stack section-stack--related">
                <label class="explorer-field" for="resource-title">
                    <span class="composer-label">"Title"</span>
                    <input id="resource-title" class="composer-input" required maxlength=crate::todos::MAX_TITLE_BYTES autocomplete="off"
                        value=move || title.get() prop:value=move || title.get() on:input=move |ev| title.set(event_target_value(&ev))/>
                </label>
                <label class="explorer-field" for="edit-resource-status">
                    <span class="composer-label">"Status"</span>
                    <select id="edit-resource-status" class="composer-input" prop:value=move || if completed.get() { "completed" } else { "open" }
                        on:change=move |ev| completed.set(event_target_value(&ev) == "completed")>
                        <option value="open" selected=move || !completed.get()>"Open"</option>
                        <option value="completed" selected=move || completed.get()>"Completed"</option>
                    </select>
                </label>
                <p class="composer-hint">"Nothing is saved until you review and choose Save changes. Leaving this page discards the draft."</p>
                <div class="action-cluster">
                    <button type="submit" class="control-frame control-frame--primary" disabled=move || !hydrated.get()>"Review changes"</button>
                    <button type="button" class="control-frame control-frame--secondary" on:click=move |_| { error.set(None); step.set(Step::View); }>"Discard draft"</button>
                </div>
            </form>

            <Show when=move || step.get() == Step::Review>
                <div class="section-stack section-stack--related">
                    <dl class="review-values">
                        <div><dt>"Saved title"</dt><dd>{move || saved.with(|item| item.title.clone())}</dd></div>
                        <div><dt>"Draft title"</dt><dd>{move || title.get()}</dd></div>
                        <div><dt>"Saved status"</dt><dd>{move || status_label(saved.with(|item| item.completed))}</dd></div>
                        <div><dt>"Draft status"</dt><dd>{move || status_label(completed.get())}</dd></div>
                    </dl>
                    <div class="action-cluster">
                        <button type="button" class="control-frame control-frame--primary" disabled=move || save.pending().get() || conflict.get().is_some() on:click=submit>
                            {move || if save.pending().get() { "Saving…" } else { "Save changes" }}
                        </button>
                        <button type="button" class="control-frame control-frame--secondary" disabled=move || save.pending().get()
                            on:click=move |_| { if conflict.get_untracked().is_none() { error.set(None); } step.set(Step::Edit); }>"Back to editing"</button>
                    </div>
                    <Show when=move || save.pending().get()><p role="status">"Saving your reviewed changes…"</p></Show>
                </div>
            </Show>
            <Show when=move || error.get().is_some()>
                <div class="feedback feedback--error section-stack section-stack--related" role="alert">
                    <p>{move || error.get().unwrap_or_default()}</p>
                    <Show when=move || conflict.get().is_some()>
                        <button type="button" class="control-frame control-frame--secondary" on:click=move |_| {
                            if let Some(current) = conflict.get_untracked() {
                                match normalize_todo_title(&title.get_untracked()) {
                                    Ok(normalized) => title.set(normalized),
                                    Err(message) => { error.set(Some(message.to_string())); return; }
                                }
                                saved.set(current.clone());
                                on_saved.run(current);
                                conflict.set(None);
                                error.set(None);
                                save.value().set(None);
                                step.set(Step::Review);
                            }
                        }>"Review latest saved values"</button>
                    </Show>
                </div>
            </Show>
        </section>
    }
}
