//! The smallest Rune app: one piece of state, a view of it, and a switch that flips it on a spring.
//! `cargo run -p hello` opens the window; `-- --snap hello.png --scene on` renders it headless.

use rune::tok::{self, motion, type_};
use rune::{AppSpec, Cx, El, col, el, row, text};

#[derive(Default)]
struct State {
    on: bool,
    flips: u32,
    color_index: usize,
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    let t = tok::theme(true);
    let active_color = tok::accent(s.color_index, true);

    let label = text(if s.on { "Lights on" } else { "Lights off" })
        .font(type_::TITLE).color(t.text);

    let knob = el().key("knob").size(18.0, 18.0).radius(9.0).fill(t.knob)
        .absolute(if s.on { 18.0 } else { 2.0 }, 2.0);

    // Screen readers hear "Lights, switch, on": the label names it, `checked` says its state.
    let switch = el().key("switch").role("switch").label("Lights").checked(s.on).focusable(true).size(38.0, 22.0).radius(11.0)
        .fill(if s.on { active_color } else { t.control_off })
        .motion(motion::SNAPPY)
        .on_press(|s: &mut State| {
            s.on = !s.on;
            s.flips += 1;
        })
        .child(knob);

    let stats = text(if s.flips == 0 {
        "Tap the switch to turn on".to_string()
    } else {
        format!("Flipped {} {}", s.flips, if s.flips == 1 { "time" } else { "times" })
    })
    .font(type_::LABEL)
    .color(t.text_secondary);

    let dots = row().gap(8.0).center().children((0..4).map(|i| {
        let is_sel = s.color_index == i;
        el().key(format!("color-{i}")).size(16.0, 16.0).radius(8.0)
            .fill(tok::accent(i, true))
            .border(if is_sel { 2.0 } else { 0.5 }, if is_sel { t.text } else { t.control_off })
            .on_press(move |s: &mut State| s.color_index = i)
    }));

    col().w_full().h_full().center().gap(12.0).fill(t.bg)
        .child(label)
        .child(switch)
        .child(stats)
        .child(dots)
}

fn main() {
    rune::run(AppSpec::<State> {
        scenes: &[("on", |s| s.on = true)],
        ..AppSpec::new("Hello", (320.0, 240.0), State::default, view)
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use rune::{Env, Runtime};

    #[test]
    fn a_tap_flips_the_switch_and_the_knob_springs_across() {
        let env = Env { system_dark: true, scale: 2.0 };
        let mut rt = Runtime::new(State::default(), view, (320.0, 200.0), env);
        rt.frame(0.0);
        let knob_x = |rt: &Runtime<State>| rt.presented("knob").unwrap().0[0];
        let off = knob_x(&rt);

        let (x, y) = rt.find("switch").unwrap();
        rt.pointer_down(x, y);
        rt.pointer_up(x, y);
        for _ in 0..6 { rt.frame(1.0 / 60.0); }
        assert!(rt.state.on);
        let mid = knob_x(&rt);
        for _ in 0..60 { rt.frame(1.0 / 60.0); }
        let on = knob_x(&rt);
        assert!(off < mid && mid < on, "the knob travels: {off} → {mid} → {on}");
        assert!((on - off - 16.0).abs() < 0.01, "and lands 16 points over");
    }
}
