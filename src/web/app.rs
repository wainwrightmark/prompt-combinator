use std::rc::Rc;

use crate::core::prelude::*;
use crate::state::{self, prelude::*};
use crate::web::prelude::*;
use itertools::Itertools;
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {

        <div class="paper container margin-bottom-large" style="display: flex; flex-direction: column;">
            <NameBox />
            <InputBox />
            <ErrorBox />
            <DisplayBox/>
        </div>
    }
}

#[function_component(ExamplesSelect)]
pub fn examples_select() -> Html {
    let oninput = Dispatch::<InputState>::new().reduce_mut_callback_with(|s, e: InputEvent| {
        let input: HtmlSelectElement = e.target_unchecked_into();
        let value = input.value();
        s.load_saved(value);
    });

    let saved_prompts = use_store_value::<SavedPromptsState>();
    let chosen = use_selector(|state: &InputState| state.name.clone())
        .as_ref()
        .clone();

    let options = saved_prompts.prompts.keys().map(|key| {
        let selected = &chosen == key;
        html!(<option {selected} value={key.clone()}>{key.clone()} </option>)
    });

    html!(
    <select {oninput}>
    {for options}
    </select>

        )
}

#[function_component(NameBox)]
pub fn name_box() -> Html {
    let name = use_selector(|state: &InputState| state.name.clone())
        .as_ref()
        .clone();
    let oninput = Dispatch::<InputState>::new().reduce_mut_callback_with(|s, e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let value = input.value();
        s.name = value;
    });

    let onclick = Dispatch::<InputState>::new().reduce_mut_callback(|s| s.save());

    html! {
        <div style="display: flex;">
        <input {oninput}   value={name}   style="width: 100px;"         />
        <button {onclick}> {"Save"} </button>
        <ExamplesSelect/>
        </div>
    }
}

#[function_component(InputBox)]
pub fn input_box() -> Html {
    let text = use_selector(|state: &InputState| state.text.clone())
        .as_ref()
        .clone();
    let oninput = Dispatch::<InputState>::new().reduce_mut_callback_with(|s, e: InputEvent| {
        let input: HtmlTextAreaElement = e.target_unchecked_into();
        let value = input.value();
        s.update_text(value);
    });

    html!(
            <div>
    <p>

    </p>
    //https://css-tricks.com/creating-an-editable-textarea-that-supports-syntax-highlighted-code/
            <textarea id="input-textarea" name="input-textarea" class="input-textarea" rows="2" {oninput}
            value={text}
            spellcheck="false"
            >
            </textarea>
            </div>
        )
}

#[function_component(ErrorBox)]
pub fn error_box() -> Html {
    let err = use_selector(|s: &InputState| s.error.clone())
        .as_ref()
        .clone()
        .unwrap_or_else(|| "‎".to_string());
    html!(<code> {err} </code>)
}

#[function_component(DisplayBox)]
pub fn display_box() -> Html {
    let output_vec = use_selector(|state: &InputState| state.output.clone())
        .as_ref()
        .clone();

    let text = output_vec.join("\r\n");

    html!(
        <div>
        <code id="output" name="output" style="display: block; white-space: pre-wrap">
        {text}
        </code>
        </div>
    )
}
