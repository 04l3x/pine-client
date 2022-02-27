use crate::app::AppRoute;
use material_yew::button::MatButton;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(LoginButton)]
pub fn login_button() -> Html {
    html! { <>
        <Link<AppRoute> to={AppRoute::Login}>
            <MatButton label="login" raised=true/>
        </Link<AppRoute>>
    </> }
}
