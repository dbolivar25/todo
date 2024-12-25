use reedline::{Prompt, PromptEditMode, PromptHistorySearch, PromptHistorySearchStatus};
use std::borrow::Cow;

#[derive(Clone)]
pub struct TodoPrompt;

impl Prompt for TodoPrompt {
    fn render_prompt_left(&self) -> Cow<str> {
        Cow::Borrowed("todo> ")
    }

    fn render_prompt_right(&self) -> Cow<str> {
        Cow::Borrowed("") // No right prompt
    }

    fn render_prompt_indicator(&self, _edit_mode: PromptEditMode) -> Cow<str> {
        Cow::Borrowed("") // No indicator needed since it's part of left prompt
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<str> {
        Cow::Borrowed("") // For multiline input
    }

    fn render_prompt_history_search_indicator(
        &self,
        history_search: PromptHistorySearch,
    ) -> Cow<str> {
        let prefix = match history_search.status {
            PromptHistorySearchStatus::Passing => "",
            PromptHistorySearchStatus::Failing => "failing ",
        };
        Cow::Owned(format!(
            "({}reverse-search: {}) ",
            prefix, history_search.term
        ))
    }
}
