#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use common::Message;

use freya::prelude::*;
use reqwest::Client;

fn main() {
    launch_with_title(app, "shuttle + freya");
}

fn app(cx: Scope) -> Element {
    use_init_focus(cx);
    let messages = use_state(cx, Vec::new);
    let writing = use_state(cx, String::new);

    use_effect(cx, (), move |_| {
        to_owned![messages];
        async move {
            let res = get_messages().await;
            if let Ok(loaded_messages) = res {
                messages.set(loaded_messages);
            }
        }
    });

    let load = move |_| {
        to_owned![messages];
        cx.spawn(async move {
            let res = get_messages().await;
            if let Ok(loaded_messages) = res {
                messages.set(loaded_messages);
            }
        })
    };

    let onchange = {
        to_owned![writing];
        move |v: String| {
            writing.set(v.clone());
        }
    };

    let send = move |_| {
        to_owned![writing, messages];
        cx.spawn(async move {
            send_message(Message { text: writing.get().clone() , author: "me".to_string() }).await;
            writing.set("".to_string());
            let res = get_messages().await;
            if let Ok(loaded_messages) = res {
                messages.set(loaded_messages);
            }
        })
    };
    

    render!(
        rect {
            width: "100%",
            height: "100%",
            padding: "15",
            Button {
                onclick: load,
                label {
                    "Load messages"
                }
            }
            ScrollView {
                show_scrollbar: true,
                width: "100%",
                height: "calc(100% - 100)",
                
                for message in messages.get() {
                    rsx!(
                        MessageBox {
                            message: message.clone()
                        }
                    )
                }
            }
            rect {
                height: "50",
                direction: "horizontal",
                Input {
                    value: writing.get().clone(),
                    onchange: onchange
                }
                Button {
                    onclick: send,
                    label {
                        "Send"
                    }
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
fn MessageBox(cx: Scope, message: Message) -> Element {
    render!(
        rect {
            height: "45",
            width: "100%",
            padding: "3",
            rect {
                background: "rgb(220, 220, 220)",
                padding: "10",
                radius: "10",
                label {
                    "{message.author}: {message.text}"
                }
            }
        }
    )
}

async fn send_message(message: Message){
   let client = Client::new();
   client.post(format!("http://127.0.0.1:8000/send?text={}&author={}", message.text, message.author)).send().await.ok();

}
async fn get_messages() -> reqwest::Result<Vec<Message>> {
    let response = reqwest::get("http://127.0.0.1:8000/messages").await;

    response?.json().await
}