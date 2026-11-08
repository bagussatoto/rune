//! A list with state: keyed rows you can add, tick off and remove. Rows animate in and out on
//! their own and the progress bar springs to its new width; the view only says where things are.
//! `cargo run -p todo -- --snap todo.png`.

use rune::tok::{self, Theme, radius, type_};
use rune::{AppSpec, Cx, El, ScrollSpec, Spring, TextEdit, col, el, row, text};
use rune_kit::{Badge, BadgeVariant, Divider, Kit, TextInput};

struct Todo { id: u32, title: String, done: bool }

struct State {
    todos: Vec<Todo>,
    next: u32,
    input: TextEdit,
}

impl Default for State {
    fn default() -> Self {
        let titles = ["Write the view", "Run it headless", "Ship it"];
        let todos = titles.iter().zip(0..)
            .map(|(title, id)| Todo { id, title: title.to_string(), done: id == 0 });
        State { todos: todos.collect(), next: 3, input: TextEdit::default() }
    }
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    let t = tok::theme(true);
    let kit = Kit::neutral(true);
    let mut list = col().key("list").gap(4.0);
    for todo in &s.todos {
        let id = todo.id;
        let check = el().key("check").size(16.0, 16.0).radius(8.0)
            .fill(if todo.done { tok::accent(0, true) } else { t.control_off })
            .on_press(move |s: &mut State| {
                if let Some(t) = s.todos.iter_mut().find(|t| t.id == id) { t.done = !t.done }
            });
        let title = text(todo.title.clone()).font(type_::BODY).grow(1.0)
            .color(if todo.done { t.text_secondary } else { t.text });
        let status = if todo.done {
            Badge::new("Done").variant(BadgeVariant::Accent).build(&kit)
        } else {
            Badge::new("Pending").variant(BadgeVariant::Subtle).build(&kit)
        };
        let remove = text("Remove").font(type_::LABEL).color(t.text_secondary)
            .on_press(move |s: &mut State| s.todos.retain(|t| t.id != id));
        list = list.child(
            row().key(format!("todo-{id}")).h(36.0).px(12.0).gap(12.0).center_y()
                .radius(radius::SM).fill(t.card)
                .child(check).child(title).child(status).child(remove),
        );
    }
    let done = s.todos.iter().filter(|t| t.done).count();
    let total = s.todos.len();

    let header = row().w_full().between().center_y()
        .child(
            row().gap(8.0).center_y()
                .child(text("Tasks").font(type_::TITLE).color(t.text))
                .child(
                    Badge::new(format!("{done}/{total}"))
                        .variant(if done == total && total > 0 { BadgeVariant::Accent } else { BadgeVariant::Neutral })
                        .build(&kit)
                )
        )
        .child(text(format!("{done} of {total} done")).font(type_::LABEL).color(t.text_secondary));

    let input_field = TextInput::new(&s.input)
        .placeholder("What needs to be done?")
        .on_edit(|s: &mut State| &mut s.input)
        .key("todo-input")
        .width(None)
        .build(&kit);

    let add_btn = text("+ Add").key("add").font(type_::LABEL).color(tok::accent(0, true))
        .on_press(|s: &mut State| {
            let val = s.input.text.trim().to_string();
            if !val.is_empty() {
                s.todos.push(Todo { id: s.next, title: val, done: false });
                s.next += 1;
                s.input.text.clear();
            }
        });

    let mut actions = row().w_full().between().center_y().child(add_btn);
    if done > 0 {
        let clear = text("Clear completed").key("clear").font(type_::LABEL).color(t.text_secondary)
            .on_press(|s: &mut State| s.todos.retain(|t| !t.done));
        actions = actions.child(clear);
    }

    let footer = col().w_full().gap(8.0).child(input_field).child(actions);

    if s.todos.is_empty() {
        list = list.child(
            row().key("empty").h(36.0).px(12.0).center_y().radius(radius::SM).fill(t.card)
                .child(text("All caught up! 🎉").font(type_::BODY).color(t.text_secondary)),
        );
    }

    col().w_full().h_full().padding(24.0).gap(12.0).fill(t.bg)
        .child(header)
        .child(progress(done, total, t))
        .child(Divider::horizontal().build(&kit))
        // Past what the window holds, the list scrolls, following new rows, and footer stays.
        .child(list.scroll(ScrollSpec { stick_bottom: true }).shrink().min_h(0.0))
        .child(footer)
}

fn progress(done: usize, total: usize, t: Theme) -> El {
    let width = 312.0 * done as f32 / total.max(1) as f32;
    el().key("track").w_full().h(4.0).radius(2.0).fill(t.control_off).child(
        el().key("bar").w(width).h(4.0).radius(2.0).fill(tok::accent(0, true))
            .motion(Spring::new(0.4, 0.2)),
    )
}

fn main() {
    rune::run(AppSpec::new("Todo", (380.0, 310.0), State::default, view));
}
