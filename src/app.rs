use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::api::converse;
use crate::chat_area::ChatArea;
use crate::conversation::Conversation;
use crate::input_field::InputField;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Title text="Welcome to Leptos"/>
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let (conversation, set_conversation) = create_signal(cx, Conversation::new());

    // Creates an `Action` to synchronize the async imperative function to the synchronous reactive
    // system.
    //
    // `Action` wraps three signals to do so:
    // 1. an input signal that is fired every time the user triggers the function
    // 2. a value signal that is fired when the async function resolves
    // 3. a pending signal indicating the loading status of the function
    let send = create_action(cx, move |new_message: &String| {
        set_conversation.update(move |convo| {
            convo.add_user_message(new_message);
        });
        
        converse(cx, conversation())
    });

    create_effect(cx, move |_| {
        if let Some(_) = send.input()() {
            set_conversation.update(move |convo| convo.add_assistant_waiting());
        }
    });

    create_effect(cx, move |_| {
        if let Some(Ok(response)) = send.value()() {
            set_conversation.update(move |convo| {
                convo.resolve_assistant_waiting(&response);
            });
        }
    });

    view! { cx,
        <div class="flex flex-col h-screen">
            <ChatArea conversation/>
            <InputField on_send=Box::new(move |text| {
                if !text.is_empty() {
                    send.dispatch(text)
                }
            })/>
        </div>
    }
}

#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during initial server-side rendering if
    // you navigate to the 404 page subsequently, the status code will not be set because there is
    // not a new HTTP request to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { cx,
        <div class="min-h-screen flex justify-center items-center">
            <h1 class="font-semibold text-2xl">
                Page not found
            </h1>
        </div>
    }
}
