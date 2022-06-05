use yew::{html, Component};

use crate::data;

use super::Profile;

impl Component for Profile {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        data::marcus_radell::profile()
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div>{format!("{self:?}")}</div>
        }
    }
}
