// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};
use lazy_static::lazy_static;
use tauri::Manager;
use serde::{Serialize, Deserialize};
use companion_backend::widget::{Widget, WidgetMetadata};
use companion_backend::widgets::dust_calculator::DustCalculatorWidget;
use companion_backend::widgets::ninja::NinjaWidget;
use companion_backend::widgets::notifier::NotifierWidget;

type WidgetMap = HashMap<String, Box<dyn Widget>>;
lazy_static! {
    static ref WIDGETS: Mutex<WidgetMap> = Mutex::new(WidgetMap::new());
}

#[derive(Serialize, Deserialize)]
struct EventPayload {
    widget: String,
    name: String,
    data: String
}

fn get_widgets_map() -> MutexGuard<'static, WidgetMap> {
    WIDGETS.lock().unwrap()
}

fn init_widgets() {
    let mut widgets = get_widgets_map();
    widgets.insert(String::from("Ninja"), Box::new(NinjaWidget));
    widgets.insert(String::from("Dust Calculator"), Box::new(DustCalculatorWidget));
    widgets.insert(String::from("Notifier"), Box::new(NotifierWidget));
}

#[tauri::command]
fn get_widgets() -> Result<Vec<Box<WidgetMetadata>>, String> {
    let widgets = get_widgets_map();
    let mut widgets_metadata: Vec<Box<WidgetMetadata>> = Vec::new();

    for widget in widgets.values() {
        widgets_metadata.push(Box::new(widget.metadata()));
    }

    Ok(widgets_metadata)
}

#[tauri::command]
fn render_widget(widget_name: String) -> Option<String> {
    let map = get_widgets_map();
    if !map.contains_key(&widget_name) {
        return None
    }

    Some(map.get(&widget_name).unwrap().render())
}

fn emit_widget_event(payload: &str) {
    if let Ok(event_payload) = serde_json::from_str::<EventPayload>(&payload) {
        let map = get_widgets_map();

        if map.contains_key(&event_payload.widget) {
            map.get(&event_payload.widget).unwrap().on_event(
                event_payload.name,
                event_payload.data
            )
        }
    }
}

fn main() {
    init_widgets();

    tauri::Builder::default()
        .setup(|app| {
            app.listen_global("widget_event", |event| {
                emit_widget_event(event.payload().unwrap())
            });

            Ok(())
        }).invoke_handler(tauri::generate_handler![
            get_widgets,
            render_widget
        ]).run(tauri::generate_context!())
        .expect("error while running tauri application");
}
