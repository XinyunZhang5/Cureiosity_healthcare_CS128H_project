use yew::prelude::*;
use yew_router::prelude::*;
use crate::{Route, UserInfo};

#[function_component(Profile)]
pub fn profile() -> Html {
    let user_info = use_context::<UseStateHandle<UserInfo>>().expect("No UserInfo provided");
    let navigator = use_navigator().unwrap();

    let navigate_to = |route: Route| {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&route);
        })
    };

    html! {
        <>
        <div style="position: fixed; top: 0; width: 100%; background-color: #f8f9fa; padding: 10px; z-index: 1000; display: flex;">
        <button style="flex: 1; padding: 15px; border: none; font-size: 18px; background-color: transparent;" onclick={navigate_to(Route::Home)}>{"Home"}</button>
        <button style="flex: 1; padding: 15px; border: none; font-size: 18px; background-color: transparent;" onclick={navigate_to(Route::Profile)}>{"Profile"}</button>
        <button style="flex: 1; padding: 15px; border: none; font-size: 18px; background-color: transparent;" onclick={navigate_to(Route::Chat)}>{"Chat"}</button>
    </div>
    
    <div style="background-image: url('/background.png'); background-size: cover; min-height: 100vh; display: flex; flex-direction: row; align-items: flex-start; margin-top: 60px; padding-top: 60px;">
        <div style="padding: 20px; margin-left: 20px; text-align: left;">
            <h1 style="font-size: 48px; margin-bottom: 20px;">{"User Profile"}</h1>
            <p style="font-size: 24px; margin-bottom: 10px;">{format!("Name: {}", user_info.name)}</p>
            <p style="font-size: 24px; margin-bottom: 10px;">{format!("Email: {}", user_info.email)}</p>
        </div>
    </div>
    
        </>
    }
}
