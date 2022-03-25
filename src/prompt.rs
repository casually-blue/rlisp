use crossterm::style::Color;
use reedline::{Prompt, PromptEditMode, PromptHistorySearch};

use alloc::borrow::Cow;

pub struct ReplPrompt;

impl Prompt for ReplPrompt {
    fn render_prompt_left(&self) -> Cow<'_, str> {
        ">> ".into()
    }
    fn render_prompt_right(&self) -> Cow<'_, str> {
        "".into()
    }

    fn render_prompt_indicator(&self, _prompt_mode: PromptEditMode) -> Cow<'_, str> {
        "".into()
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<'_, str> {
        "... ".into()
    }

    fn render_prompt_history_search_indicator(
        &self,
        _history_search: PromptHistorySearch,
    ) -> Cow<'_, str> {
        "".into()
    }

    fn get_prompt_color(&self) -> Color {
        Color::Red
    }
}
