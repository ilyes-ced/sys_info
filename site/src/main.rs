use yew::prelude::*;






#[derive(Properties, PartialEq)]
pub struct AlertProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    html! {
        <div class={classes!("alert",props.class.clone())}>
        {for props.children.iter()}
        </div>
    }
}




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

	let AlertProps = AlertProps {
		children: ,
		class: "Smith".to_owned(),
	};
	html! {
		<div>
			<alert ..AlertProps.clone() />
			<br/>
			<br/>
			<br/>
			<br/>
			<br/>
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