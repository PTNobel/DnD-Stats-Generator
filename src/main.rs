use rand::distributions::{Distribution, Uniform};
use yew::prelude::*;

const PREFERRED_COST: i8 = 32;

const fn individual_score_cost(x: &u8) -> i8 {
    match x {
        3 => -13,
        4 => -10,
        5 => -7,
        6 => -4,
        7 => -2,
        8 => 0,
        9 => 1,
        10 => 2,
        11 => 3,
        12 => 4,
        13 => 5,
        14 => 7,
        15 => 9,
        16 => 12,
        17 => 15,
        18 => 19,
        _ => unreachable!(),
    }
}

struct AbilityScore([u8; 6]);

impl AbilityScore {
    fn get_cost(&self) -> i8 {
        self.0.iter().map(individual_score_cost).sum()
    }

    fn new() -> Self {
        let mut output = AbilityScore([four_d_six_drop_lowest(); 6]);

        for score in &mut output.0 {
            *score = four_d_six_drop_lowest()
        }

        output
    }

    fn new_with_cost(cost: i8) -> Self {
        // Minimum cost ability scores occurs with probability
        // (6**4)**-6 == 6**-24 ~= 4.45e-19
        // The maximum cost ability scores occurs with probability
        // $\left(\left(
        //      \binom{4}{3} \left(\frac{1}{6^3}\right) \left(\frac{5}{6}\right)
        //      + \frac{1}{6^4}\right)
        //  \right)^6 \right) =\frac{117649}{6499837226778624} \approxeq
        //  1.81 \times 10^-11$
        //  The number of iterations before this algorithm halts is geometricly
        //  distributed.
        //  If we make the assumption that a loop takes a microsecond it will
        //  take over 2 hours, 46 minutes, and 40 seconds in expectation for
        //  this to halt if cost == individual_score_cost(&18) * 6.
        //  It will obviously take longer if the minimum cost is requested.
        //
        // These asserts are therefore not a guarantee this algorithm halts in a
        // reasonable amount of time. They're just hear to guarantee that we
        // halt in finite time, almost surely.
        // This guarantee is, of course, completely useless.
        assert!(cost >= individual_score_cost(&3) * 6);
        assert!(cost <= individual_score_cost(&18) * 6);
        loop {
            let mut output = AbilityScore::new();
            if output.get_cost() == cost {
                output.0.sort_unstable();
                break output;
            }
        }
    }
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

struct Model {
    first: AbilityScore,
    second: AbilityScore,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            first: AbilityScore::new_with_cost(PREFERRED_COST),
            second: AbilityScore::new_with_cost(PREFERRED_COST),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p> {"Choose one of the following set of scores:"} </p>

                <h4>{ "Set of Scores A"} </h4>

                <ul>
                    <li> { self.first.0[0] } </li>
                    <li> { self.first.0[1] }</li>
                    <li>{ self.first.0[2] } </li>
                    <li>{ self.first.0[3] } </li>
                    <li>{ self.first.0[4] } </li>
                    <li>{ self.first.0[5] } </li>
                </ul>

                <h4> {"Set of Scores B"} </h4>
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
