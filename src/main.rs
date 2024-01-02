// TODO: Add keyboard support
use yew::{
    html,
    classes,
    BaseComponent,
    Component,
    Context,
    Html,
    Classes,
};

#[derive(Clone, Copy)]
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

fn number_button<COMP>(num: f32, styles: Classes, ctx: &Context<COMP>) -> Html 
where
    COMP: BaseComponent,
    <COMP as yew::BaseComponent>::Message: std::convert::From<Msg>,
{
    html! {
        <button
            class={classes!("bg-slate-400", "text-black", "font-bold", "py-2", "px-4", "h-20", "rounded", styles)}
            onclick={ctx.link().callback(move |_| Msg::ClickNumber(num))}>
            { format!("{num}") }
        </button>
    }
}

fn button<COMP>(msg: Msg, styles: Classes, ctx: &Context<COMP>) -> Html
where
    COMP: BaseComponent,
    <COMP as yew::BaseComponent>::Message: std::convert::From<Msg>,
{
    html! {
        <button class={classes!("bg-slate-400", "text-black", "font-bold", "py-2", "px-4", "h-20", "rounded", styles)}
            onclick={ctx.link().callback(move |_| msg)}>
            {
                match msg {
                    Msg::Calculate => "=".to_string(),
                    Msg::ClickOperator(o) => {
                        o.to_string().trim().to_string()
                    },
                    Msg::ClickDot => ".".to_string(),
                    Msg::Backspace => "\u{232b}".to_string(),
                    Msg::Clear => "C".to_string(),
                    _ => String::new(),
                }
            }
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
            Msg::ClickDot => {
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
        let no_styles = Classes::new();
        html! {
            <div class={classes!("font-mono", "bg-slate-500", "mx-auto", "sm:mt-4", "p-4", "sm:rounded-lg", "sm:container", "sm:w-full", "md:w-1/2", "lg:w-1/3", "xl:w-1/4")}>
                <div class={classes!()}>
                    <p class={classes!("bg-slate-200", "rounded-t-lg", "p-1")}>
                        { &self.number_1 }
                        {
                            if !self.set_number_1 {
                                self.operator.to_string() + &self.number_2
                            } else {
                                "".to_string()
                            }
                        }
                    </p>
                    <p class={classes!("text-right", "bg-slate-200", "rounded-b-lg", "mb-3", "p-1")}>
                        { "= " }{ &self.result }
                    </p>
                </div>
                <div class="grid grid-cols-5 gap-3"> // Button panel
                    { number_button(1., no_styles.clone(), ctx) }
                    { number_button(2., no_styles.clone(), ctx) }
                    { number_button(3., no_styles.clone(), ctx) }
                    { button(Msg::Backspace, Classes::from("bg-slate-600"), ctx) }
                    { button(Msg::Clear, Classes::from("bg-slate-600"), ctx) }

                    { number_button(4., no_styles.clone(), ctx) }
                    { number_button(5., no_styles.clone(), ctx) }
                    { number_button(6., no_styles.clone(), ctx) }
                    { button(Msg::ClickOperator(Operator::Mul), Classes::from("bg-slate-600"), ctx) }
                    { button(Msg::ClickOperator(Operator::Div), Classes::from("bg-slate-600"), ctx) }

                    { number_button(7., no_styles.clone(), ctx) }
                    { number_button(8., no_styles.clone(), ctx) }
                    { number_button(9., no_styles.clone(), ctx) }
                    { button(Msg::ClickOperator(Operator::Add), Classes::from("bg-slate-600"), ctx) }
                    { button(Msg::ClickOperator(Operator::Sub), Classes::from("bg-slate-600"), ctx) }

                    { number_button(0., Classes::from("col-span-2"), ctx) }
                    { button(Msg::ClickDot, no_styles.clone(), ctx) }
                    { button(Msg::Calculate, Classes::from("col-span-2 bg-slate-600"), ctx) }
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
