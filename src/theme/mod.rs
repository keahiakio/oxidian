use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeVariant {
    CatppuccinMocha,
    GruvboxMaterial,
    Everforest,
    OneDark,
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub variant: ThemeVariant,
    // Base colors
    pub bg: Color,
    pub bg_dark: Color,
    pub bg_highlight: Color,
    pub fg: Color,
    pub fg_muted: Color,
    // Accent colors
    pub accent: Color,
    pub primary: Color,
    pub secondary: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub info: Color,
    // UI specific
    pub border: Color,
    pub border_active: Color,
    pub tab_active_bg: Color,
    pub tab_active_fg: Color,
    pub tab_inactive_bg: Color,
    pub tab_inactive_fg: Color,
    // Statusline modes
    pub mode_normal_bg: Color,
    pub mode_normal_fg: Color,
    pub mode_insert_bg: Color,
    pub mode_insert_fg: Color,
    pub mode_visual_bg: Color,
    pub mode_visual_fg: Color,
    pub mode_command_bg: Color,
    pub mode_command_fg: Color,
    // Markdown elements
    pub heading1: Color,
    pub heading2: Color,
    pub heading3: Color,
    pub heading4: Color,
    pub link: Color,
    pub tag: Color,
    pub code_bg: Color,
    pub code_fg: Color,
    pub task_done: Color,
    pub task_todo: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::catppuccin_mocha()
    }
}

impl Theme {
    pub fn catppuccin_mocha() -> Self {
        Self {
            variant: ThemeVariant::CatppuccinMocha,
            bg: Color::Rgb(30, 30, 46),            // #1E1E2E
            bg_dark: Color::Rgb(24, 24, 37),       // #181825
            bg_highlight: Color::Rgb(49, 50, 68),  // #313244
            fg: Color::Rgb(205, 214, 244),         // #CDD6F4
            fg_muted: Color::Rgb(147, 153, 178),   // #9399B2

            accent: Color::Rgb(203, 166, 247),     // Mauve #CBA6F7
            primary: Color::Rgb(137, 180, 250),    // Blue #89B4FA
            secondary: Color::Rgb(245, 194, 231),  // Pink #F5C2E7
            success: Color::Rgb(166, 227, 161),    // Green #A6E3A1
            warning: Color::Rgb(249, 226, 175),    // Yellow #F9E2AF
            danger: Color::Rgb(243, 139, 168),     // Red #F38BA8
            info: Color::Rgb(148, 226, 213),       // Teal #94E2D5

            border: Color::Rgb(88, 91, 112),       // Surface2 #585B70
            border_active: Color::Rgb(137, 180, 250), // Blue #89B4FA

            tab_active_bg: Color::Rgb(49, 50, 68),
            tab_active_fg: Color::Rgb(137, 180, 250),
            tab_inactive_bg: Color::Rgb(24, 24, 37),
            tab_inactive_fg: Color::Rgb(108, 112, 134),

            mode_normal_bg: Color::Rgb(137, 180, 250), // Blue
            mode_normal_fg: Color::Rgb(17, 17, 27),
            mode_insert_bg: Color::Rgb(166, 227, 161), // Green
            mode_insert_fg: Color::Rgb(17, 17, 27),
            mode_visual_bg: Color::Rgb(203, 166, 247), // Mauve
            mode_visual_fg: Color::Rgb(17, 17, 27),
            mode_command_bg: Color::Rgb(249, 226, 175), // Peach/Yellow
            mode_command_fg: Color::Rgb(17, 17, 27),

            heading1: Color::Rgb(203, 166, 247),   // Mauve
            heading2: Color::Rgb(137, 180, 250),   // Blue
            heading3: Color::Rgb(148, 226, 213),   // Teal
            heading4: Color::Rgb(166, 227, 161),   // Green
            link: Color::Rgb(137, 220, 235),       // Sky
            tag: Color::Rgb(250, 179, 135),        // Peach
            code_bg: Color::Rgb(24, 24, 37),       // Crust
            code_fg: Color::Rgb(245, 224, 220),    // Rosewater
            task_done: Color::Rgb(166, 227, 161),  // Green
            task_todo: Color::Rgb(243, 139, 168),  // Red
        }
    }

