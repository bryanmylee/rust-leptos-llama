<p align="center">
    <img width="667" alt="leptos" src="https://github.com/bryanmylee/rust-leptos-llama/assets/42545742/b1f9c074-7e94-42c0-b77c-6282965acd38">
</p>

# Leptos with Llama

An experiment on the feasibility of building a full-stack application purely in Rust.

The front-end is built with [Leptos](https://github.com/leptos-rs/leptos), and the server-side capabilities are powered by Actix Web.

The backend uses the [Rustformers LLM](https://github.com/rustformers/llm) crate to run a Llama large language model (LLM).

## Running your project

Download and install a Llama model. For this project, we use the [Wizard-Vicuna-7B-Uncensored-GGML](https://huggingface.co/TheBloke/Wizard-Vicuna-7B-Uncensored-GGML) model.

Create `.env` and set the path of the model as `MODEL_PATH`.

Run `cargo leptos watch`.

Then access the project at `http://localhost:8000`
