use bon::Builder;

use crate::types::Text;
use crate::views::partials::components::Tab;
use crate::views::theme::Theme;

use super::Panel;

#[derive(Clone, Debug, Builder)]
pub struct Component {
    pub id: Text,
    pub title: Text,
    pub subtitle: Text,
    pub tabs: Vec<Tab>,
    pub panels: Vec<Panel>,
    #[builder(default)]
    pub theme: Theme,
}