    pub fn gruvbox_material() -> Self {
        Self {
            variant: ThemeVariant::GruvboxMaterial,
            bg: Color::Rgb(40, 40, 40),            // #282828
            bg_dark: Color::Rgb(30, 30, 30),       // #1e1e1e
            bg_highlight: Color::Rgb(60, 56, 54),  // #3c3836
            fg: Color::Rgb(223, 191, 142),         // #dfbf8e
            fg_muted: Color::Rgb(146, 131, 116),   // #928374

            accent: Color::Rgb(211, 134, 155),     // Purple #d3869b
            primary: Color::Rgb(215, 153, 33),     // Yellow #d79921
            secondary: Color::Rgb(231, 138, 78),   // Orange #e78a4e
            success: Color::Rgb(169, 182, 101),    // Green #a9b665
            warning: Color::Rgb(215, 153, 33),     // Yellow #d79921
            danger: Color::Rgb(234, 105, 98),      // Red #ea6962
            info: Color::Rgb(137, 180, 130),       // Aqua #89b482

            border: Color::Rgb(80, 73, 69),
            border_active: Color::Rgb(215, 153, 33),

            tab_active_bg: Color::Rgb(60, 56, 54),
            tab_active_fg: Color::Rgb(215, 153, 33),
            tab_inactive_bg: Color::Rgb(30, 30, 30),
            tab_inactive_fg: Color::Rgb(146, 131, 116),

            mode_normal_bg: Color::Rgb(169, 182, 101),
            mode_normal_fg: Color::Rgb(40, 40, 40),
            mode_insert_bg: Color::Rgb(137, 180, 130),
            mode_insert_fg: Color::Rgb(40, 40, 40),
            mode_visual_bg: Color::Rgb(211, 134, 155),
            mode_visual_fg: Color::Rgb(40, 40, 40),
            mode_command_bg: Color::Rgb(231, 138, 78),
            mode_command_fg: Color::Rgb(40, 40, 40),

            heading1: Color::Rgb(231, 138, 78),
            heading2: Color::Rgb(215, 153, 33),
            heading3: Color::Rgb(169, 182, 101),
            heading4: Color::Rgb(137, 180, 130),
            link: Color::Rgb(125, 174, 163),
            tag: Color::Rgb(211, 134, 155),
            code_bg: Color::Rgb(30, 30, 30),
            code_fg: Color::Rgb(223, 191, 142),
            task_done: Color::Rgb(169, 182, 101),
            task_todo: Color::Rgb(234, 105, 98),
        }
    }

    pub fn everforest() -> Self {
        Self {
            variant: ThemeVariant::Everforest,
            bg: Color::Rgb(45, 53, 59),            // #2d353b
            bg_dark: Color::Rgb(35, 42, 46),       // #232a2e
            bg_highlight: Color::Rgb(58, 68, 74),  // #3a444a
            fg: Color::Rgb(211, 198, 170),         // #d3c6aa
            fg_muted: Color::Rgb(133, 146, 137),   // #859289

            accent: Color::Rgb(214, 153, 182),     // Purple #d699b6
            primary: Color::Rgb(167, 192, 128),    // Green #a7c080
            secondary: Color::Rgb(230, 152, 117),  // Orange #e69875
            success: Color::Rgb(167, 192, 128),
            warning: Color::Rgb(219, 188, 127),    // Yellow #dbbc7f
            danger: Color::Rgb(230, 126, 128),     // Red #e67e80
            info: Color::Rgb(127, 187, 179),       // Aqua #7fbbb3

            border: Color::Rgb(76, 88, 93),
            border_active: Color::Rgb(167, 192, 128),

            tab_active_bg: Color::Rgb(58, 68, 74),
            tab_active_fg: Color::Rgb(167, 192, 128),
            tab_inactive_bg: Color::Rgb(35, 42, 46),
            tab_inactive_fg: Color::Rgb(133, 146, 137),

            mode_normal_bg: Color::Rgb(167, 192, 128),
            mode_normal_fg: Color::Rgb(35, 42, 46),
            mode_insert_bg: Color::Rgb(127, 187, 179),
            mode_insert_fg: Color::Rgb(35, 42, 46),
            mode_visual_bg: Color::Rgb(214, 153, 182),
            mode_visual_fg: Color::Rgb(35, 42, 46),
            mode_command_bg: Color::Rgb(219, 188, 127),
            mode_command_fg: Color::Rgb(35, 42, 46),

            heading1: Color::Rgb(167, 192, 128),
            heading2: Color::Rgb(127, 187, 179),
            heading3: Color::Rgb(219, 188, 127),
            heading4: Color::Rgb(230, 152, 117),
            link: Color::Rgb(127, 187, 179),
            tag: Color::Rgb(214, 153, 182),
            code_bg: Color::Rgb(35, 42, 46),
            code_fg: Color::Rgb(211, 198, 170),
            task_done: Color::Rgb(167, 192, 128),
            task_todo: Color::Rgb(230, 126, 128),
        }
    }

    pub fn cycle_next(&self) -> Self {
        match self.variant {
            ThemeVariant::CatppuccinMocha => Self::gruvbox_material(),
            ThemeVariant::GruvboxMaterial => Self::everforest(),
            ThemeVariant::Everforest => Self::catppuccin_mocha(),
            ThemeVariant::OneDark => Self::catppuccin_mocha(),
        }
    }

    pub fn name(&self) -> &'static str {
        match self.variant {
            ThemeVariant::CatppuccinMocha => "Catppuccin Mocha",
            ThemeVariant::GruvboxMaterial => "Gruvbox Material",
            ThemeVariant::Everforest => "Everforest",
            ThemeVariant::OneDark => "OneDark",
        }
    }
}
