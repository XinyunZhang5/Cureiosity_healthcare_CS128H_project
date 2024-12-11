use yew::prelude::*;
use yew_router::prelude::*;
mod login;
mod home;
mod profile;
mod chat;

use login::Login;
use home::Home;
use profile::Profile;
use chat::Chat;

#[derive(Clone, PartialEq)]
pub struct UserInfo {
    pub name: String,
    pub email: String,
}

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/home")]
    Home,
    #[at("/profile")]
    Profile,
    #[at("/chat")]
    Chat,
}

#[function_component(App)]
fn app() -> Html {
    let user_info = use_state(|| UserInfo {
        name: "".to_string(),
        email: "".to_string(),
    });

    html! {
        <ContextProvider<UseStateHandle<UserInfo>> context={user_info}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<UseStateHandle<UserInfo>>>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Login => html! { <Login /> },
        Route::Home => html! { <Home /> },
        Route::Profile => html! { <Profile /> },
        Route::Chat => html! { <Chat /> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
