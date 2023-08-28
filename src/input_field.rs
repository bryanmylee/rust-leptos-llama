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
        <form on:submit=handle_submit class="p-4 gap-2 flex justify-center items-center bg-white">
            <input
                type="text"
                node_ref=input_ref
                class="w-full border-2 border-gray-200 rounded-lg p-2 focus-visible:border-transparent focus-visible:ring ring-blue-500 focus:outline-none"
            />
            <button
                type="submit"
                class="bg-blue-500 text-white rounded-full flex-shrink-0 w-10 h-10 flex justify-center items-center focus-visible:ring ring-blue-500 ring-offset-2 focus:outline-none"
            >
                "→"
            </button>
        </form>
    }
}
