use is_terminal::IsTerminal;
use owo_colors::{style, Style};

#[derive(Debug, Clone)]
pub struct GraphicalTheme {
    pub characters: ThemeCharacters,
    pub styles: ThemeStyles,
}

impl GraphicalTheme {
    pub fn ascii() -> Self {
        Self {
            characters: ThemeCharacters::ascii(),
            styles: ThemeStyles::ansi(),
        }
    }

    pub fn unicode() -> Self {
        Self {
            characters: ThemeCharacters::unicode(),
            styles: ThemeStyles::rgb(),
        }
    }

    pub fn unicode_nocolor() -> Self {
        Self {
            characters: ThemeCharacters::unicode(),
            styles: ThemeStyles::none(),
        }
    }

    pub fn none() -> Self {
        Self {
            characters: ThemeCharacters::ascii(),
            styles: ThemeStyles::none(),
        }
    }
}

impl Default for GraphicalTheme {
    fn default() -> Self {
        match std::env::var("NO_COLOR") {
            _ if !std::io::stdout().is_terminal() || !std::io::stderr().is_terminal() => {
                Self::ascii()
            }
            Ok(string) if string != "0" => Self::unicode_nocolor(),
            _ => Self::unicode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ThemeStyles {
    pub error: Style,
    pub warning: Style,
    pub advice: Style,
    pub help: Style,
    pub link: Style,
    pub linum: Style,
    pub highlights: Vec<Style>,
}

impl ThemeStyles {
    pub fn rgb() -> Self {
        Self {
            error: style().fg_rgb::<225, 80, 80>().bold(),
            warning: style().fg_rgb::<244, 191, 117>().bold(),
            advice: style().fg_rgb::<106, 159, 181>(),
            help: style().fg_rgb::<106, 159, 181>(),
            link: style().fg_rgb::<92, 157, 255>().bold(),
            linum: style().dimmed(),
            highlights: vec![
                style().fg_rgb::<246, 87, 248>(),
                style().fg_rgb::<30, 201, 212>(),
                style().fg_rgb::<145, 246, 111>(),
            ],
        }
    }

    pub fn ansi() -> Self {
        Self {
            error: style().red(),
            warning: style().yellow(),
            advice: style().cyan(),
            help: style().cyan(),
            link: style().bold(),
            linum: style().dimmed(),
            highlights: vec![
                style().magenta().bold(),
                style().yellow().bold(),
                style().green().bold(),
            ],
        }
    }

    pub fn none() -> Self {
        Self {
            error: style(),
            warning: style(),
            advice: style(),
            help: style(),
            link: style(),
            linum: style(),
            highlights: vec![style()],
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ThemeCharacters {
    pub hbar: char,
    pub vbar: char,
    pub xbar: char,
    pub vbar_break: char,

    pub uarrow: char,
    pub rarrow: char,

    pub ltop: char,
    pub mtop: char,
    pub rtop: char,
    pub lbot: char,
    pub rbot: char,
    pub mbot: char,

    pub lbox: char,
    pub rbox: char,

    pub lcross: char,
    pub rcross: char,

    pub underbar: char,
    pub underline: char,

    pub error: String,
    pub warning: String,
    pub advice: String,
}

impl ThemeCharacters {
    pub fn unicode() -> Self {
        Self {
            hbar: '─',
            vbar: '│',
            xbar: '┼',
            vbar_break: '·',
            uarrow: '▲',
            rarrow: '▶',
            ltop: '╭',
            mtop: '┬',
            rtop: '╮',
            lbot: '╰',
            mbot: '┴',
            rbot: '╯',
            lbox: '[',
            rbox: ']',
            lcross: '├',
            rcross: '┤',
            underbar: '┬',
            underline: '─',
            error: "×".into(),
            warning: "⚠".into(),
            advice: "☞".into(),
        }
    }

    pub fn emoji() -> Self {
        Self {
            hbar: '─',
            vbar: '│',
            xbar: '┼',
            vbar_break: '·',
            uarrow: '▲',
            rarrow: '▶',
            ltop: '╭',
            mtop: '┬',
            rtop: '╮',
            lbot: '╰',
            mbot: '┴',
            rbot: '╯',
            lbox: '[',
            rbox: ']',
            lcross: '├',
            rcross: '┤',
            underbar: '┬',
            underline: '─',
            error: "💥".into(),
            warning: "⚠️".into(),
            advice: "💡".into(),
        }
    }

    pub fn ascii() -> Self {
        Self {
            hbar: '-',
            vbar: '|',
            xbar: '+',
            vbar_break: ':',
            uarrow: '^',
            rarrow: '>',
            ltop: ',',
            mtop: 'v',
            rtop: '.',
            lbot: '`',
            mbot: '^',
            rbot: '\'',
            lbox: '[',
            rbox: ']',
            lcross: '|',
            rcross: '|',
            underbar: '|',
            underline: '^',
            error: "x".into(),
            warning: "!".into(),
            advice: ">".into(),
        }
    }
}
