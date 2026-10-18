const std = @import("std");
const mem = std.mem;
const unicode = std.unicode;
const ascii = std.ascii;

const chouonpu = @import("chouonpu.zig");
const Config = @import("Config.zig");
const hiragana = @import("hiragana.zig");
const katakana = @import("katakana.zig");
const punctuation = @import("punctuation.zig");
const sokuon = @import("sokuon.zig");

const Transliterator = @This();

const State = union(enum) {
    start,
    hiragana: usize,
    katakana: usize,
    chouonpu,
    sokuon,
    prolongation_reset,
    punctuation_marks: usize,
    punctuation_quotes,
    toggle_kana,
    toggle_raw,
    small_vowel,
    virtual_stop,
    raw,
};

const Toggles = struct {
    katakana: bool = false,
    raw_text: bool = false,
};

state: State = .start,
text: unicode.Utf8View,
toggles: Toggles = .{},
single_quotes_index: u1 = 0,
double_quotes_index: u1 = 0,
config: Config,

pub fn init(text: []const u8, config: Config) !Transliterator {
    return .{
        .config = config,
        .text = try .init(text),
        .toggles = .{ .katakana = config.start_with_katakana },
    };
}

pub fn run(trans: *Transliterator, allocator: mem.Allocator) ![]const u8 {
    var result = try std.ArrayList(u8).initCapacity(allocator, trans.text.bytes.len * 4);

    while (trans.text.bytes.len > 0) {
        try result.appendSlice(allocator, trans.transition());
    }

    return try result.toOwnedSlice(allocator);
}

