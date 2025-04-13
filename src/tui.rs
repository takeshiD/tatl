use crate::message::CommitMessage;
use crate::provider::MessageProvider;

use color_eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind};
use futures::{stream::FuturesUnordered, StreamExt};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{CYAN, RED, SLATE},
        Color, Modifier, Style,
    },
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
    DefaultTerminal,
};
use std::time::Duration;

const TITLE_STYLE: Style = Style::new().bg(CYAN.c700).add_modifier(Modifier::BOLD);
const CURSOR_STYLE: Style = Style::new().bg(SLATE.c700);
const SELECTED_STYLE: Style = Style::new().fg(CYAN.c200);
const NONSELECTED_STYLE: Style = Style::new().fg(SLATE.c200);
const MSGTYPE_STYLE: Style = Style::new().fg(CYAN.c500);
const BREAKINGCHANGE_STYLE: Color = RED.c600;
const SCOPE_STYLE: Color = CYAN.c200;

impl From<&CommitMessageItem> for ListItem<'_> {
    fn from(value: &CommitMessageItem) -> Self {
        let mut message = Line::default();
        if value.select {
            message.push_span(Span::styled("[🗸] ", SELECTED_STYLE))
        } else {
            message.push_span(Span::styled("[ ] ", NONSELECTED_STYLE))
        }
        if let Some(msgtype) = value.message.commit_type.clone() {
            message.push_span(Span::styled(format!("{}", msgtype), MSGTYPE_STYLE));
            if let Some(scope) = value.message.scope.clone() {
                message.push_span(Span::styled(format!("({})", scope), SCOPE_STYLE));
            }
            if value.message.breaking_change {
                message.push_span(Span::styled("!", BREAKINGCHANGE_STYLE));
            }
            message.push_span(Span::raw(": "));
        }
        message.push_span(Span::raw(value.message.description.clone()));
        ListItem::new(message)
    }
}

#[derive(Clone)]
struct CommitMessageItem {
    message: CommitMessage,
    select: bool,
}

impl CommitMessageItem {
    fn new(message: CommitMessage) -> Self {
        Self {
            message,
            select: false,
        }
    }
}

#[derive(Clone)]
struct CommitMessageList {
    messages: Vec<CommitMessageItem>,
    state: ListState,
}

impl CommitMessageList {
    fn new() -> Self {
        Self {
            messages: Vec::new(),
            state: ListState::default(),
        }
    }
}

#[derive(Clone)]
pub struct App {
    message_list: CommitMessageList,
    should_exit: bool,
}

// Constructor
impl App {
    pub fn new() -> Self {
        Self {
            message_list: CommitMessageList::new(),
            should_exit: false,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

// Key Logic
impl App {
    pub async fn run(
        &mut self,
        mut terminal: DefaultTerminal,
        provider: MessageProvider,
        diff_content: &str,
        generate_count: usize,
    ) -> Result<()> {
        let mut tasks = FuturesUnordered::new();
        for _ in 0..generate_count {
            tasks.push(provider.generate_message(diff_content, "Japanese"));
        }
        let period = Duration::from_millis(10);
        let mut interval = tokio::time::interval(period);
        let mut reader = EventStream::new();
        while !self.should_exit {
            let mut this = self.clone();
            tokio::select! {
                Some(msg) = tasks.next() => {
                    match msg {
                        Ok(message) => self.push_message(message),
                        Err(_) => panic!("Failed")
                    }
                },
                Some(Ok(event)) = reader.next() => {
                    match event {
                        Event::Key(key) => self.handle_key(key),
                            _ => panic!("Failed"),
                    }
                },
                _ = interval.tick() => {
                    terminal.draw(|frame| frame.render_widget(&mut this, frame.area()))?;
                },
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
    fn push_message(&mut self, message: CommitMessage) {
        self.message_list
            .messages
            .push(CommitMessageItem::new(message))
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
