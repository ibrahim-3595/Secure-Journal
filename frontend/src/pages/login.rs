use dioxus::prelude::*;
use crate::{api, Route, state::AppState};

#[component]
pub fn Login() -> Element {
    let nav = navigator();
    let mut app_state = use_context::<Signal<AppState>>();
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error_msg = use_signal(|| String::new());
    let mut loading = use_signal(|| false);
    let mut show_password = use_signal(|| false);

    let handle_login = move |_| {
        let username_val = username();
        let password_val = password();

        spawn(async move {
            loading.set(true);
            error_msg.set(String::new());

            match api::login(username_val.clone(), password_val).await {
                Ok(auth_resp) => {
                    if auth_resp.ok {
                        app_state.write().logged_in = true;
                        app_state.write().username = username_val;
                        nav.push(Route::Home {});
                    } else {
                        error_msg.set(auth_resp.message);
                    }
                }
                Err(e) => error_msg.set(e),
            }

            loading.set(false);
        });
    };

    rsx! {
        div {
            class: "min-h-screen
                    bg-gradient-to-br from-indigo-600 via-purple-600 to-pink-500
                    flex items-center justify-center p-6",

            div {
                class: "bg-white/10 backdrop-blur-xl shadow-2xl 
                        rounded-3xl p-10 w-full max-w-lg
                        transition-all duration-300 hover:scale-[1.02]",

                // HEADER
                div {
                    class: "text-center mb-10",
                    h1 {
                        class: "text-5xl font-extrabold text-white drop-shadow-lg",
                        "🔐 Secure Journal"
                    }
                    p {
                        class: "text-white/80 mt-3 text-lg",
                        "Your encrypted private writing space"
                    }
                }

                // ERROR BOX
                if !error_msg().is_empty() {
                    div {
                        class: "bg-red-500/20 border border-red-400 text-red-200
                                px-4 py-3 rounded-xl mb-5 animate-pulse",
                        "{error_msg()}"
                    }
                }

                // FORM FIELDS
                div {
                    class: "space-y-6",

                    // USERNAME
                    div {
                        label {
                            class: "block text-white/80 text-sm font-semibold mb-2",
                            "Username"
                        }
                        input {
                            class: "w-full px-4 py-3 rounded-xl bg-white/20 
                                    text-white backdrop-blur-lg border border-white/30
                                    focus:outline-none focus:ring-2 focus:ring-yellow-300
                                    placeholder-white/50 transition",
                            r#type: "text",
                            placeholder: "Enter username",
                            value: "{username()}",
                            oninput: move |e| username.set(e.value().clone()),
                        }
                    }

                    // PASSWORD
                    div {
                        label {
                            class: "block text-white/80 text-sm font-semibold mb-2",
                            "Password"
                        }
                        div {
                            class: "relative",
                            input {
                                class: "w-full px-4 py-3 rounded-xl bg-white/20 
                                        text-white backdrop-blur-lg border border-white/30
                                        focus:outline-none focus:ring-2 focus:ring-yellow-300
                                        placeholder-white/50 pr-12 transition",
                                r#type: if show_password() { "text" } else { "password" },
                                placeholder: "Enter password",
                                value: "{password()}",
                                oninput: move |e| password.set(e.value().clone()),
                            }
                            button {
                                class: "absolute right-3 top-3 text-white/80 hover:text-white",
                                onclick: move |_| show_password.set(!show_password()),
                                if show_password() { "🙈" } else { "👁️" }
                            }
                        }
                    }

                    // LOGIN BUTTON
                    button {
                        class: "w-full bg-yellow-300 hover:bg-yellow-400 
                                text-gray-900 font-bold py-3 rounded-xl
                                transition transform hover:scale-[1.03]
                                shadow-lg",
                        disabled: loading(),
                        onclick: handle_login,

                        if loading() {
                            span { class: "animate-spin mr-2 inline-block", "⏳" }
                            "Logging in..."
                        } else {
                            "Login"
                        }
                    }

                    // SIGNUP LINK
                    div {
                        class: "text-center mt-6",
                        span { class: "text-white/80", "Don't have an account? " }
                        Link {
                            to: Route::Signup {},
                            class: "text-yellow-300 hover:text-yellow-200 font-semibold underline",
                            "Sign up"
                        }
                    }
                }
            }
        }
    }
}
