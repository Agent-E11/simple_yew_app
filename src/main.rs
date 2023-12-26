use std::{net, path::Display};

use yew::{html, Html, Component, Context};

pub enum Msg {
    Calculate,
    SetNumber(bool, f32),
    SetOperator(Operator),
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
    history: Vec<f32>,
}

impl Component for Calculator {
    type Message = Msg;
    type Properties = ();
    
    fn create(ctx: &Context<Self>) -> Self {
        Self { result: 0., number_1: 0., number_2: 0., operator: Operator::Add, history: vec![] }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Calculate => unimplemented!(),
            Msg::SetNumber(set_1, n) => if set_1 { self.number_1 = n } else { self.number_2 = n},
            Msg::SetOperator(o) => self.operator = o,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>{ self.number_1 }{ &self.operator }{ self.number_2 }</p>
                <div>
                    <button class="button" onclick={ctx.link().callback(|_| Msg::SetNumber(true, 1.))}>
                        { "1" }
                    </button>
                    <button class="button" onclick={ctx.link().callback(|_| Msg::SetNumber(true, 2.))}>
                        { "2" }
                    </button>
                    <button class="button" onclick={ctx.link().callback(|_| Msg::SetNumber(true, 3.))}>
                        { "3" }
                    </button>
                </div>
                <div>
                    <button class="button" onclick={ctx.link().callback(|_| Msg::SetNumber(true, 4.))}>
                        { "4" }
                    </button>
                    <button class="button" onclick={ctx.link().callback(|_| Msg::SetNumber(true, 5.))}>
                        { "5" }
                    </button>
                    <button class="button" onclick={ctx.link().callback(|_| Msg::SetNumber(true, 6.))}>
                        { "6" }
                    </button>
                </div>
                <div>
                    <button class="button" onclick={ctx.link().callback(|_| Msg::SetNumber(true, 7.))}>
                        { "7" }
                    </button>
                    <button class="button" onclick={ctx.link().callback(|_| Msg::SetNumber(true, 8.))}>
                        { "8" }
                    </button>
                    <button class="button" onclick={ctx.link().callback(|_| Msg::SetNumber(true, 9.))}>
                        { "9" }
                    </button>
                </div>

                <p class="counter">
                    {self.result }
                </p>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Calculator>::new().render();
}
