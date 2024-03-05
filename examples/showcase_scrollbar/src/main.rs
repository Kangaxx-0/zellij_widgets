use std::collections::BTreeMap;
use zellij_tile::prelude::*;
use zellij_widgets::prelude::*;

#[derive(Default, Clone)]
struct State {
    is_loading: bool,
    pressed_key: char,
    scroll_state: ScrollbarStateInternal,
    text: Vec<String>,
}

#[derive(Default, Clone)]
struct ScrollbarStateInternal {
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[
            EventType::SessionUpdate,
            EventType::Key,
            EventType::ModeUpdate,
        ]);
        self.is_loading = true;
        self.text = vec![r##"Hello, everyone! This is the LONGEST TEXT EVER! I was inspired by the various other "longest texts ever" on the internet, and I wanted to make my own. So here it is! This is going to be a WORLD RECORD! This is actually my third attempt at doing this. The first time, I didn't save it. The second time, the Neocities editor crashed. Now I'm writing this in Notepad, then copying it into the Neocities editor instead of typing it directly in the Neocities editor to avoid crashing. It sucks that my past two attempts are gone now. Those actually got pretty long. Not the longest, but still pretty long. I hope this one won't get lost somehow. Anyways, let's talk about WAFFLES! I like waffles. Waffles are cool. Waffles is a funny word. There's a Teen Titans Go episode called "Waffles" where the word "Waffles" is said a hundred-something times. It's pretty annoying. There's also a Teen Titans Go episode about Pig Latin. Don't know what Pig Latin is? It's a language where you take all the consonants before the first vowel, move them to the end, and add '-ay' to the end. If the word begins with a vowel, you just add '-way' to the end. For example, "Waffles" becomes "Afflesway". I've been speaking Pig Latin fluently since the fourth grade, so it surprised me when I saw the episode for the first time. I speak Pig Latin with my sister sometimes. It's pretty fun. I like speaking it in public so that everyone around us gets confused. That's never actually happened before, but if it ever does, 'twill be pretty funny. By the way, "'twill" is a word I invented recently, and it's a contraction of "it will". I really hope it gains popularity in the near future, because "'twill" is WAY more fun than saying "it'll". "It'll" is too boring. Nobody likes boring. This is nowhere near being the longest text ever, but eventually it will be! I might still be writing this a decade later, who knows?"##.to_string(), "line-1".to_string(),"line-2".to_string(),"line-3".to_string(),"line-4".to_string(),"line-5".to_string(),"line-6".to_string(),"line-7".to_string(),"line-8".to_string(),"line-0".to_string(),"line-10".to_string(),"line-11".to_string(),"line-12".to_string(),"line-13".to_string(),"line-14".to_string(),"line-15".to_string(),"line-16".to_string(),"line-17".to_string(),"line-18".to_string(),"line-19".to_string(),"line-20".to_string(),"line-21".to_string(),"line-22".to_string(),"line-23".to_string(),"line-24".to_string(),"line-25".to_string(),"line-26".to_string(),"line-27".to_string(),"line-28".to_string(),"line-29".to_string(),"line-30".to_string(),"line-31".to_string(),"line-32".to_string(),];
        self.scroll_state = ScrollbarStateInternal {
            vertical_scroll_state: ScrollbarState::new(self.text.len()),
            horizontal_scroll_state: ScrollbarState::new(self.text[0].len()),
            vertical_scroll: 0,
            horizontal_scroll: 0,
        };
    }

    fn update(&mut self, event: Event) -> bool {
        matches!(event, Event::Key(c) if {
            self.handle_key(c);
            true
        })
    }

    fn render(&mut self, rows: usize, cols: usize) {
        let stdout = std::io::stdout();
        let mut pane = PluginPane::new(stdout, rows as u16, cols as u16);

        // convert Vec<String> to Vec<Line>
        let text = self
            .text
            .iter()
            .map(|s| Line::raw(s.clone()))
            .collect::<Vec<Line>>();

        match self.pressed_key {
            'c' | 'p' => {
                // no loop for testing
                let _ = pane.draw(|frame| ui(frame, text, &self.scroll_state));
            }
            _ => {}
        }
    }
}

fn ui(frame: &mut Frame, text: Vec<Line>, state: &ScrollbarStateInternal) {
    let layouts = Layout::default()
        .direction(Orientation::Vertical)
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Percentage(50),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let scrollbar_layout = Layout::default()
        .direction(Orientation::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(layouts[1]);

    render_title(frame, layouts[0]);

    render_scrollbar(frame, scrollbar_layout[1], text, state);
}

fn render_title(frame: &mut Frame, area: Geometry) {
    frame.render_widget(
        Paragraph::new("scollbar demo".slow_blink())
            .red()
            .alignment(Alignment::Center),
        area,
    );
}

fn render_scrollbar(
    frame: &mut Frame,
    area: Geometry,
    text: Vec<Line>,
    state: &ScrollbarStateInternal,
) {
    frame.render_widget(
        Paragraph::new(text.clone())
            .block(Block::default().borders(Borders::ALL).title("paragraph"))
            .green(),
        area,
    );
    frame.render_state_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .symbols(style::symbols::scrollbar::VERTICAL)
            .begin_symbol(None)
            .track_symbol(None)
            .end_symbol(None),
        area.inner(&Margin {
            horizontal: 0,
            vertical: 1,
        }),
        &state.vertical_scroll_state,
    );
    frame.render_state_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .begin_symbol(None)
            .track_symbol(None)
            .end_symbol(None),
        area.inner(&Margin {
            horizontal: 1,
            vertical: 0,
        }),
        &state.horizontal_scroll_state,
    );
}

impl State {
    fn handle_key(&mut self, e: Key) {
        if let Key::Char(c) = e {
            if c == 'c' {
                self.pressed_key = c;
            }
        }
    }
}
