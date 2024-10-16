use yew::prelude::*;

use crate::components::Counter;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <h1>{ "Hello World!"} </h1>
            <subtitle style="display:block; margin-top:-40px;"><i> {" (Counter edition)"} </i></subtitle>

            <section id="counter">
                <Counter />
            </section>

            <span class="subtitle">{ "made using Yew with " }<i class="heart" /></span>
        </main>
    }
}
