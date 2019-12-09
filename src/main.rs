use orbtk::api::widget::StatesContext;
use orbtk::prelude::*;

//rbtk_api::widget::states_context::StatesContext<'_>
#[derive(Debug, Copy, Clone)]
enum Action {
    Increment(i32),
}

#[derive(Default, AsAny)]
pub struct MainViewState {
    num: i32,
    action: Option<Action>,
}

impl MainViewState {
    fn action(&mut self, action: impl Into<Option<Action>>) {
        self.action = action.into();
    }
}

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").child(
            Grid::create()
                .rows(Rows::create().row(72.0).row("*").build())
                .child(
                    Button::create()
                        .min_size(100.0, 50.0)
                        .text(String::from("hello").to_string())
                        .on_click(move |states, _| -> bool {
                            state(id, states).action(Action::Increment(10));
                            true
                        })
                        .build(ctx),
                )
                .child(
                    TextBlock::create()
                        .width(0.0)
                        .height(14.0)
                        .text(String::from("hello2").to_string())
                        .selector(Selector::from("text-block").id("input"))
                        .vertical_alignment("start")
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}

impl State for MainViewState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if let Some(action) = self.action {
            match action {
                Action::Increment(digit) => {
                    self.num += digit;
                    println!("STATE {}", self.num);
                }
                _ => {}
            }
            self.action = None;
        }
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
