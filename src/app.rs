use yew::{html, Component};

use crate::domains::profiles;

pub struct App {}

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &yew::Context<Self>) -> yew::Html {
        html! {
            <>
            <div class="content">
            <header>
              <h1><a href="/" class="logo-link">{"🧡"}</a> { " Monadium.org" }</h1>
            </header>

            <main>
                <profiles::Profile/>
            </main>
            </div>

        <nav class="navigation">
        <h2>{"Navigation"}</h2>

        <ul>
          <li><p><a href="/">{"🧡"}</a></p></li>
          <li><p><a href="/our-mission">{"/our-mission"}</a></p></li>
          <li><p><a href="/profiles">{"/profiles"}</a></p></li>
          <li><p><a href="/teams">{"/teams"}</a></p></li>
        </ul>
        </nav>
        </>
        }
    }
}
