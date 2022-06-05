use yew::{html, Component};

pub struct Comp {}

impl Component for Comp {
    type Message = ();

    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &yew::Context<Self>) -> yew::Html {
        html! {
            <div class="content">
                <crate::domains::profiles::Profile />
            </div>
        }
    }
}
