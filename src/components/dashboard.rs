use yew::prelude::*;

pub struct Dashboard {}

pub enum Msg {}

impl Component for Dashboard {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! { <>
            <div>
                <p>{"welcome"}</p>
            </div>
        </> }
    }
}
