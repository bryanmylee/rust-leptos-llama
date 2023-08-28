use leptos::*;

#[component]
pub fn InputField(
    cx: Scope,
    #[prop(optional)] on_send: Option<Box<dyn Fn(String)>>,
) -> impl IntoView {
    let input_ref = create_node_ref::<html::Input>(cx);

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let Some(input) = input_ref() else {
            return;
        };
        if let Some(on_send) = &on_send {
            on_send(input.value());
        }
        input.set_value("");
    };

    view! { cx,
        <form on:submit=handle_submit class="sticky bottom-0 p-4 gap-4 flex justify-center items-center">
            <input
                type="text"
                node_ref=input_ref
                class="w-full rounded-lg px-4 py-3 focus-visible:border-transparent focus-visible:ring ring-blue-500 focus:outline-none shadow-lg border border-gray-200"
                placeholder="Chat with us!"
            />
            <button
                type="submit"
                class="bg-blue-500 text-white rounded-full flex-shrink-0 w-12 h-12 flex justify-center items-center focus-visible:ring ring-blue-500 ring-offset-2 focus:outline-none shadow-md shadow-blue-500/50"
            >
                "→"
            </button>
        </form>
    }
}
