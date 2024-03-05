use std::collections::BTreeMap;
use zellij_tile::prelude::*;
use zellij_widgets::prelude::*;

#[derive(Default, Clone)]
struct State {
    is_loading: bool,
    pressed_key: char,
    scroll_state: ScrollbarState,
    text: String,
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
        self.text = r##"Hello, everyone! This is the LONGEST TEXT EVER! I was inspired by the various other "longest texts ever" on the internet, and I wanted to make my own. So here it is! This is going to be a WORLD RECORD! This is actually my third attempt at doing this. The first time, I didn't save it. The second time, the Neocities editor crashed. Now I'm writing this in Notepad, then copying it into the Neocities editor instead of typing it directly in the Neocities editor to avoid crashing. It sucks that my past two attempts are gone now. Those actually got pretty long. Not the longest, but still pretty long. I hope this one won't get lost somehow. Anyways, let's talk about WAFFLES! I like waffles. Waffles are cool. Waffles is a funny word. There's a Teen Titans Go episode called "Waffles" where the word "Waffles" is said a hundred-something times. It's pretty annoying. There's also a Teen Titans Go episode about Pig Latin. Don't know what Pig Latin is? It's a language where you take all the consonants before the first vowel, move them to the end, and add '-ay' to the end. If the word begins with a vowel, you just add '-way' to the end. For example, "Waffles" becomes "Afflesway". I've been speaking Pig Latin fluently since the fourth grade, so it surprised me when I saw the episode for the first time. I speak Pig Latin with my sister sometimes. It's pretty fun. I like speaking it in public so that everyone around us gets confused. That's never actually happened before, but if it ever does, 'twill be pretty funny. By the way, "'twill" is a word I invented recently, and it's a contraction of "it will". I really hope it gains popularity in the near future, because "'twill" is WAY more fun than saying "it'll". "It'll" is too boring. Nobody likes boring. This is nowhere near being the longest text ever, but eventually it will be! I might still be writing this a decade later, who knows? But right now, it's not very long. But I'll just keep writing until it is the longest! Have you ever heard the song "Dau Dau" by Awesome Scampis? It's an amazing song. Look it up on YouTube! I play that song all the time around my sister! It drives her crazy, and I love it. Another way I like driving my sister crazy is by speaking my own made up language to her. She hates the languages I make! The only language that we both speak besides English is Pig Latin. I think you already knew that. Whatever. I think I'm gonna go for now. Bye! Hi, I'm back now. I'm gonna contribute more to this soon-to-be giant wall of text. I just realised I have a giant stuffed frog on my bed. I forgot his name. I'm pretty sure it was something stupid though. I think it was "FROG" in Morse Code or something. Morse Code is cool. I know a bit of it, but I'm not very good at it. I'm also not very good at French. I barely know anything in French, and my pronunciation probably sucks. But I'm learning it, at least. I'm also learning Esperanto."##.to_string();
        self.scroll_state = ScrollbarState::new(self.text.len());
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

        match self.pressed_key {
            'c' | 'p' => {
                // no loop for testing
                let _ = pane.draw(|frame| ui(frame, &self.text, &self.scroll_state));
            }
            _ => {}
        }
    }
}

fn ui(frame: &mut Frame, text: &str, state: &ScrollbarState) {
    let layouts = Layout::default()
        .direction(Orientation::Vertical)
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Percentage(85),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(frame.size());

    render_title(frame, layouts[0]);

    render_scrollbar(frame, layouts[1], text, state);
}

fn render_title(frame: &mut Frame, area: Geometry) {
    frame.render_widget(
        Paragraph::new("scollbar demo".slow_blink())
            .red()
            .alignment(Alignment::Center),
        area,
    );
}

fn render_scrollbar(frame: &mut Frame, area: Geometry, text: &str, state: &ScrollbarState) {
    frame.render_widget(
        Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Block"))
            .scroll((1, 1)),
        area,
    );
    frame.render_state_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("#"))
            .end_symbol(Some("#")),
        area,
        state,
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
