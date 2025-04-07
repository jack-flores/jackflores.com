use yew::prelude::*;

#[function_component]
fn Frontend() -> Html {
    let counter = use_state(|| 0);

    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div class="mt-[35px] text-center text-white font-cubano">
            <button class="w-1/2 h-[400px] text-[18em] font-cubano text-white text-center bg-[#555555] border-[8px]" {onclick}>{ "+1" }</button>
            <p class="text-[15em]">{ *counter }</p>
        </div>

    }
}

fn main() {
    yew::Renderer::<Frontend>::new().render();
}
