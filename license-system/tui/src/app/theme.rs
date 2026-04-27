use ratatui::style::Color;

#[derive(Clone, Debug)]
pub struct Theme {
    pub name: String,
    pub bg: Color,
    pub fg: Color,
    pub accent: Color,
    pub highlight: Color,
    pub error: Color,
    pub success: Color,
    pub warning: Color,
    pub border: Color,
    pub title: Color,
    pub muted: Color,
}

impl Theme {
    pub fn dcdev() -> Self {
        Self {
            name: "dcdev".into(),
            bg: Color::Rgb(18, 8, 8),
            fg: Color::Rgb(240, 210, 210),
            accent: Color::Rgb(255, 60, 60),
            highlight: Color::Rgb(120, 20, 20),
            error: Color::Rgb(255, 30, 30),
            success: Color::Rgb(180, 255, 100),
            warning: Color::Rgb(255, 160, 40),
            border: Color::Rgb(160, 30, 30),
            title: Color::Rgb(255, 80, 80),
            muted: Color::Rgb(140, 70, 70),
        }
    }

    pub fn dark() -> Self {
        Self {
            name: "dark".into(),
            bg: Color::Rgb(26, 26, 46),
            fg: Color::Rgb(224, 224, 224),
            accent: Color::Rgb(0, 212, 255),
            highlight: Color::Rgb(100, 100, 200),
            error: Color::Rgb(255, 85, 85),
            success: Color::Rgb(80, 250, 123),
            warning: Color::Rgb(255, 183, 77),
            border: Color::Rgb(80, 80, 120),
            title: Color::Rgb(0, 212, 255),
            muted: Color::Rgb(100, 100, 130),
        }
    }

    pub fn light() -> Self {
        Self {
            name: "light".into(),
            bg: Color::Rgb(250, 250, 250),
            fg: Color::Rgb(40, 40, 40),
            accent: Color::Rgb(0, 120, 215),
            highlight: Color::Rgb(200, 200, 255),
            error: Color::Rgb(220, 50, 50),
            success: Color::Rgb(40, 180, 99),
            warning: Color::Rgb(230, 126, 34),
            border: Color::Rgb(180, 180, 180),
            title: Color::Rgb(0, 120, 215),
            muted: Color::Rgb(150, 150, 150),
        }
    }

    pub fn dracula() -> Self {
        Self {
            name: "dracula".into(),
            bg: Color::Rgb(40, 42, 54),
            fg: Color::Rgb(248, 248, 242),
            accent: Color::Rgb(139, 233, 253),
            highlight: Color::Rgb(98, 114, 164),
            error: Color::Rgb(255, 85, 85),
            success: Color::Rgb(80, 250, 123),
            warning: Color::Rgb(241, 250, 140),
            border: Color::Rgb(68, 71, 90),
            title: Color::Rgb(189, 147, 249),
            muted: Color::Rgb(98, 114, 164),
        }
    }

    pub fn nord() -> Self {
        Self {
            name: "nord".into(),
            bg: Color::Rgb(46, 52, 64),
            fg: Color::Rgb(236, 239, 244),
            accent: Color::Rgb(136, 192, 208),
            highlight: Color::Rgb(94, 129, 172),
            error: Color::Rgb(191, 97, 106),
            success: Color::Rgb(163, 190, 140),
            warning: Color::Rgb(235, 203, 139),
            border: Color::Rgb(76, 86, 106),
            title: Color::Rgb(129, 161, 193),
            muted: Color::Rgb(76, 86, 106),
        }
    }

    pub fn gruvbox() -> Self {
        Self {
            name: "gruvbox".into(),
            bg: Color::Rgb(40, 40, 40),
            fg: Color::Rgb(235, 219, 178),
            accent: Color::Rgb(254, 128, 25),
            highlight: Color::Rgb(215, 153, 33),
            error: Color::Rgb(251, 73, 52),
            success: Color::Rgb(184, 187, 38),
            warning: Color::Rgb(250, 189, 47),
            border: Color::Rgb(80, 73, 69),
            title: Color::Rgb(142, 192, 124),
            muted: Color::Rgb(146, 131, 116),
        }
    }

    pub fn by_name(name: &str) -> Self {
        match name {
            "dcdev" => Self::dcdev(),
            "dark" => Self::dark(),
            "light" => Self::light(),
            "dracula" => Self::dracula(),
            "nord" => Self::nord(),
            "gruvbox" => Self::gruvbox(),
            _ => Self::dcdev(),
        }
    }

    pub fn names() -> Vec<String> {
        vec!["dcdev", "dark", "light", "dracula", "nord", "gruvbox"]
            .into_iter()
            .map(String::from)
            .collect()
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dcdev()
    }
}
