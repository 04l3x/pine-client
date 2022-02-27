use yew::prelude::*;

pub struct RepositoryViewer;
impl Component for RepositoryViewer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! { <>
            {"A cool visor for each repo"}
        </> }
    }
}