fn transition(trans: *Transliterator) []const u8 {
    delta: switch (trans.state) {
        .start => {
            trans.state = if (trans.toggles.raw_text)
                .raw
            else if (trans.toggles.katakana)
                .{ .katakana = katakana.max_romaji_size }
            else
                .{ .hiragana = hiragana.max_romaji_size };
        },
        .hiragana => |n| {
            blk: {
                blk2: {
                    if (n < hiragana.min_romaji_size or n > hiragana.max_romaji_size) break :blk;

                    const input = trans.parse(n);
                    if (input.len != n) break :blk2;

                    var buf: [hiragana.max_romaji_size]u8 = undefined;
                    const query = ascii.lowerString(&buf, input);
                    const value = hiragana.syllabary.get(query);

                    if (value) |s| {
                        trans.eat(n - 1);
                        trans.state = .chouonpu;

                        return s;
                    }
                }

                trans.state = .{ .hiragana = n - 1 };
                break :delta;
            }

            trans.state = .sokuon;
        },
        .katakana => |n| {
            blk: {
                blk2: {
                    if (n < katakana.min_romaji_size or n > katakana.max_romaji_size) break :blk;

                    const input = trans.parse(n);
                    if (input.len != n) break :blk2;

                    var buf: [katakana.max_romaji_size]u8 = undefined;
                    const query = ascii.lowerString(&buf, input);
                    const value =
                        if (trans.config.extended_katakana)
                            katakana.extended_syllabary.get(query) orelse
                                katakana.syllabary.get(query)
                        else
                            katakana.syllabary.get(query);

                    if (value) |s| {
                        trans.eat(n - 1);
                        trans.state = .chouonpu;
                        return s;
                    }
                }

                trans.state = .{ .katakana = n - 1 };
                break :delta;
            }

            trans.state = .sokuon;
        },
        .chouonpu => {
            blk: {
                const input = trans.parse(2);
                if (input.len != 2) break :blk;

                var buf: [2]u8 = undefined;
                const query = ascii.lowerString(&buf, input);

                if ((trans.toggles.katakana or trans.config.force_prolongation) and
                    chouonpu.matches.has(query))
                {
                    trans.eat(2);
                    trans.state = .start;
                    return chouonpu.graph;
                }
            }

            trans.state = .prolongation_reset;
        },
        .prolongation_reset => {
            var n: usize = 1; // if not a match, eats only the already matched char

            blk: {
                const c = trans.config.special_chars.reset_prolongation orelse break :blk;
                const input = trans.parse(2);
                if (input.len != 2 or input[1] != c) break :blk; // enforce ASCII

                n = 2; // eats away the prolongation reset char
            }

            trans.eat(n);
            trans.state = .start;
        },
        .sokuon => {
            blk: {
                const input = trans.parse(2);
                if (input.len != 2) break :blk;

                var buf: [2]u8 = undefined;
                const query = ascii.lowerString(&buf, input);

                if (sokuon.matches.has(query)) {
                    trans.eat(1);
                    trans.state = .start;
                    return if (trans.toggles.katakana)
                        katakana.sokuon
                    else
                        hiragana.sokuon;
                }
            }

            trans.state = .{ .punctuation_marks = punctuation.max_romaji_size };
        },
        .punctuation_marks => |n| {
            if (!trans.config.parse_punctuation) {
                trans.state = .toggle_kana;
                break :delta;
            }

            blk: {
                blk2: {
                    if (n < punctuation.min_romaji_size or n > punctuation.max_romaji_size) break :blk;

                    const input = trans.parse(n);
                    if (input.len != n) break :blk2;

                    if (punctuation.marks.get(input)) |s| {
                        trans.eat(n);
                        trans.state = .start;
                        return s;
                    }
                }

                trans.state = .{ .punctuation_marks = n - 1 };
                break :delta;
            }

            trans.state = .punctuation_quotes;
        },
        .punctuation_quotes => {
            blk: {
                const input = trans.parse(1);
                if (input.len != 1) break :blk;

                if (input[0] == '\'') {
                    trans.eat(input.len);
                    trans.state = .start;

                    const i = trans.single_quotes_index;
                    trans.single_quotes_index = 1 - i;

                    return punctuation.single_quotes[i];
                }

                if (input[0] == '"') {
                    trans.eat(1);
                    trans.state = .start;

                    const i = trans.double_quotes_index;
                    trans.double_quotes_index = 1 - i;

                    return punctuation.double_quotes[i];
                }
            }

            trans.state = .toggle_kana;
        },
        .toggle_kana => {
            blk: {
                const c = trans.config.special_chars.kana orelse break :blk;
                const input = trans.parse(1);
                if (input.len != 1 or input[0] != c) break :blk; // enforce ASCII

                trans.eat(1);
                trans.state = .start;
                trans.toggles.katakana = !trans.toggles.katakana;
                break :delta;
            }

            trans.state = .toggle_raw;
        },
        .toggle_raw => {
            blk: {
                const c = trans.config.special_chars.raw_text orelse break :blk;
                const input = trans.parse(1);
                if (input.len != 1 or input[0] != c) break :blk; // enforce ASCII

                trans.eat(1);
                trans.state = .start;
                trans.toggles.raw_text = !trans.toggles.raw_text;
                break :delta;
            }

            trans.state = .small_vowel;
        },
        .small_vowel => {
            blk: {
                const c = trans.config.special_chars.small_vowel orelse break :blk;
                const input = trans.parse(2);
                if (input.len != 2 or input[0] != c) break :blk;

                trans.eat(1); // consume the special char first
                trans.state = .start; // we go back to start regardless of small vowel match

                var buf: [1]u8 = undefined;
                const query = ascii.lowerString(&buf, input[1..]);

                const source = if (trans.toggles.katakana)
                    katakana.small_vowels
                else
                    hiragana.small_vowels;

                if (source.get(query)) |s| {
                    trans.state = .chouonpu;
                    return s;
                }
            }

            trans.state = .virtual_stop;
        },
        .virtual_stop => {
            blk: {
                const c = trans.config.special_chars.virt_stop orelse break :blk;
                const input = trans.parse(1);
                if (input.len != 1 or input[0] != c) break :blk;

                trans.eat(1);
                trans.state = .start;

                return if (trans.toggles.katakana)
                    katakana.sokuon
                else
                    hiragana.sokuon;
            }

            trans.state = .raw;
        },
        .raw => {
            trans.state = .start;

            const input = trans.parse(1);
            if (input[0] == trans.config.special_chars.raw_text) {
                trans.state = .toggle_raw;
                return "";
            }

            trans.eat(input.len);
            return input;
        },
    }

    return "";
}

fn eat(trans: *Transliterator, n: usize) void {
    trans.text.bytes = trans.text.bytes[n..];
}

fn parse(trans: *Transliterator, n: usize) []const u8 {
    var it = trans.text.iterator();
    return it.peek(n);
}
