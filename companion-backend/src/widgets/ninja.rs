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

    fn on_event(&self, name: String, data: String) {
        match name.as_str() {
            "advise" => self.advise(data),
            _ => {}
        }
    }

    fn render(&self) -> String {
        "<button onclick=\"emitEvent('Ninja', 'advise', 'alexpresso')\">\
            Recevoir des conseils
        </button>".to_string()
    }
}

impl NinjaWidget {
    fn advise(&self, discord_username: String) {
        println!("{}", discord_username);
    }
}