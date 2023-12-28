// TODO: Add keyboard support
use yew::{html, Html, Component, Context};

pub enum Msg {
    Calculate,
    ClickNumber(f32),
    ClickOperator(Operator),
    Backspace,
    Clear,
    // TODO: Add "LoadFromHistory(index)"
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
    result: String,
    number_1: String,
    number_2: String,
    operator: Operator,
    set_number_1: bool,
    fragile_input: bool,
    history: Vec<(f32, Operator, f32, f32)>,
}
impl Calculator {
    pub fn calculate(&mut self) {
        // TODO: Add calculation to history
        let num_1: f32 = self.number_1.parse().unwrap();
        let num_2: f32 = self.number_2.parse().unwrap();
        let res = match self.operator {
            Operator::Mul => num_1 * num_2,
            Operator::Div => num_1 / num_2,
            Operator::Add => num_1 + num_2,
            Operator::Sub => num_1 - num_2,
        };
        if res.is_nan() || res.is_infinite() {
            self.result = "Error".to_string();
            self.number_1 = "0".to_string();
        } else {
            self.result = res.to_string();
            self.number_1 = res.to_string();
        }
        self.set_number_1 = true;
        self.fragile_input = true;
    }
}

impl Component for Calculator {
    type Message = Msg;
    type Properties = ();
    
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            result: "0".to_string(),
            number_1: 0.0.to_string(),
            number_2: 0.0.to_string(),
            operator: Operator::Add,
            set_number_1: true,
            fragile_input: false,
            history: vec![]
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Calculate => self.calculate(),
            Msg::ClickNumber(n) => {
                if self.set_number_1 {
                    if self.number_1.parse::<f32>().unwrap() == 0. || self.fragile_input {
                        self.number_1 = n.to_string();
                    } else {
                        self.number_1.push_str(&n.to_string());
                    }
                    self.number_2 = "0".to_string();
                    self.operator = Operator::Add;
                } else if self.number_2.parse::<f32>().unwrap() == 0. {
                    self.number_2 = n.to_string();
                } else {
                    self.number_2.push_str(&n.to_string());
                }
                self.fragile_input = false;
            },

            Msg::ClickOperator(o) => {
                if !self.fragile_input && !self.set_number_1 {
                    self.calculate()
                }
                self.number_2 = "0".to_string();
                self.operator = o;
                self.set_number_1 = false;
                self.fragile_input = true;
            },
            Msg::Backspace => {
                if self.set_number_1 {
                    if self.number_1.len() <= 1 || self.fragile_input {
                        self.number_1 = "0".to_string();
                    } else {
                        self.number_1.pop();
                    }
                } else if self.number_2.parse::<f32>().unwrap() == 0. {
                    self.set_number_1 = true;
                } else if self.number_2.len() <= 1 {
                    self.number_2 = "0".to_string();
                } else {
                    self.number_2.pop();
                }
                self.fragile_input = false;
            },
            Msg::Clear => { // Reset
                self.number_1 = "0".to_string();
                self.number_2 = "0".to_string();
                self.set_number_1 = true;
                self.fragile_input = false;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div class="text-box">
                    <p>{ &self.number_1 }{ if !self.set_number_1 { self.operator.to_string() + &self.number_2 } else { "".to_string() } }</p>
                </div>
                <p class="result">
                    { &self.result }
                </p>
                <div class="number-buttons">
                    <button class="small-button" onclick={ctx.link().callback(move |_| Msg::ClickNumber(1.))}>
                        { "1" }
                    </button>
                    <button class="small-button" onclick={ctx.link().callback(move |_| Msg::ClickNumber(2.))}>
                        { "2" }
                    </button>
                    <button class="small-button" onclick={ctx.link().callback(move |_| Msg::ClickNumber(3.))}>
                        { "3" }
                    </button>
                    <button class="small-button" onclick={ctx.link().callback(move |_| Msg::ClickNumber(4.))}>
                        { "4" }
                    </button>
                    <button class="small-button" onclick={ctx.link().callback(move |_| Msg::ClickNumber(5.))}>
                        { "5" }
                    </button>
                    <button class="small-button" onclick={ctx.link().callback(move |_| Msg::ClickNumber(6.))}>
                        { "6" }
                    </button>
                    <button class="small-button" onclick={ctx.link().callback(move |_| Msg::ClickNumber(7.))}>
                        { "7" }
                    </button>
                    <button class="small-button" onclick={ctx.link().callback(move |_| Msg::ClickNumber(8.))}>
                        { "8" }
                    </button>
                    <button class="small-button" onclick={ctx.link().callback(move |_| Msg::ClickNumber(9.))}>
                        { "9" }
                    </button>
                    <button class="small-button" onclick={ctx.link().callback(move |_| Msg::ClickNumber(0.))}>
                        { "0" }
                    </button>
                </div>
                <div class="operator-buttons">
                    <button class="small-button" onclick={ctx.link().callback(|_| Msg::ClickOperator(Operator::Mul))}>
                        { "*" }
                    </button>
                    <button class="small-button" onclick={ctx.link().callback(|_| Msg::ClickOperator(Operator::Div))}>
                        { "/" }
                    </button>
                    <button class="small-button" onclick={ctx.link().callback(|_| Msg::ClickOperator(Operator::Add))}>
                        { "+" }
                    </button>
                    <button class="small-button" onclick={ctx.link().callback(|_| Msg::ClickOperator(Operator::Sub))}>
                        { "-" }
                    </button>
                    <button class="wide-button" onclick={ctx.link().callback(|_| Msg::Calculate)}>
                        { "=" }
                    </button>
                    <button class="wide-button" onclick={ctx.link().callback(|_| Msg::Backspace)}>
                        { "Backspace" }
                    </button>
                    <button class="wide-button" onclick={ctx.link().callback(|_| Msg::Clear)}>
                        { "Clear" }
                    </button>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Calculator>::new().render();
}
