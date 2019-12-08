use orbtk::prelude::*;
use orbtk::widgets;

#[derive(Debug, Copy, Clone)]
enum Action {
    Increment(i32),
}

#[derive(Default)]
pub struct MainViewState {
    num: i32,
    action: Option<Action>,
}

struct Event {
    x: f32,
    y: f32,
}

impl MainViewState {
    fn action(&mut self, action: impl Into<Option<Action>>) {
        self.action = action.into();
    }
}

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").child(
            Button::create()
                .min_size(100.0, 50.0)
                .text(String::from("hello").to_string())
                .on_click(move |states| -> bool {
                    println!("hello {:?}", states);
                    //  state(id, states).action(Action::Increment(10));

                    //  state(id, states).action(Action::Operator(sight));
                    true
                })
                .build(ctx),
        )
    }
}

impl State for MainViewState {
    fn update(&self, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                Action::Increment(digit) => {
                    //    self.num += digit;
                    println!("Self {:?}", self.num);
                }

                _ => {}
            }
        }

        //  self.action = None;
    }
}

widget!(MainView<MainViewState> {});

fn main() {
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - minimal example")
                .position((100.0, 100.0))
                .size(400.0, 400.0)
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}

fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut MainViewState {
    states.get_mut(id)
}
