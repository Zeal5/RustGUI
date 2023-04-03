use druid::{WindowDesc, AppLauncher};
mod ui;
use ui::ui_builder;


fn main() {
    let main_window = WindowDesc::new(ui_builder())
    .window_size((400.0,400.0))
    .title("My TODO APP");

    AppLauncher::with_window(main_window)
    .launch(0)
    .expect("not working")

}
