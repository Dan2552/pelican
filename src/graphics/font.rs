use crate::platform::Bundle;

pub struct Font {
    path: String,
    size: u32
}

// PATHS = [
//     "/System/Library/Fonts",
//     "/System/Library/Fonts/Cache",
//     "/System/Library/Fonts/Supplemental"
//   ].freeze

//   TYPES = [
//     ".ttc",
//     ".ttf",
//     ".fon",
//     ""
//   ]

fn find_font(font_name: &str, bundle: &Bundle) -> String {
    // lookup = PATHS + [bundle.path_for_resource(nil)]

    // lookup.each do |path|
    //   TYPES.each do |type|
    //     potential = File.join(path, "#{font_name}#{type}")
    //     return potential if File.file?(potential)
    //   end
    // end

    // raise "Font \"#{font_name}\" not found. Searched in #{lookup}"
    "TODO".to_string()
}

// def font_for(context)
// @context_fonts ||= {}
// @context_fonts[context] ||= c__setup_font(
//   @font_path,
//   @font_size * context.render_scale
// )
// end

impl Font {
    pub fn new(font_name: String, font_size: u32, bundle: &Bundle) -> Font {
        let font_path = find_font(&font_name, bundle);

        Font {
            path: font_path,
            size: font_size
        }
    }

    // Get a drawable layer from the font for the given context.
    // pub fn layer_for(context: Context, text: &str) -> Layer {
        // font_data = font_for(context)
        // width, height = c__calculate_size(font_data, text, @font_size)
        // texture = Layer.new(context, Size.new(width, height))
        // c__create_texture(font_data, context, texture, text)
        // texture
    // }
}

// impl Drawable for Font {
//     fn layer_for(context: Context) -> Layer {

//     }
// }
