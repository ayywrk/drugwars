use crate::utils::StringManips;

pub trait BoxContent {
    fn get_lines(&self, width: usize) -> Vec<String>;
    fn len(&self) -> usize;
}

pub trait Part {
    fn get_lines(&self, width: usize) -> Vec<String>;
}

#[derive(Default, Clone)]
pub struct RenderBoxContent<const N: usize> {
    header: Option<[String; N]>,
    content: Vec<[String; N]>,
    sizes: Option<[usize; N]>,
}

impl<const N: usize> RenderBoxContent<N> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn header(&mut self, header: [String; N]) -> &mut Self {
        self.header = Some(header);
        self
    }

    pub fn sizes(&mut self, sizes: [usize; N]) -> &mut Self {
        self.sizes = Some(sizes);
        self
    }

    pub fn add_row(&mut self, row: [String; N]) -> &mut Self {
        self.content.push(row);
        self
    }
    pub fn get(&self) -> Self {
        self.clone()
    }
}

impl<const N: usize> BoxContent for RenderBoxContent<N> {
    fn get_lines(&self, width: usize) -> Vec<String> {
        //let column_width = width / N;

        let mut lines = vec![];

        if let Some(header) = &self.header {
            let mut header_line = String::new();

            for (idx, cell) in header.iter().enumerate() {
                let column_width = match self.sizes {
                    Some(sizes) => sizes[idx],
                    None => width / N,
                };

                let value = &cell.pretty_truncate(column_width);
                let spaces = " ".repeat(column_width - value.len());
                header_line += &format!("{}{}", value, spaces);
            }
            header_line += &" ".repeat(width - header_line.len());
            lines.push(header_line);
        }

        for row in &self.content {
            let mut row_line = String::new();

            for (idx, cell) in row.iter().enumerate() {
                let column_width = match self.sizes {
                    Some(sizes) => sizes[idx],
                    None => width / N,
                };

                let value = &cell.pretty_truncate(column_width);
                let spaces = " ".repeat(column_width - value.irc_safe_len());
                row_line += &format!("{}{}", value, spaces);
            }

            row_line += &" ".repeat(width - row_line.irc_safe_len());
            lines.push(row_line);
        }
        lines
    }

    fn len(&self) -> usize {
        let mut len = self.content.len();

        if self.header.is_some() {
            len += 1;
        }
        len
    }
}

#[derive(Default, Clone)]
pub struct RenderBox<'a, const N: usize> {
    headers: Option<[String; N]>,
    columns: Option<[&'a dyn BoxContent; N]>,
    sizes: Option<[usize; N]>,
}

impl<'a, const N: usize> RenderBox<'a, N> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn headers(&mut self, headers: [String; N]) -> &mut Self {
        self.headers = Some(headers);
        self
    }

    pub fn sizes(&mut self, sizes: [usize; N]) -> &mut Self {
        self.sizes = Some(sizes);
        self
    }

    pub fn add_content(&mut self, columns: [&'a dyn BoxContent; N]) -> &mut Self {
        self.columns = Some(columns);
        self
    }

    pub fn get(&self) -> Self {
        self.clone()
    }
}

impl<'a, const N: usize> Part for RenderBox<'a, N> {
    fn get_lines(&self, width: usize) -> Vec<String> {
        let column_width = (width / N) - 3;

        let max_rows = self
            .columns
            .unwrap()
            .iter()
            .fold(0, |acc, elem| acc.max(elem.len()));

        let column_lines = self
            .columns
            .unwrap()
            .iter()
            .map(|column| column.get_lines(column_width))
            .collect::<Vec<_>>();

        let mut lines = vec![];

        match &self.headers {
            Some(column_headers) => {
                let mut line = String::new();

                let mut first = true;
                for header in column_headers {
                    if first {
                        line += "╭";
                        first = false;
                    } else {
                        line += "┬";
                    }
                    let value = format!("{} ", &header.pretty_truncate(column_width));
                    line += &format!(" {} {}", value, "─".repeat(column_width - value.len()));
                }

                if line.irc_safe_len() == width {
                    line.pop();
                } else if line.irc_safe_len() < width - 1 {
                    line += &"─".repeat(width - line.irc_safe_len() - 1);
                }

                line += "╮";
                lines.push(line);
            }
            None => {
                lines.push(format!("╭{}╮", "─".repeat(width - 2)));
            }
        }

        if max_rows > 0 {
            for row_index in 0..max_rows {
                let mut line = String::new();

                for col_index in 0..N {
                    if row_index >= column_lines[col_index].len() {
                        line += &format!("│ {} ", " ".repeat(column_width));
                    } else {
                        let column_line = &column_lines[col_index][row_index];
                        line += &format!("│ {} ", column_line);
                    }
                }

                if line.irc_safe_len() == width {
                    line.pop();
                } else if line.irc_safe_len() < width - 1 {
                    line += &" ".repeat(width - line.irc_safe_len() - 1);
                }

                line += "│";

                lines.push(line);
            }
        }

        let mut bottom_line = String::new();

        let mut first = true;
        for _ in 0..N {
            if first {
                bottom_line += "╰";
                first = false;
            } else {
                bottom_line += "┴";
            }
            bottom_line += &format!("{}", "─".repeat(width / N - 1));
        }

        if bottom_line.irc_safe_len() == width {
            bottom_line.pop();
        } else if bottom_line.irc_safe_len() < width - 1 {
            bottom_line += &"─".repeat(width - bottom_line.irc_safe_len() - 1);
        }

        bottom_line += "╯";
        lines.push(bottom_line);

        lines
    }
}
pub struct Renderer<'a> {
    width: usize,
    boxes: Vec<&'a dyn Part>,
}

impl<'a> Renderer<'a> {
    pub fn new(width: usize) -> Self {
        Self {
            width: width,
            boxes: vec![],
        }
    }

    pub fn add_box(&mut self, element: &'a impl Part) -> &mut Self {
        self.boxes.push(element);
        self
    }

    pub fn build(&self) -> Vec<String> {
        let out = self
            .boxes
            .iter()
            .map(|elem| elem.get_lines(self.width))
            .flatten()
            .collect::<Vec<_>>();

        out
    }
}
