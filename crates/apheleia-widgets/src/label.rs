use apheleia_ui::node::node::NodeTrait;

pub enum TextOverflow {
    DoNothing,
    Ellipses,
    Scoll,
}

pub struct Label {
    pub overflow: TextOverflow,
    pub text: String,

    i: u16,
}
impl NodeTrait for Label {
    fn initial_setup(&mut self, ctx: &mut apheleia_ui::commands::InitialCallContext) {
        match self.overflow {
            TextOverflow::Scoll => {
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

    fn update(&mut self) {
    }

    fn render(
        &self,
        ctx: &mut apheleia_ui::contexts::RenderContext,
        buf: &mut apheleia_core::buffer::Buffer,
    ) {
        let size = &ctx.size;
        match self.overflow {
            TextOverflow::DoNothing => {
                buf.write_line(0, 0, &self.text, None);
            }
            TextOverflow::Scoll => {}
            TextOverflow::Ellipses => {
                if self.text.len() > size.0 as usize {
                    buf.write_line(
                        0,
                        0,
                        &(self.text.split_at((size.0 - 3) as usize).0.to_string() + "..."),
                        None,
                    );
                } else {
                    buf.write_line(0, 0, &self.text, None);
                }
            }
        }
    }
}

impl Label {
    pub fn new(text: &str) -> Self {
        Label {
            overflow: TextOverflow::Ellipses,
            text: text.to_string(),
        }
    }
}
