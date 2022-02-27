use crate::app::AppRoute;
use material_yew::button::MatButton;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(RegisterButton)]
pub fn register_button() -> Html {
    html! { <>
        <Link<AppRoute> to={AppRoute::Register}>
            <MatButton label="register" raised=true/>
        </Link<AppRoute>>
    </> }
}
