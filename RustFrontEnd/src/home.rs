use yew::prelude::*;
use yew_router::prelude::*;
use yew::function_component;
use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let on_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Chat); 
        })
    };

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
    
            <div style="background-image: url('/background.png'); background-size: cover; min-height: 100vh; display: flex; flex-direction: row; align-items: center;margin-top: 60px;">
            
            <div style="flex: 1; padding: 20px; display: flex; justify-content: center; align-items: center;">
            <div style="text-align: left; width: 75%;">
                <h2 style="font-size: 48px; line-height: 1.2;">{"Health Tips"}</h2>
                <ul style="font-size: 24px; line-height: 1.5;">
                    <li>{"Maintain a balanced diet."}</li>
                    <li>{"Exercise regularly to stay fit."}</li>
                    <li>{"Take breaks to manage stress effectively."}</li>
                    <li>{"Stay hydrated and get enough sleep."}</li>
                    <li>{"Seek professional help if needed."}</li>
                </ul>
            </div>
            </div>
            <div style="flex: 1; padding: 20px; display: flex; justify-content: center; align-items: center;">
            <div style="text-align: center;">
                <h2 style="font-size: 36px; line-height: 1.5;">{"Do you want to talk with a professional?"}</h2>
                <button style="padding: 15px 30px; font-size: 20px; border: none; background-color: #007BFF; color: white; border-radius: 5px;" onclick={on_click}>{"Let's Talk"}</button>
            </div>
        </div>
            </div>
    
        </>
    }
}
