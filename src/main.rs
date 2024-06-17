use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
  html! {
  <div class="max-w-xl mx-auto">
      <h1 class="text-4xl font-bold text-center">{ "WoRSdle" }</h1>
      <div class="flex justify-center items-center">
        <Field />
      </div>
  </div>
  }
}

#[derive(Clone, PartialEq)]
pub enum LetterState {
  Grey,
  Black,
  Yellow,
  Green,
}

#[derive(PartialEq, Properties)]
pub struct TileProps {
  pub letter: Option<char>,
  #[prop_or(LetterState::Grey)]
  pub state: LetterState,
}

#[function_component]
pub fn Tile(props: &TileProps) -> Html {
  html! {
      <div class="inline-flex border-2 border-gray-600 w-full justify-center items-center text-4xl box-border">
      {
        if let Some(letter) = props.letter {
          html! {letter}
        } else {
          html! {}
        }
      }
      </div>
  }
}

#[function_component]
pub fn Field() -> Html {
  html! {
    <div class="grid grid-rows-6 gap-1 p-2 box-border w-[350px] h-[420px]">
      {
        (0..6).map(|row: i32| html! {
         <div class="grid grid-cols-5 gap-1">
          {
            (0..5).map(|col: i32| html! {
            <Tile key={format!("{}-{}", row, col)} letter={None} />
            }).collect::<Html>()
          }
         </div>
        }).collect::<Html>()
      }
    </div>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
