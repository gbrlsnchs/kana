const Config = @This();

/// These are special ASCII characters that trigger extra
/// functionality.
const SpecialChars = struct {
    kana: ?u8 = null,
    raw_text: ?u8 = null,
    reset_prolongation: ?u8 = null,
    small_vowel: ?u8 = null,
    virt_stop: ?u8 = null,
};

start_with_katakana: bool = false,
extended_katakana: bool = false,
parse_punctuation: bool = false,
force_prolongation: bool = false,
special_chars: SpecialChars = .{},
