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
    
    let messages = move || conversation().messages;

    view! { cx,
        <ul class="flex-1 flex flex-col gap-4 p-4" node_ref=chat_ref>
            <For
                each=messages
                key=|m| m.id
                view=move |cx, message| {
                    let message = create_memo(
                        cx,
                        move |_| {
                            conversation
                                .with(move |convo| {
                                    convo
                                        .messages
                                        .iter()
                                        .find(|msg| msg.id == message.id)
                                        .expect("message with id should exist")
                                        .clone()
                                })
                        },
                    );
                    let bubble_class = move || {
                        if message().from_user {
                            "bg-blue-500 text-white self-end max-w-md p-4 rounded-2xl shadow-md shadow-blue-500/50"
                        } else {
                            "bg-gray-200 text-black self-start max-w-md p-4 rounded-2xl shadow-md"
                        }
                    };
                    let message_content = move || message().text.clone();
                    view! { cx, <li class=bubble_class>{message_content}</li> }
                }
            />
        </ul>
    }
}
