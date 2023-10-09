use crate::widget::{Widget, WidgetMetadata};

pub struct DustCalculatorWidget;
impl Widget for DustCalculatorWidget {
    fn metadata(&self) -> WidgetMetadata {
        WidgetMetadata {
            icon: "https://cdn.discordapp.com/emojis/948289012693610546.png?v=1".to_string(),
            name: "Dust Calculator".to_string(),
            description: "Minimiser l'impact du recyclage".to_string(),
            color: "#8C21BD".to_string(),
        }
    }

    fn render(&self) -> String {
        format!("Hello render {}", &self.metadata().name)
    }
}