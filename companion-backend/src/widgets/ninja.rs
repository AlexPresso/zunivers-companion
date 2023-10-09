use crate::widget::{Widget, WidgetMetadata};

pub struct NinjaWidget;
impl Widget for NinjaWidget {
    fn metadata(&self) -> WidgetMetadata {
        WidgetMetadata {
            icon: String::from("https://i.imgur.com/2g2YE3B.png"),
            name: String::from("Ninja"),
            description: String::from("Calcul des commandes conseillées"),
            color: String::from("#FF3434"),
        }
    }

    fn render(&self) -> String {
        format!("Hello render {}", &self.metadata().name)
    }
}