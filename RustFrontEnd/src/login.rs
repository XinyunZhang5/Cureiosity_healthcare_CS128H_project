use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use crate::UserInfo;
use wasm_bindgen::JsCast; // 修复错误，导入 JsCast

#[function_component(Login)]
pub fn login() -> Html {
    let user_info = use_context::<UseStateHandle<UserInfo>>().expect("No UserInfo provided");
    let navigator = use_navigator().unwrap();

    let on_submit = {
        let user_info = user_info.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default(); // 防止表单提交刷新页面

            // 获取目标表单
            let form = event
                .target_unchecked_into::<web_sys::HtmlFormElement>(); // 强制转换为表单元素
            
            // 通过 query_selector 获取 Name 和 Email 的值
            let name_input = form
                .query_selector("[name='name']")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::HtmlInputElement>() // 使用 JsCast 提供的 dyn_into
                .unwrap()
                .value();

            let email_input = form
                .query_selector("[name='email']")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::HtmlInputElement>() // 使用 JsCast 提供的 dyn_into
                .unwrap()
                .value();

            // 更新全局状态
            user_info.set(UserInfo { name: name_input, email: email_input });

            // 跳转到 Home 页面
            navigator.push(&Route::Home);
        })
    };

    html! {
        <div>
            <h1>{"Login"}</h1>
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
