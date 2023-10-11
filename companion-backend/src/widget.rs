use std::any::Any;
use serde::Serialize;

#[derive(Serialize)]
pub struct WidgetMetadata {
    pub icon: String,
    pub name: String,
    pub description: String,
    pub color: String
}

pub trait Widget: Any + Send + Sync {
    fn metadata(&self) -> WidgetMetadata {
        WidgetMetadata {
            icon: String::from("default icon"),
            name: String::from("default name"),
            description: String::from("default description"),
            color: String::from("default color")
        }
    }

    fn on_widget_load(&self) {}
    fn on_widget_unload(&self) {}

    fn on_event(&self, name: String, data: String) {}
    fn render(&self) -> String;
}