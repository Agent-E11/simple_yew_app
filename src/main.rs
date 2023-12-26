use yew::{html, Html, Component, Context};

pub enum Msg {
    Calculate,
    SetNumber(bool, f32),
    SetOperator(Operator),
    ToggleNumber,
}
pub enum Operator {
    Mul,
    Div,
    Add,
    Sub,
}
impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mul => write!(f, " * "),
            Self::Div => write!(f, " / "),
            Self::Add => write!(f, " + "),
            Self::Sub => write!(f, " - "),
        }
    }
}

pub struct Calculator {
    result: f32,
    number_1: f32,
    number_2: f32,
    operator: Operator,
    set_number_1: bool,
    history: Vec<f32>,
}

impl Component for Calculator {
    type Message = Msg;
    type Properties = ();
    
    fn create(ctx: &Context<Self>) -> Self {
        Self { result: 0., number_1: 0., number_2: 0., operator: Operator::Add, set_number_1: true, history: vec![] }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Calculate => {
                let res = match self.operator {
                    Operator::Mul => self.number_1 * self.number_2,
                    Operator::Div => self.number_1 / self.number_2,
                    Operator::Add => self.number_1 + self.number_2,
                    Operator::Sub => self.number_1 - self.number_2,
                };
                self.result = res;
                self.number_1 = res;
            },
            Msg::SetNumber(set_1, n) => if set_1 { self.number_1 = n } else { self.number_2 = n},
            Msg::SetOperator(o) => self.operator = o,
            Msg::ToggleNumber => self.set_number_1 = !self.set_number_1,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let set_1 = self.set_number_1;
        html! {
            <div>
                <p>{ self.number_1 }{ &self.operator }{ self.number_2 }</p>
                <div>
                    <button class="small-button" onclick={ctx.link().callback(move |_| Msg::SetNumber(set_1, 1.))}>
                        { "1" }
                    </button>
                    <button class="small-button" onclick={ctx.link().callback(move |_| Msg::SetNumber(set_1, 2.))}>
                        { "2" }
                    </button>
                    <button class="small-button" onclick={ctx.link().callback(move |_| Msg::SetNumber(set_1, 3.))}>
                        { "3" }
                    </button>
                    <button class="small-button" onclick={ctx.link().callback(|_| Msg::SetOperator(Operator::Mul))}>
                        { "*" }
                    </button>
                    <button class="small-button" onclick={ctx.link().callback(|_| Msg::SetOperator(Operator::Div))}>
                        { "/" }
                    </button>

                </div>
                <div>
                    <button class="small-button" onclick={ctx.link().callback(move |_| Msg::SetNumber(set_1, 4.))}>
                        { "4" }
                    </button>
                    <button class="small-button" onclick={ctx.link().callback(move |_| Msg::SetNumber(set_1, 5.))}>
                        { "5" }
                    </button>
                    <button class="small-button" onclick={ctx.link().callback(move |_| Msg::SetNumber(set_1, 6.))}>
                        { "6" }
                    </button>
                    <button class="small-button" onclick={ctx.link().callback(|_| Msg::SetOperator(Operator::Add))}>
                        { "+" }
                    </button>
                    <button class="small-button" onclick={ctx.link().callback(|_| Msg::SetOperator(Operator::Sub))}>
                        { "-" }
                    </button>
                </div>
                <div>
                    <button class="small-button" onclick={ctx.link().callback(move |_| Msg::SetNumber(set_1, 7.))}>
                        { "7" }
                    </button>
                    <button class="small-button" onclick={ctx.link().callback(move |_| Msg::SetNumber(set_1, 8.))}>
                        { "8" }
                    </button>
                    <button class="small-button" onclick={ctx.link().callback(move |_| Msg::SetNumber(set_1, 9.))}>
                        { "9" }
                    </button>
                    <button class="wide-button" onclick={ctx.link().callback(|_| Msg::Calculate)}>
                        { "=" }
                    </button>
                </div>
                <button class="wide-button" onclick={ctx.link().callback(|_| Msg::ToggleNumber)}>
                    {
                        if self.set_number_1 { "1" }
                        else { "2" }
                    }
                </button>

                <p class="counter">
                    { self.result }
                </p>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Calculator>::new().render();
}
