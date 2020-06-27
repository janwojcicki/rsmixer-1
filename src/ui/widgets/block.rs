use crate::{
    draw_range, repeat_string,
    ui::{
        util::{get_style, Rect},
        Widget,
    },
    Result,
};

use std::io::Write;

use crossterm::{cursor::MoveTo, execute};

#[derive(Clone)]
pub struct BlockWidget {
    pub title: String,
    pub clean_inside: bool,
}

impl BlockWidget {
    pub fn default() -> Self {
        Self {
            title: String::from(""),
            clean_inside: false,
        }
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn clean_inside(mut self, clean: bool) -> Self {
        self.clean_inside = clean;
        self
    }
}

impl<W: Write> Widget<W> for BlockWidget {
    fn render(self, area: Rect, buf: &mut W) -> Result<()> {
        let top = get_style("normal").apply(format!(
            "┌{}{}┐",
            self.title,
            repeat_string!("─", area.width - 2 - self.title.len() as u16)
        ));
        let bot = get_style("normal").apply(format!("└{}┘", repeat_string!("─", area.width - 2)));
        execute!(buf, MoveTo(0, 0))?;
        write!(buf, "{}", top)?;
        execute!(buf, MoveTo(0, area.height - 1))?;
        write!(buf, "{}", bot)?;

        if self.clean_inside {
            let sides =
                get_style("normal").apply(format!("│{}│", repeat_string!(" ", area.width - 2)));
            for i in 0..area.height - 2 {
                execute!(buf, MoveTo(0, i + 1 + area.y))?;
                write!(buf, "{}", sides.clone())?;
            }
        } else {
            draw_range!(buf, "│", 0..1, 1..area.height - 1, get_style("normal"));
            draw_range!(
                buf,
                "│",
                area.width - 1..area.width,
                1..area.height - 1,
                get_style("normal")
            );
        }

        buf.flush()?;

        Ok(())
    }
}