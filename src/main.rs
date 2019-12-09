use orbtk::api::widget::StatesContext;
use orbtk::prelude::*;

//rbtk_api::widget::states_context::StatesContext<'_>
#[derive(Debug, Copy, Clone)]
enum Action {
    Increment(usize),
}

#[derive(Default, AsAny)]
pub struct MainViewState {
    num: usize,
    action: Option<Action>,
}

impl MainViewState {
    fn action(&mut self, action: impl Into<Option<Action>>) {
        self.action = action.into();
    }
}

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
            .counter(0)
            .result("Button count: 0")
            .child(
                Button::create()
                    .selector(Selector::from("button").id("btn"))
                    .min_size(100.0, 50.0)
                    .text(("result", id))
                    .on_click(move |states, _| -> bool {
                        state(id, states).action(Action::Increment(10));
                        true
                    })
                    .build(ctx),
            )
    }
}

impl State for MainViewState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if let Some(action) = self.action {
            match action {
                Action::Increment(digit) => {
                    *ctx.widget().get_mut::<usize>("counter") += digit as usize;
                    let counter = *ctx.widget().get::<usize>("counter");
                    ctx.widget().set(
                        "result",
                        String16::from(format!("Button count: {}", counter)),
                    );
                }
                _ => {}
            }

            // Is it possible to get rid of this line ?
            self.action = None;
        }
    }
}

widget!(MainView<MainViewState> {
    age:i32,
    counter:usize,
    result: String16
});

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
