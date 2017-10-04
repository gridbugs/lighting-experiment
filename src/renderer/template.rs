use std::collections::HashMap;
use handlebars::Handlebars;
use toml::Value;

use renderer::sizes;
use renderer::tile_renderer::instance_flags;
use content::{DepthType, SpriteEffect};

macro_rules! include_shader_part {
    ($table:expr, $handlebars:expr, $key:expr, $file_name:expr) => {
        {
            let bytes = include_bytes!(concat!("shaders/", $file_name));
            let shader_str = ::std::str::from_utf8(bytes)
                .expect("Failed to convert part to utf8");
            let expanded = $handlebars.template_render(shader_str, &$table)
                .expect("Failed to render part template");
            $table.insert($key, Value::String(expanded));
        }
    }
}

pub fn make_shader_template_context() -> (Handlebars, HashMap<&'static str, Value>) {
    let handlebars = {
        let mut h = Handlebars::new();
        h.register_escape_fn(|input| input.to_string());
        h
    };

    use self::Value::*;
    let mut table = hashmap!{
        "FLAGS_ENABLED" => Integer(instance_flags::ENABLED as i64),
        "FLAGS_SPRITE_EFFECT" => Integer(instance_flags::SPRITE_EFFECT as i64),
        "DEPTH_FIXED" => Integer(DepthType::Fixed as i64),
        "DEPTH_GRADIENT" => Integer(DepthType::Gradient as i64),
        "DEPTH_BOTTOM" => Integer(DepthType::Bottom as i64),
        "MAX_CELL_TABLE_SIZE" => Integer(sizes::MAX_CELL_TABLE_SIZE as i64),
        "SPRITE_EFFECT_WATER" => Integer(SpriteEffect::Water as i64),
        "MAX_NUM_LIGHTS" => Integer(sizes::MAX_NUM_LIGHTS as i64),
        "TBO_VISION_ENTRY_SIZE" => Integer(sizes::TBO_VISION_ENTRY_SIZE as i64),
        "TBO_VISION_BITMAP_OFFSET" => Integer(sizes::TBO_VISION_BITMAP_OFFSET as i64),
        "TBO_VISION_BUFFER_SIZE" => Integer(sizes::TBO_VISION_BUFFER_SIZE as i64),
    };

    include_shader_part!(table, handlebars, "INCLUDE_VISION", "vision.150.hbs.comp");
    include_shader_part!(table, handlebars, "INCLUDE_DIMENSIONS", "dimensions.150.hbs.comp");
    include_shader_part!(table, handlebars, "INCLUDE_SCROLL_OFFSET", "scroll_offset.150.hbs.comp");

    (handlebars, table)
}

pub fn populate_shader(handlebars: &Handlebars, table: &HashMap<&'static str, Value>, shader: &[u8]) -> String {
    let shader_str = ::std::str::from_utf8(shader)
        .expect("Failed to convert shader to utf8");

    handlebars.template_render(shader_str, table)
        .expect("Failed to render shader template")
}
