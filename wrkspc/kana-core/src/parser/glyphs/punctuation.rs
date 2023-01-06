use phf::{phf_map as map, Map};

pub static MARKS: Map<&'static str, &'static str> = map! {
	">" => "】",
	"<" => "【",
	"]" => "］",
	"[" => "［",
	"(" => "（",
	")" => "）",
	"{" => "｛",
	"}" => "｝",
	"..." => "…",
	"," => "、",
	"=" => "＝",
	"." => "。",
	":" => "：",
	"!" => "！",
	"?" => "？",
};

pub const SINGLE_QUOTES: [&'static str; 2] = ["「", "」"];
pub const DOUBLE_QUOTES: [&'static str; 2] = ["『", "』"];
