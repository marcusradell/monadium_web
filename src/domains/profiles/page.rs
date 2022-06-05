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
                    <div>
                    <section>
                        <h2>{"Dream"}</h2>
                        <strong>{ &self.dream }</strong>
                    </section>

                    <section>
            <h2>{"Experience"}
            {if self.since() > 0 {html!{<small><em>{format!("({} years)", self.since())}</em></small>}} else {html!{}} }
            </h2>

            <p><em>{"Based on tags in gigs."}</em></p>

            // <section>
            //     {%for exp in experiences -%}
            //     <h3>{{exp.tag}} {% if favorites.includes(exp.tag) %} 🧡 {% endif %}</h3>
            //     <p>{% if exp.totalYears === 0 %} <1 year {% else %} {{exp.totalYears}} years {%endif%}</p>
            //     {% endfor -%}
            // </section>
        </section>

                    </div>
                }
    }
}
