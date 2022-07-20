use yew::prelude::*;

fn main() {
	yew::start_app::<App>();
}

#[function_component(Bar)]
pub fn bar() -> Html {
	html! {
		<div class={"fixed flex top-0 h-[80px] w-screen bg-blue-100 justify-evenly items-center"}>
		{"Bar!"}
		</div>
	}
}

#[function_component(App)]
pub fn app() -> Html {
	html! {
		<div class={"w-screen h-screen"}>
		<div class={"flex bg-red-400 h-screen max-h-screen"}>
			<Colour/>
			<Controls/>
		</div>
		</div>
	}
}

#[function_component(Colour)]
pub fn colour() -> Html {
	html! {
		<div class={"bg-green-100 aspect-square"}>
			<div class={"rounded-full aspect-square bg-[#FF3000] m-40"}></div>
		</div>
	}
}

#[function_component(Controls)]
pub fn controls() -> Html {
	html! {
		<div class={"flex grow justify-center items-center bg-yellow-100"}>
		{"hello :)"}
		</div>
	}
}
