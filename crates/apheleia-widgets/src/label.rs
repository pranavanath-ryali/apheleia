use apheleia_core::{Color::Red, style::Style};
use apheleia_ui::{
    contexts::{self, Commands, Context},
    node::{
        data::{DirtyRenderLevel::SimpleDirty, NodeData},
        node::NodeTrait,
    },
};

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
    fn initial_setup(&mut self, ctx: &mut Context, _data: &NodeData) {
        match self.overflow {
            TextOverflow::Scoll(_, _) => {
                ctx.add_command(Commands::RegisterForUpdate);
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut Context, data: &NodeData) {
        if let Some(size) = data.size {
            match &self.overflow {
                TextOverflow::Scoll(ticks_per_char, ticks_for_wait) => {
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

                    ctx.add_command(Commands::MarkRenderDirty(SimpleDirty));
                }
                _ => {}
            }
        }
    }

    fn render(&self, buf: &mut apheleia_core::buffer::Buffer, ctx: &Context, data: &NodeData) {
        println!("YAY");
        let size = data.size.unwrap();

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
            TextOverflow::DoNothing => {
                buf.write_line(0, 0, &self.text, self.style);
                return;
            }
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

    fn event(&mut self, ctx: &mut Context, data: &NodeData) {
        todo!()
    }
}

impl Label {
    pub fn new() -> Self {
        Label {
            overflow: TextOverflow::Ellipses,
            text: "Label Node".to_string(),
            style: None,
            alignment: LabelAlignment::Left,

            i: 0,
            counter: 0.,
            scroll_right_dir: false,
            should_scroll: true,
            scroll_wait_count: 0,
        }
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.text = label.to_string();
        self
    }

    pub fn with_overflow(mut self, overflow: TextOverflow) -> Self {
        self.overflow = overflow;
        self
    }

    pub fn with_alignment(mut self, alignment: LabelAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    pub fn build(self) -> Box<Self> {
        Box::new(self)
    }
}
