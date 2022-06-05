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
            <div>
                <h1>{"Monadium.org"}</h1>

                <p>{"A lab for building Monadium.org with Yew."}</p>

                <crate::domains::profiles::Profile />
            </div>
        }
    }
}
