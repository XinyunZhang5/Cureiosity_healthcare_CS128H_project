use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use crate::UserInfo;
use wasm_bindgen::JsCast;

#[function_component(Login)]
pub fn login() -> Html {
    let user_info = use_context::<UseStateHandle<UserInfo>>().expect("No UserInfo provided");
    let navigator = use_navigator().unwrap();

    let on_submit = {
        let user_info = user_info.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default(); 

            let form = event
                .target_unchecked_into::<web_sys::HtmlFormElement>();
            
            let name_input = form
                .query_selector("[name='name']")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap()
                .value();

            let email_input = form
                .query_selector("[name='email']")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap()
                .value();

            user_info.set(UserInfo { name: name_input, email: email_input });

            navigator.push(&Route::Home);
        })
    };

    html! {
        <div style="background-image: url('/background.png'); background-size: cover; min-height: 100vh; display: flex; flex-direction: column; padding: 20px;">
            <h1 style = "margin-top: 60px;">{"Login"}</h1>
            <form onsubmit={on_submit}>
                <label>{"Name: "}</label>
                <input type="text" name="name" required=true />
                <br />
                <label>{"Email: "}</label>
                <input type="email" name="email" required=true />
                <br />
                <button type="submit">{"Get Started"}</button>
            </form>
        </div>
    }
}
