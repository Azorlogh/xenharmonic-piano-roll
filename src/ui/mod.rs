use druid::widget::WidgetExt;
use druid::Widget;

use crate::state::*;

pub mod layout_editor;
pub mod sheet_editor;

pub fn build() -> impl Widget<State> {
	sheet_editor::build().lens(editors::State::sheet_editor).lens(State::editors)
}

pub fn build_layout_editor() -> impl Widget<State> {
	layout_editor::build()
		.lens(editors::State::layout_editor)
		.lens(State::editors)
}
