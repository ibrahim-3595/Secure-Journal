use dioxus::prelude::*;
use std::time::Duration;
use crate::{components::navbar::Navbar, Route, state::AppState};

#[component]
pub fn NewEntry() -> Element {
    let app_state = use_context::<Signal<AppState>>();
    let nav = navigator();
    let mut title = use_signal(|| String::new());
    let mut content = use_signal(|| String::new());
    let mut tags = use_signal(|| String::new());
    let mut success = use_signal(|| false);
    let mut loading = use_signal(|| false);

    // Redirect if not logged in
    if !app_state().logged_in {
        nav.push(Route::Login {});
        return rsx! { div {} };
    }

    let handle_save = move |_| {
        let title_val = title();
        let content_val = content();
        // let tags_val = tags();

        if title_val.is_empty() || content_val.is_empty() {
            return;
        }

        spawn(async move {
            loading.set(true);
            
            // TODO: Save to backend using api::create_entry()
            // let entry = JournalEntry {
            //     id: None,
            //     title: title_val,
            //     content: content_val,
            //     tags: tags_val.split(',').map(|s| s.trim().to_string()).collect(),
            //     created_at: chrono::Local::now().to_string(),
            // };
            // match api::create_entry(entry).await { ... }
            
            // Simulate API call
            #[cfg(target_arch = "wasm32")]
            gloo_timers::future::sleep(Duration::from_secs(1)).await;
            
            #[cfg(not(target_arch = "wasm32"))]
            async_std::task::sleep(Duration::from_secs(1)).await;
            
            loading.set(false);
            success.set(true);
            
            // Redirect delay
            #[cfg(target_arch = "wasm32")]
            gloo_timers::future::sleep(Duration::from_secs(2)).await;
            
            #[cfg(not(target_arch = "wasm32"))]
            async_std::task::sleep(Duration::from_secs(2)).await;
            
            nav.push(Route::Entries {});
        });
    };

    rsx! {
        div {
            class: "min-h-screen bg-gradient-to-br from-green-50 to-blue-100",
            
            Navbar { show_back: true, show_logout: false }

            div {
                class: "max-w-4xl mx-auto px-4 py-12",
                h2 { class: "text-4xl font-bold text-gray-800 mb-8", "✍️ New Journal Entry" }

                if success() {
                    div {
                        class: "bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded-lg mb-4",
                        "Entry saved successfully! Redirecting..."
                    }
                }

                div {
                    class: "bg-white rounded-2xl shadow-xl p-8",
                    div {
                        class: "space-y-6",
                        div {
                            label {
                                class: "block text-sm font-medium text-gray-700 mb-2",
                                "Title"
                            }
                            input {
                                class: "w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition",
                                r#type: "text",
                                placeholder: "Give your entry a title",
                                value: "{title()}",
                                oninput: move |e| title.set(e.value().clone()),
                            }
                        }

                        div {
                            label {
                                class: "block text-sm font-medium text-gray-700 mb-2",
                                "Content"
                            }
                            textarea {
                                class: "w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition min-h-64",
                                placeholder: "Write your thoughts...",
                                value: "{content()}",
                                oninput: move |e| content.set(e.value().clone()),
                            }
                        }

                        div {
                            label {
                                class: "block text-sm font-medium text-gray-700 mb-2",
                                "Tags (comma-separated)"
                            }
                            input {
                                class: "w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition",
                                r#type: "text",
                                placeholder: "personal, work, goals",
                                value: "{tags()}",
                                oninput: move |e| tags.set(e.value().clone()),
                            }
                        }

                        button {
                            class: "w-full bg-indigo-600 hover:bg-indigo-700 text-white font-semibold py-3 px-4 rounded-lg transition duration-200 transform hover:scale-105",
                            disabled: loading() || title().is_empty() || content().is_empty(),
                            onclick: handle_save,
                            if loading() {
                                "Saving..."
                            } else {
                                "💾 Save Entry"
                            }
                        }
                    }
                }
            }
        }
    }
}