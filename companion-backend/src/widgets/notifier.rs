use crate::widget::{Widget, WidgetMetadata};

pub struct NotifierWidget;
impl Widget for NotifierWidget {
    fn metadata(&self) -> WidgetMetadata {
        WidgetMetadata {
            icon: "https://zunivers.zerator.com/img/logo-mini.aea51074.png".to_string(),
            name: "Notifier".to_string(),
            description: "Recevoir les notifications ZUnivers".to_string(),
            color: "#05b600".to_string()
        }
    }

    fn render(&self) -> String {
        format!("Hello render {}", &self.metadata().name)
    }
}