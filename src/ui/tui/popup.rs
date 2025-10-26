use ratatui::{
    style::Style,
    widgets::{Block, Borders, List, ListItem},
    layout::{Rect, Layout, Constraint, Direction},
    Frame,
};

use crate::color_scheme::ColorScheme;
use crate::language::Language;
use crate::time_selection::TimeSelection;
use crate::settings::Settings;
use crate::ui::tui::app::App;

pub enum PopupContent {
    Language,
    ColorScheme,
    TimeSelection,
    WordNumberSelection,
    Settings,
    BatchSizeSelection,
    TopWordsSelection,
}

pub struct PopupState {
    pub open: bool,
    pub selected: usize,
}

pub struct PopupStates {
    pub language: PopupState,
    pub color_scheme: PopupState,
    pub time_selection: PopupState,
    pub word_number_selection: PopupState,
    pub settings: PopupState,
    pub batch_size_selection: PopupState,
    pub top_words_selection: PopupState,
}

pub trait PopupData {
    fn title(&self) -> &'static str;
    fn items(&self) -> Vec<String>;
    fn selected_index<'a>(&self, app: &'a App) -> &'a usize;
}

impl PopupData for PopupContent {
    fn title(&self) -> &'static str {
        match self {
            PopupContent::Language => "Select Language",
            PopupContent::ColorScheme => "Select Color Scheme",
            PopupContent::TimeSelection => "Select Time",
            PopupContent::WordNumberSelection => "Select Number of Words",
            PopupContent::Settings => "Select Setting",
            PopupContent::BatchSizeSelection => "Select Batch Size",
            PopupContent::TopWordsSelection => "Select Top Words",
        }
    }

    fn items(&self) -> Vec<String> {
        match self {
            PopupContent::Language => Language::all().iter().map(|x| x.to_string()).collect(),
            PopupContent::ColorScheme => ColorScheme::all().iter().map(|x| x.name().to_string()).collect(),
            PopupContent::TimeSelection => TimeSelection::all().iter().map(|x| x.to_string()).collect(),
            PopupContent::WordNumberSelection => vec!["25".to_string(), "50".to_string(), "100".to_string(), "200".to_string(), "500".to_string()],
            PopupContent::Settings => Settings::all().iter().map(|x| x.to_string()).collect(),
            PopupContent::BatchSizeSelection => vec!["10".to_string(), "25".to_string(), "50".to_string(), "100".to_string(), "200".to_string()],
            PopupContent::TopWordsSelection => vec!["100".to_string(), "200".to_string(), "500".to_string(), "1000".to_string()],
        }
    }

    fn selected_index<'a>(&self, app: &'a App) -> &'a usize {
        match self {
            PopupContent::Language => &app.popup_states.language.selected,
            PopupContent::ColorScheme => &app.popup_states.color_scheme.selected,
            PopupContent::TimeSelection => &app.popup_states.time_selection.selected,
            PopupContent::WordNumberSelection => &app.popup_states.word_number_selection.selected,
            PopupContent::Settings => &app.popup_states.settings.selected,
            PopupContent::BatchSizeSelection => &app.popup_states.batch_size_selection.selected,
            PopupContent::TopWordsSelection => &app.popup_states.top_words_selection.selected,
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let safe_percent_x = percent_x.min(95);
    let safe_percent_y = percent_y.min(95);

    if r.width < 10 || r.height < 6 {
        return Rect::new(
            r.x + 1,
            r.y + 1,
            (r.width.saturating_sub(2)).max(1),
            (r.height.saturating_sub(2)).max(1),
        );
    }

    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - safe_percent_y) / 2),
            Constraint::Percentage(safe_percent_y),
            Constraint::Percentage((100 - safe_percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - safe_percent_x) / 2),
            Constraint::Percentage(safe_percent_x),
            Constraint::Percentage((100 - safe_percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn render_popup(frame: &mut Frame, app: &App, area: Rect, color_scheme: ColorScheme, content: PopupContent) {
    let bg_color = color_scheme.bg_color();
    let main_color = color_scheme.main_color();
    let ref_color = color_scheme.ref_color();
    let border_color = color_scheme.border_color();

    let popup_area = centered_rect(30, 30, area);
    frame.render_widget(ratatui::widgets::Clear, popup_area);

    let items: Vec<ListItem> = content
        .items()
        .iter()
        .enumerate()
        .map(|(i, text)| {
            let style = if i == *content.selected_index(app) {
                Style::default().fg(bg_color).bg(main_color)
            } else {
                Style::default().fg(ref_color)
            };
            ListItem::new(text.clone()).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(content.title())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .style(Style::default().bg(bg_color)),
    );
    frame.render_widget(list, popup_area);
}
