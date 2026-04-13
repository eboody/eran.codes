use maud::{Markup, Render};

pub(super) struct CardGrid {
    extra_class: Option<&'static str>,
    content: Markup,
}

impl CardGrid {
    const BASE_CLASS: &str = "ui-portfolio-card-grid";

    pub fn new(content: Markup) -> Self {
        Self {
            extra_class: None,
            content,
        }
    }

    pub fn extra_class(mut self, class: &'static str) -> Self {
        self.extra_class = Some(class);
        self
    }

    fn class_attr(&self) -> String {
        match self.extra_class {
            Some(extra_class) => format!("{} {extra_class}", Self::BASE_CLASS),
            None => Self::BASE_CLASS.to_string(),
        }
    }
}

impl Render for CardGrid {
    fn render(&self) -> maud::Markup {
        let class_attr = self.class_attr();

        maud::html! {
            div class=(class_attr) {
                (&self.content)
            }
        }
    }
}

pub(super) struct InsetCard {
    extra_class: Option<&'static str>,
    content: Markup,
}

impl InsetCard {
    const BASE_CLASS: &str = "ui-portfolio-card u-inset-card";

    pub fn new(content: Markup) -> Self {
        Self {
            extra_class: None,
            content,
        }
    }

    pub fn extra_class(mut self, class: &'static str) -> Self {
        self.extra_class = Some(class);
        self
    }

    fn class_attr(&self) -> String {
        match self.extra_class {
            Some(extra_class) => format!("{} {extra_class}", Self::BASE_CLASS),
            None => Self::BASE_CLASS.to_string(),
        }
    }
}

impl Render for InsetCard {
    fn render(&self) -> maud::Markup {
        let class_attr = self.class_attr();

        maud::html! {
            article class=(class_attr) {
                (&self.content)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum SurfaceTag {
    Header,
    Section,
}

pub(super) struct Surface {
    tag: SurfaceTag,
    extra_class: Option<&'static str>,
    content: Markup,
}

impl Surface {
    const BASE_CLASS: &str = "u-surface-card ui-portfolio-surface";

    pub fn header(content: Markup) -> Self {
        Self {
            tag: SurfaceTag::Header,
            extra_class: None,
            content,
        }
    }

    pub fn section(content: Markup) -> Self {
        Self {
            tag: SurfaceTag::Section,
            extra_class: None,
            content,
        }
    }

    pub fn extra_class(mut self, class: &'static str) -> Self {
        self.extra_class = Some(class);
        self
    }

    fn class_attr(&self) -> String {
        match self.extra_class {
            Some(extra_class) => format!("{} {extra_class}", Self::BASE_CLASS),
            None => String::from(Self::BASE_CLASS),
        }
    }
}

impl Render for Surface {
    fn render(&self) -> maud::Markup {
        let class_attr = self.class_attr();

        match self.tag {
            SurfaceTag::Header => maud::html! {
                header class=(class_attr) {
                    (&self.content)
                }
            },
            SurfaceTag::Section => maud::html! {
                section class=(class_attr) {
                    (&self.content)
                }
            },
        }
    }
}

pub(super) struct CardFooter {
    content: Markup,
}

impl CardFooter {
    const BASE_CLASS: &str = "ui-portfolio-card-footer";

    pub fn new(content: Markup) -> Self {
        Self { content }
    }
}

impl Render for CardFooter {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class=(Self::BASE_CLASS) {
                (&self.content)
            }
        }
    }
}
