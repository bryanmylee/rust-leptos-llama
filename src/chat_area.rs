use leptos::*;

use crate::conversation::Conversation;

#[component]
pub fn ChatArea(cx: Scope, conversation: ReadSignal<Conversation>) -> impl IntoView {
    let chat_ref = create_node_ref::<html::Ul>(cx);

    create_effect(cx, move |_| {
        conversation();
        if let Some(chat_ref) = chat_ref() {
            chat_ref.set_scroll_top(chat_ref.scroll_height());
        }
    });

    view! { cx,
        <ul class="flex-1 flex flex-col gap-4 p-4" node_ref=chat_ref>
            {move || {
                conversation()
                    .messages
                    .iter()
                    .map(|message| {
                        view! { cx,
                            <li class=if message.from_user {
                                "bg-blue-500 text-white self-end max-w-md p-4 rounded-2xl shadow-md shadow-blue-500/50"
                            } else {
                                "bg-gray-200 text-black self-start max-w-md p-4 rounded-2xl shadow-md"
                            }>{message.text.clone()}</li>
                        }
                    })
                    .collect_view(cx)
            }}

        </ul>
    }
}
