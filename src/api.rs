use crate::conversation::Conversation;
use cfg_if::cfg_if;
use itertools::Itertools;
use leptos::*;

// Creates a server-side API with logic that only runs on server.
//
// This also automatically creates a function with the same name that can be invoked on the client
// side to automatically make an HTTP request to this function.
//
// The created function will return a type that matches this function's signature.
#[server(Converse "/api")]
pub async fn converse(cx: Scope, conversation: Conversation) -> Result<String, ServerFnError> {
    use actix_web::dev::ConnectionInfo;
    use actix_web::web::Data;
    /// extractors allow us to access pieces of the HTTP request and also share common data across all
    /// server-side logic.
    use leptos_actix::extract;
    use llm::models::Llama;
    use rand;

    // The actix web framework is the tool that invokes our server side logic, so we cannot
    // explicitly define what we want to pass into our function.
    //
    // To get around this, we define app data that we then access in our server handler using
    // `extract`. In this project, our data will be our Llama model.
    let model = extract(cx, |data: Data<Llama>, _connection: ConnectionInfo| async {
        data.into_inner()
    })
    .await
    .unwrap();

    use llm::KnownModel;

    // Every time we feed the model with text, we format as such:
    // - "### Assistant:" for assistant generated text
    // - "### Human:" for user generated text
    // - end every prompt with an open ended "### Assistant:"

    let assistant_name = "### Assistant";
    let user_name = "### Human";
    let persona = "A chat between a human and an assistant";
    let initial_prompt = format!(
        "\
{assistant_name}: Hello - How may I help you today?
"
    );
    let next_prompts = conversation
        .messages
        .into_iter()
        .map(|message| {
            let text = message.text;
            if message.from_user {
                format!("{assistant_name}: {text}")
            } else {
                format!("{user_name}: {text}")
            }
        })
        .join("\n");

    let history = initial_prompt + &next_prompts;

    let mut result = String::new();
    let mut rng = rand::thread_rng();
    let mut buffer = String::new();

    let mut session = model.start_session(Default::default());

    session
        .infer(
            model.as_ref(),
            &mut rng,
            &llm::InferenceRequest {
                prompt: format!(
                    "\
{persona}
{history}
{assistant_name}:"
                )
                .as_str()
                .into(),
                parameters: &llm::InferenceParameters::default(),
                play_back_previous_tokens: false,
                maximum_token_count: None,
            },
            &mut Default::default(),
            inference_callback(String::from(user_name), &mut buffer, &mut result),
        )
        .unwrap_or_else(|err| panic!("{err}"));

    Ok(result)
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    fn inference_callback<'a>(
        stop_sequence: String,
        buf: &'a mut String,
        out_str: &'a mut String,
    ) -> impl FnMut(llm::InferenceResponse) -> Result<llm::InferenceFeedback, !> + 'a {
        use llm::InferenceFeedback::{Halt, Continue};
        move |resp| match resp {
            llm::InferenceResponse::InferredToken(token) => {
                let mut reverse_buf = buf.clone();
                reverse_buf.push_str(token.as_str());
                if stop_sequence.as_str().eq(reverse_buf.as_str()) {
                    buf.clear();
                    return Ok::<llm::InferenceFeedback, !>(Halt);
                }
                if stop_sequence.as_str().starts_with(reverse_buf.as_str()) {
                    buf.push_str(token.as_str());
                    return Ok(Continue);
                }

                if buf.is_empty() {
                    out_str.push_str(&token);
                } else {
                    out_str.push_str(&reverse_buf);
                }

                Ok(Continue)
            },
            llm::InferenceResponse::EotToken => Ok(Halt),
            _ => Ok(Continue),
        }
    }
}
}
