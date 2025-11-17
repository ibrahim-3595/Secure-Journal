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
                Err(e) => {
                    error_msg.set(e);
                }
            }
            
            loading.set(false);
        });
    };

    rsx! {
        div {
            class: "min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 flex items-center justify-center p-4",
            div {
                class: "bg-white rounded-2xl shadow-2xl p-8 w-full max-w-md",
                div {
                    class: "text-center mb-8",
                    h1 { 
                        class: "text-4xl font-bold text-gray-800 mb-2",
                        "📔 Secure Journal"
                    }
                    p { 
                        class: "text-gray-600",
                        "Login to access your private journal"
                    }
                }

                if !error_msg().is_empty() {
                    div {
                        class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded-lg mb-4",
                        "{error_msg()}"
                    }
                }

                div {
                    class: "space-y-4",
                    div {
                        label {
                            class: "block text-sm font-medium text-gray-700 mb-2",
                            "Username"
                        }
                        input {
                            class: "w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition",
                            r#type: "text",
                            placeholder: "Enter your username",
                            value: "{username()}",
                            oninput: move |e| username.set(e.value().clone()),
                        }
                    }

                    div {
                        label {
                            class: "block text-sm font-medium text-gray-700 mb-2",
                            "Password"
                        }
                        input {
                            class: "w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition",
                            r#type: "password",
                            placeholder: "Enter your password",
                            value: "{password()}",
                            oninput: move |e| password.set(e.value().clone()),
                        }
                    }

                    button {
                        class: "w-full bg-indigo-600 hover:bg-indigo-700 text-white font-semibold py-3 px-4 rounded-lg transition duration-200 transform hover:scale-105",
                        disabled: loading(),
                        onclick: handle_login,
                        if loading() {
                            "Logging in..."
                        } else {
                            "Login"
                        }
                    }

                    div {
                        class: "text-center mt-4",
                        span { class: "text-gray-600", "Don't have an account? " }
                        Link {
                            to: Route::Signup {},
                            class: "text-indigo-600 hover:text-indigo-800 font-semibold",
                            "Sign up"
                        }
                    }
                }
            }
        }
    }
}