use yew::prelude::*;

pub struct CreateRepo;

impl Component for CreateRepo {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! { <div>
            <div>
                <label for={"repo_name"}>{"Repository name*"}</label>
                <input id={"repo_name"} name={"repo_name"} type={"text"}/>
            </div>
            <div>
                <label for={"description"}>{"Description"}</label>
                <textarea id={"description"} name={"description"}></textarea>
            </div>
            <div>
                <label for={"visibility"}>{"Visibility*"}</label>
                <select name={"visibility"} id={"visibiility"} required=true>
                    <option value={""} >{"Select"}</option>
                    <option value={"public"} >{"public"}</option>
                    <option value={"private"} >{"private"}</option>
                </select>
            </div>
            <button>{"create"}</button>
        </div> }
    }
}
