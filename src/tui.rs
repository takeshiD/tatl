use crate::message::{CommitMessage, ConventionalType};

use anyhow::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{CYAN, RED, SLATE},
        Color, Modifier, Style,
    },
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
    DefaultTerminal,
};

const TITLE_STYLE: Style = Style::new().bg(CYAN.c700).add_modifier(Modifier::BOLD);
const CURSOR_STYLE: Style = Style::new().bg(SLATE.c700);
const SELECTED_STYLE: Style = Style::new().fg(CYAN.c200);
const NONSELECTED_STYLE: Style = Style::new().fg(SLATE.c200);
const MSGTYPE_STYLE: Style = Style::new().fg(CYAN.c500);
const BREAKINGCHANGE_STYLE: Color = RED.c600;
const SCOPE_STYLE: Color = CYAN.c200;

impl From<&CommitMessage> for ListItem<'_> {
    fn from(value: &CommitMessage) -> Self {
        let mut message = Line::default();
        if value.select {
            message.push_span(Span::styled("[🗸] ", SELECTED_STYLE))
        } else {
            message.push_span(Span::styled("[ ] ", NONSELECTED_STYLE))
        }
        if let Some(msgtype) = value.msgtype.clone() {
            message.push_span(Span::styled(format!("{}", msgtype), MSGTYPE_STYLE));
            if let Some(scope) = value.scope.clone() {
                message.push_span(Span::styled(format!("({})", scope), SCOPE_STYLE));
            }
            if value.breaking_change {
                message.push_span(Span::styled("!", BREAKINGCHANGE_STYLE));
            }
            message.push_span(Span::raw(": "));
        }
        message.push_span(Span::raw(value.subject.clone()));
        ListItem::new(message)
    }
}

struct CommitMessageList {
    messages: Vec<CommitMessage>,
    state: ListState,
}

impl
    FromIterator<(
        Option<ConventionalType>,
        bool,
        Option<String>,
        String,
        Option<String>,
        bool,
    )> for CommitMessageList
{
    fn from_iter<
        T: IntoIterator<
            Item = (
                Option<ConventionalType>,
                bool,
                Option<String>,
                String,
                Option<String>,
                bool,
            ),
        >,
    >(
        iter: T,
    ) -> Self {
        let messages = iter
            .into_iter()
            .map(|(msgtype, breaking_change, scope, subject, body, select)| {
                CommitMessage::new(msgtype, breaking_change, scope, subject, body, select)
            })
            .collect();
        let state = ListState::default().with_selected(Some(0));
        Self { messages, state }
    }
}

pub struct App {
    message_list: CommitMessageList,
    should_exit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_exit: false,
            message_list: CommitMessageList::from_iter([
                (None, false, None, "Helloworld".to_string(), None, false),
                (
                    Some(ConventionalType::Fix),
                    false,
                    None,
                    "Helloworld".to_string(),
                    None,
                    false,
                ),
                (
                    Some(ConventionalType::Feat),
                    false,
                    None,
                    "Yesterday".to_string(),
                    None,
                    false,
                ),
                (
                    Some(ConventionalType::Build),
                    true,
                    None,
                    "Yesterday".to_string(),
                    None,
                    false,
                ),
                (
                    Some(ConventionalType::Docs),
                    true,
                    Some("lang".to_string()),
                    "Yesterday".to_string(),
                    None,
                    false,
                ),
            ]),
        }
    }
}

// Key Logic
impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }
    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('j') | KeyCode::Down => self.cursor_down(),
            KeyCode::Char('k') | KeyCode::Up => self.cursor_up(),
            KeyCode::Char('g') | KeyCode::Home => self.cursor_top(),
            KeyCode::Char('G') | KeyCode::End => self.cursor_end(),
            KeyCode::Char(' ') => self.select(),
            KeyCode::Enter => self.confirm(),
            _ => {}
        }
    }
    fn cursor_down(&mut self) {
        self.message_list.state.select_next();
    }
    fn cursor_up(&mut self) {
        self.message_list.state.select_previous();
    }
    fn cursor_top(&mut self) {
        self.message_list.state.select_first();
    }
    fn cursor_end(&mut self) {
        self.message_list.state.select_last();
    }
    fn select(&mut self) {
        if let Some(index) = self.message_list.state.selected() {
            self.message_list.messages[index].select = !self.message_list.messages[index].select;
        }
    }
    fn confirm(&mut self) {
        self.should_exit = true;
    }
}

// Rednering logic
impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, list_area] =
            Layout::vertical([Constraint::Length(2), Constraint::Fill(1)]).areas(area);
        App::render_header(header_area, buf);
        self.render_list(list_area, buf);
    }
}

impl App {
    fn render_header(area: Rect, buf: &mut Buffer) {
        let mut header = Line::default();
        header.push_span(Span::styled("TALT\n", TITLE_STYLE));
        header.push_span(Span::raw(
            "Move ↑↓ or 'j'/'k'  Select 'Space'  Confirm 'Enter'  Quit 'q'",
        ));
        Paragraph::new(header).render(area, buf);
    }
    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new().title(Line::raw("? Select suggested commit messages"));
        let items: Vec<ListItem> = self
            .message_list
            .messages
            .iter()
            .map(ListItem::from)
            .collect();
        let list = List::new(items)
            .block(block)
            .highlight_style(CURSOR_STYLE)
            .highlight_symbol("❯ ")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);
        StatefulWidget::render(list, area, buf, &mut self.message_list.state);
    }
}
