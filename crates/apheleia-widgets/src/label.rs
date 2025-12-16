use apheleia_core::{Color::Red, style::Style};
use apheleia_ui::{contexts::UpdateContext, node::node::NodeTrait};

pub enum TextOverflow {
    DoNothing,
    Ellipses,
    Scoll(u16, u16),
}

pub enum LabelAlignment {
    Left,
    Center,
    Right,
}

pub struct Label {
    pub overflow: TextOverflow,
    pub text: String,
    pub style: Option<Style>,
    pub alignment: LabelAlignment,

    i: u16,
    counter: f32,
    scroll_right_dir: bool,
    should_scroll: bool,
    scroll_wait_count: u16,
}
impl NodeTrait for Label {
    fn initial_setup(&mut self, ctx: &mut apheleia_ui::commands::InitialCallContext) {
        match self.overflow {
            TextOverflow::Scoll(_, _) => {
                ctx.add_command(
                    apheleia_ui::commands::IntialCallCommands::RegisterUpdateType(
                        apheleia_ui::rootnode::UpdateType::Update,
                    ),
                );
            }
            _ => {}
        }
    }

    fn event(&mut self) {}

    fn update(&mut self, ctx: &mut UpdateContext) {
        match &self.overflow {
            TextOverflow::Scoll(ticks_per_char, ticks_for_wait) => {
                if let Some(size) = ctx.size {
                    if self.text.len() > size.0 as usize {
                        self.counter += (1. / *ticks_per_char as f32);
                        if self.counter > 1. {
                            self.counter = 0.;

                            if !self.should_scroll {
                                self.scroll_wait_count += 1;
                                if self.scroll_wait_count >= *ticks_for_wait {
                                    self.scroll_wait_count = 0;
                                    self.should_scroll = true;

                                    if self.scroll_right_dir {
                                        self.i = self.text.len() as u16 - size.0;
                                    } else {
                                        self.i = 0;
                                    }
                                }
                            } else {
                                if self.scroll_right_dir {
                                    if self.i == 0 {
                                        self.scroll_right_dir = false;
                                        self.should_scroll = false;
                                    } else {
                                        self.i -= 1;
                                    }
                                } else {
                                    if self.i > self.text.len() as u16 - size.0 - 1 {
                                        self.scroll_right_dir = true;
                                        self.should_scroll = false;
                                    } else {
                                        self.i += 1;
                                    }
                                }
                            }
                        }
                    } else {
                        self.i = 0;
                    }
                }
            }
            _ => {}
        }
    }

    fn render(
        &self,
        ctx: &mut apheleia_ui::contexts::RenderContext,
        buf: &mut apheleia_core::buffer::Buffer,
    ) {
        let size = &ctx.size;

        if self.text.len() <= size.0 as usize {
            match self.alignment {
                LabelAlignment::Left => {
                    buf.write_line(0, 0, &self.text, self.style);
                }
                LabelAlignment::Right => {
                    buf.write_line(size.0 - self.text.len() as u16, 0, &self.text, self.style);
                }
                LabelAlignment::Center => {
                    buf.write_line(
                        (size.0 as f32 / 2.).ceil() as u16
                            - (self.text.len() as f32 / 2.).ceil() as u16,
                        0,
                        &self.text,
                        self.style,
                    );
                }
            }

            return;
        }
        match self.overflow {
            TextOverflow::DoNothing => {}
            TextOverflow::Scoll(_, _) => {
                buf.write_line(
                    0,
                    0,
                    &(self.text.split_at(self.i as usize).1.to_string()),
                    self.style,
                );

                return;
            }
            TextOverflow::Ellipses => {
                if self.text.len() > size.0 as usize {
                    buf.write_line(
                        0,
                        0,
                        &(self.text.split_at((size.0 - 3) as usize).0.to_string() + "..."),
                        self.style,
                    );

                    return;
                }
            }
        }

        buf.write_line(0, 0, &self.text, self.style);
    }
}

impl Label {
    pub fn new(
        text: &str,
        style: Option<Style>,
        alignment: Option<LabelAlignment>,
        overflow: Option<TextOverflow>,
    ) -> Self {
        Label {
            overflow: overflow.unwrap_or_else(|| TextOverflow::Ellipses),
            text: text.to_string(),
            style: style,
            alignment: alignment.unwrap_or_else(|| LabelAlignment::Left),

            i: 0,
            counter: 0.,
            scroll_right_dir: false,
            should_scroll: true,
            scroll_wait_count: 0,
        }
    }
}
