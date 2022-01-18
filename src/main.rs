use std::collections::HashMap;

use yew::prelude::*;
use rand::distributions::{Uniform, Distribution};

struct AbilityScore([u8; 6]);

fn calculate_cost(abilities: &AbilityScore) -> i8 {

    let score_costs: HashMap<u8, i8> = HashMap::from([
               (3, -13),
               (4, -10),
               (5, -7),
               (6, -4),
               (7, -2),
               (8, 0),
               (9, 1),
               (10, 2),
               (11, 3),
               (12, 4),
               (13, 5),
               (14, 7),
               (15, 9),
               (16, 12),
               (17, 15),
               (18, 19),
    ]);

    abilities.0.iter().map(|v| score_costs[v]).sum()
}

fn four_d_six_drop_lowest() -> u8 {
    let mut rng = rand::thread_rng();
    let d6: Uniform<u8> = Uniform::new_inclusive(1, 6);
    let mut dies = [0u8; 4];
    for die in &mut dies {
        *die = d6.sample(&mut rng);
    }
    dies.iter().sum::<u8>() - *dies.iter().min().unwrap()
}

fn generate_valid_score() -> AbilityScore {
    let mut output = AbilityScore([four_d_six_drop_lowest(); 6]);
    loop {
        for score in &mut output.0 {
            *score = four_d_six_drop_lowest()
        }

        if calculate_cost(&output) == 32 { break }
    }
    output.0.sort();
    output
}


struct Model {
    first: AbilityScore,
    second: AbilityScore,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            first:  generate_valid_score(),
            second: generate_valid_score(),
        }
    }

    

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        html! {
            <div>
                <p> {"Choose one of the following set of scores:"} </p>

                <h6>{ "First Option"} </h6>

                <ul>
                    <li> { self.first.0[0] } </li>
                    <li> { self.first.0[1] }</li>
                    <li>{ self.first.0[2] } </li>
                    <li>{ self.first.0[3] } </li>
                    <li>{ self.first.0[4] } </li>
                    <li>{ self.first.0[5] } </li>
                </ul>

                <h6> {"Second Option"} </h6>
                <ul>
                    <li> { self.second.0[0] } </li>
                    <li> { self.second.0[1] }</li>
                    <li>{ self.second.0[2] } </li>
                    <li>{ self.second.0[3] } </li>
                    <li>{ self.second.0[4] } </li>
                    <li>{ self.second.0[5] } </li>
                </ul>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
