use druid::widget::Label;
use druid::Widget;

pub fn ui_builder() -> impl Widget<u64> {
    Label::new("Hello World")
}