use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
};

use crate::widgets::known_size::KnownSize;

type RenderFn = Box<dyn FnOnce(Rect, &mut Buffer)>;

pub struct PopupLayout {
    rows: Vec<(Constraint, RenderFn)>,
    width: u16,
    height: u16,
}

impl PopupLayout {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            rows: Vec::new(),
            width,
            height,
        }
    }

    pub fn row(mut self, constraint: Constraint, widget: impl Widget + 'static) -> Self {
        self.rows.push((
            constraint,
            Box::new(move |area, buf| widget.render(area, buf)),
        ));
        self
    }
}

impl KnownSize for PopupLayout {
    fn width(&self) -> usize {
        self.width as usize
    }

    fn height(&self) -> usize {
        self.height as usize
    }
}

impl Widget for PopupLayout {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let constraints: Vec<Constraint> = self.rows.iter().map(|(c, _)| *c).collect();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(area);

        for (i, (_, render_fn)) in self.rows.into_iter().enumerate() {
            render_fn(chunks[i], buf);
        }
    }
}
