use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use yew::function_component;
use yew::use_state;

#[function_component(Chat)]
pub fn chat() -> Html {
    let navigator = use_navigator().unwrap();
    let messages = use_state(|| Vec::new()); // 聊天消息的状态
    let input_value = use_state(|| String::new());

    let navigate_to = |route: Route| {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&route);
        })
    };

    let on_send = {
        let messages = messages.clone();
        let input_value = input_value.clone();
        Callback::from(move |_| {
            let mut msgs = (*messages).clone();
            let new_message = (*input_value).clone();
            if !new_message.is_empty() {
                msgs.push(new_message);
                messages.set(msgs);
                input_value.set(String::new());
            }
        })
    };

    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            input_value.set(input.value());
        })
    };

    html! {
        <>
            <div style="position: fixed; top: 0; width: 100%; background-color: #f8f9fa; padding: 10px; z-index: 1000; display: flex;">
                <button style="flex: 1; padding: 15px; border: none; font-size: 18px; background-color: transparent;" onclick={navigate_to(Route::Home)}>{"Home"}</button>
                <button style="flex: 1; padding: 15px; border: none; font-size: 18px; background-color: transparent;" onclick={navigate_to(Route::Profile)}>{"Profile"}</button>
                <button style="flex: 1; padding: 15px; border: none; font-size: 18px; background-color: transparent;" onclick={navigate_to(Route::Chat)}>{"Chat"}</button>
            </div>

            <div style="background-image: url('/background.png'); background-size: cover; min-height: 100vh; display: flex; flex-direction: column; margin-top: 60px; padding: 20px;">

                <h1 style="text-align: center; font-size: 48px; margin-bottom: 20px;">{"Chat Platform"}</h1>

                <div style="flex: 1; overflow-y: auto; background: rgba(255, 255, 255, 0.5); border-radius: 10px; padding: 10px;">
                    {
                        for (*messages).iter().map(|msg| {
                            html! { <p style="margin: 5px 0;">{msg}</p> }
                        })
                    }
                </div>

                <div style="display: flex; margin-top: 10px;">
                    <input
                        type="text"
                        placeholder="Type your message..."
                        value={(*input_value).clone()}
                        oninput={on_input}
                        style="flex: 1; padding: 10px; border: 1px solid #ccc; border-radius: 5px; margin-right: 10px;" />
                    <button
                        onclick={on_send}
                        style="padding: 10px 20px; background-color: #007BFF; color: white; border: none; border-radius: 5px; cursor: pointer;">
                        {"Send"}
                    </button>
                </div>
            </div>
        </>
    }
}
