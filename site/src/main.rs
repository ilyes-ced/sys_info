use yew::prelude::*;



#[function_component]
fn App() -> Html {
	let counter = use_state(|| 0);
	let onclick = {
		let counter = counter.clone();
		move || {
			let value = *counter + 1;
			counter.set(value);
		}
	};

	html! {
		<div>
			<div class={classes!("overflow-x-auto")}>
			  <table class={classes!("table")}>
				<thead>
				  <tr>
					<th></th>
					<th>{"Name"}</th>
					<th>{"Job"}</th>
					<th>{"Favorite Color"}</th>
				  </tr>
				</thead>
				<tbody>

				  <tr>
					<th>{1}</th>
					<td>{"Cy Ganderton"}</td>
					<td>{"Quality Control Specialist"}</td>
					<td>{"Blue"}</td>
				  </tr>

				  <tr>
					<th>{2}</th>
					<td>{"Hart Hagerty"}</td>
					<td>{"Desktop Support Technician"}</td>
					<td>{"Purple"}</td>
				  </tr>

				  <tr>
					<th>{3}</th>
					<td>{"Brice Swyre"}</td>
					<td>{"Tax Accountant"}</td>
					<td>{"Red"}</td>
				  </tr>
				</tbody>
			  </table>
			</div>
		</div>
	}
}

fn main() {
	yew::Renderer::<App>::new().render();
}