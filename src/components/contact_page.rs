use leptos::{ev::SubmitEvent, prelude::*};

use crate::api::SubmitContact;

#[component]
pub fn ContactPage() -> impl IntoView {
    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let topic = RwSignal::new(String::new());
    let message = RwSignal::new(String::new());
    let website = RwSignal::new(String::new());
    let local_error = RwSignal::new(None::<String>);

    let submit_action = ServerAction::<SubmitContact>::new();

    Effect::new(move |_| {
        if let Some(Ok(response)) = submit_action.value().get() {
            if response.accepted {
                name.set(String::new());
                email.set(String::new());
                topic.set(String::new());
                message.set(String::new());
                website.set(String::new());
                local_error.set(None);
            }
        }
    });

    let server_message = move || {
        submit_action.value().get().and_then(|result| match result {
            Ok(response) => Some(response.message),
            Err(error) => Some(error.to_string()),
        })
    };
    let submit_disabled = move || {
        submit_action.pending().get()
            || name.with(|value| value.trim().is_empty())
            || email.with(|value| value.trim().is_empty())
            || topic.with(|value| value.trim().is_empty())
            || message.with(|value| value.trim().is_empty())
    };

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        if name.with(|value| value.trim().is_empty())
            || email.with(|value| value.trim().is_empty())
            || topic.with(|value| value.trim().is_empty())
            || message.with(|value| value.trim().is_empty())
        {
            local_error.set(Some(
                "Complete every visible field before sending.".to_string(),
            ));
            return;
        }

        local_error.set(None);
        submit_action.dispatch(SubmitContact {
            name: name.get_untracked(),
            email: email.get_untracked(),
            topic: topic.get_untracked(),
            message: message.get_untracked(),
            website: website.get_untracked(),
        });
    };

    view! {
        <main class="page-shell">
            <section class="hero contact-hero">
                <p class="eyebrow">"Public contact route"</p>
                <div class="hero-grid contact-grid">
                    <div class="hero-copy">
                        <h1>"Contact"</h1>
                        <p class="hero-lede">
                            "A bounded edge intake path with server-side validation, D1 persistence,
                            and session-scoped abuse controls."
                        </p>
                    </div>

                    <form class="composer-card contact-card" on:submit=on_submit>
                        <label class="composer-label" for="contact-name">"Name"</label>
                        <input
                            id="contact-name"
                            class="composer-input"
                            type="text"
                            name="name"
                            autocomplete="name"
                            maxlength="80"
                            prop:value=move || name.get()
                            on:input=move |ev| name.set(event_target_value(&ev))
                        />

                        <label class="composer-label" for="contact-email">"Email"</label>
                        <input
                            id="contact-email"
                            class="composer-input"
                            type="email"
                            name="email"
                            autocomplete="email"
                            maxlength="254"
                            prop:value=move || email.get()
                            on:input=move |ev| email.set(event_target_value(&ev))
                        />

                        <label class="composer-label" for="contact-topic">"Topic"</label>
                        <input
                            id="contact-topic"
                            class="composer-input"
                            type="text"
                            name="topic"
                            autocomplete="off"
                            maxlength="120"
                            prop:value=move || topic.get()
                            on:input=move |ev| topic.set(event_target_value(&ev))
                        />

                        <label class="composer-label" for="contact-message">"Message"</label>
                        <textarea
                            id="contact-message"
                            class="composer-input contact-message"
                            name="message"
                            rows="7"
                            maxlength="1800"
                            prop:value=move || message.get()
                            on:input=move |ev| message.set(event_target_value(&ev))
                        ></textarea>

                        <label class="contact-hidden-field" aria-hidden="true">
                            "Website"
                            <input
                                type="text"
                                name="website"
                                autocomplete="off"
                                tabindex="-1"
                                prop:value=move || website.get()
                                on:input=move |ev| website.set(event_target_value(&ev))
                            />
                        </label>

                        <button class="composer-button contact-submit" type="submit" disabled=submit_disabled>
                            {move || {
                                if submit_action.pending().get() {
                                    "Sending..."
                                } else {
                                    "Send"
                                }
                            }}
                        </button>
                    </form>
                </div>
            </section>

            <Show when=move || local_error.get().is_some() || server_message().is_some()>
                <div
                    class="feedback"
                    class:feedback--error=move || local_error.get().is_some()
                    role="status"
                >
                    {move || local_error.get().or_else(server_message).unwrap_or_else(String::new)}
                </div>
            </Show>
        </main>
    }
}
