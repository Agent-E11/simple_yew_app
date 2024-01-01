// TODO: Add keyboard support
use yew::{
    html,
    classes,
    BaseComponent,
    Component,
    Context,
    Html,
};

pub enum Msg {
    Calculate,
    ClickNumber(f32),
    ClickOperator(Operator),
    ClickDot,
    Backspace,
    Clear,
    LoadFromHistory(usize),
}
#[derive(Clone, Copy)]
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

fn number_button<COMP>(num: f32, ctx: &Context<COMP>) -> Html 
where
    COMP: BaseComponent,
    <COMP as yew::BaseComponent>::Message: std::convert::From<Msg>,
{
    html! {
        <button
            class={classes!("bg-slate-400", "text-black", "font-bold", "py-2", "px-4", "h-20", "rounded")}
            onclick={ctx.link().callback(move |_| Msg::ClickNumber(num))}>
            { format!("{num}") }
        </button>
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
        let num_1: f32 = self.number_1.parse().unwrap();
        let num_2: f32 = self.number_2.parse().unwrap();
        let res = match self.operator {
            Operator::Mul => num_1 * num_2,
            Operator::Div => num_1 / num_2,
            Operator::Add => num_1 + num_2,
            Operator::Sub => num_1 - num_2,
        };

        self.history.push((num_1, self.operator, num_2, res));

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
                    if (self.number_1.parse::<f32>().unwrap() == 0. && !self.number_1.contains('.')) || self.fragile_input {
                        self.number_1 = n.to_string();
                    } else {
                        self.number_1.push_str(&n.to_string());
                    }
                    self.number_2 = "0".to_string();
                    self.operator = Operator::Add;
                } else if self.number_2.parse::<f32>().unwrap() == 0. && !self.number_2.contains('.') {
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
            Msg::ClickDot => { // FIXME: when the number equals 0 (like 0. or 0.0000), the whole number is replaced
                if self.set_number_1 {
                    if !self.number_1.contains('.') {
                        self.number_1.push('.');
                    }
                } else if !self.number_2.contains('.') {
                    self.number_2.push('.');
                }
                self.fragile_input = false;
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
            },
            Msg::LoadFromHistory(i) => {
                match self.history.get(i) {
                    None => (),
                    Some(calc) => {
                        self.number_1 = calc.0.to_string();
                        self.operator = calc.1;
                        self.number_2 = calc.2.to_string();
                        self.result = calc.3.to_string();

                        self.set_number_1 = false;
                        self.fragile_input = false;
                    }
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="mx-auto px-4 sm:container sm:w-full md:w-1/2 lg:w-1/3 xl:w-1/4">
                <div>
                    <p>
                        { &self.number_1 }
                        {
                            if !self.set_number_1 {
                                self.operator.to_string() + &self.number_2
                            } else {
                                "".to_string()
                            }
                        }
                    </p>
                </div>
                <p>
                    { &self.result }
                </p>
                <div class="grid grid-cols-3 gap-1"> // Button panel
                    { number_button(1., ctx) }
                    { number_button(2., ctx) }
                    { number_button(3., ctx) }
                    { number_button(4., ctx) }
                    { number_button(5., ctx) }
                    { number_button(6., ctx) }
                    { number_button(7., ctx) }
                    { number_button(8., ctx) }
                    { number_button(9., ctx) }
                    { number_button(0., ctx) }
                    <button
                        class={classes!("bg-slate-400", "text-black", "font-bold", "py-2", "px-4", "h-20", "rounded")}
                        onclick={ctx.link().callback(move |_| Msg::ClickDot)}>
                        { "." }
                    </button>
                </div>
                <div class="operator-buttons">
                    <button onclick={ctx.link().callback(|_| Msg::ClickOperator(Operator::Mul))}>
                        { "*" }
                    </button>
                    <button onclick={ctx.link().callback(|_| Msg::ClickOperator(Operator::Div))}>
                        { "/" }
                    </button>
                    <button onclick={ctx.link().callback(|_| Msg::ClickOperator(Operator::Add))}>
                        { "+" }
                    </button>
                    <button onclick={ctx.link().callback(|_| Msg::ClickOperator(Operator::Sub))}>
                        { "-" }
                    </button>
                    <button onclick={ctx.link().callback(|_| Msg::Calculate)}>
                        { "=" }
                    </button>
                    <button onclick={ctx.link().callback(|_| Msg::Backspace)}>
                        { "\u{232b}" }
                    </button>
                    <button onclick={ctx.link().callback(|_| Msg::Clear)}>
                        { "C" }
                    </button>
                </div>
                <div>
                    <h1>{ "History" }</h1>
                    {
                        self.history.iter().enumerate().map(|index_calculation| {
                            let (i, c) = index_calculation;
                            html! {
                                <p onclick={ctx.link().callback(move |_| Msg::LoadFromHistory(i))}>
                                    { c.0 }{ c.1 }{ c.2 }{ " = " }{ c.3 }
                                </p>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Calculator>::new().render();
}
