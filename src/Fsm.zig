const std = @import("std");
const heap = std.heap;
const mem = std.mem;
const unicode = std.unicode;
const ascii = std.ascii;

const Config = @import("./Config.zig");
const Input = @import("./Input.zig");
const hiragana = @import("./hiragana.zig");
const katakana = @import("./katakana.zig");
const sokuon = @import("./sokuon.zig");
const chouonpu = @import("./chouonpu.zig");
const punctuation = @import("./punctuation.zig");
const helpers = @import("./helpers.zig");

const kana = enum {
    hiragana,
    katakana,

    fn get_syllabogram(self: kana, allocator: mem.Allocator, query: []const u8, cfg: Config) ?[]const u8 {
        const lowercase_query = ascii.allocLowerString(allocator, query) catch return null;
        defer allocator.free(lowercase_query);

        return switch (self) {
            .hiragana => hiragana.Syllabary.get(lowercase_query),
            .katakana => if (cfg.extended_katakana)
                katakana.ExtendedSyllabary.get(lowercase_query) orelse katakana.Syllabary.get(lowercase_query)
            else
                katakana.Syllabary.get(lowercase_query),
        };
    }

    fn get_sokuon(self: kana, allocator: mem.Allocator, query: []const u8) ?[]const u8 {
        const lowercase_query = ascii.allocLowerString(allocator, query) catch return null;
        defer allocator.free(lowercase_query);

        if (sokuon.Matches.has(lowercase_query)) {
            return switch (self) {
                .hiragana => hiragana.sokuon,
                .katakana => katakana.sokuon,
            };
        }

        return null;
    }

    fn get_small_vowels(self: kana, allocator: mem.Allocator, query: []const u8) ?[]const u8 {
        const lowercase_query = ascii.allocLowerString(allocator, query) catch return null;
        defer allocator.free(lowercase_query);

        return switch (self) {
            .hiragana => hiragana.SmallVowels.get(lowercase_query),
            .katakana => katakana.SmallVowels.get(lowercase_query),
        };
    }

    fn get_chouonpu(self: kana, allocator: mem.Allocator, query: []const u8, cfg: Config) ?[]const u8 {
        const lowercase_query = ascii.allocLowerString(allocator, query) catch return null;
        defer allocator.free(lowercase_query);

        return switch (self) {
            .hiragana => if (cfg.force_prolongation and chouonpu.Matches.has(lowercase_query)) chouonpu.Graph else null,
            .katakana => if (chouonpu.Matches.has(lowercase_query)) chouonpu.Graph else null,
        };
    }
};

const Fsm = @This();

pub const State = union(enum) {
    init,
    raw_toggle,
    kana_toggle,
    size_router,
    long,
    medium,
    short,
    tiny,
    sokuon,
    virt_sokuon,
    small_vowel,
    chouonpu,
    raw_text: u8,
    punctuation: struct { @"0": usize, @"1": *const State },
    fallback,
};

input: Input,
state: State = .init,
kanas: helpers.Switch(kana) = .{
    .data = [2]kana{
        .hiragana,
        .katakana,
    },
},
single_quotes: helpers.Switch([]const u8) = .{
    .data = punctuation.single_quotes,
},
double_quotes: helpers.Switch([]const u8) = .{
    .data = punctuation.double_quotes,
},
config: Config,
allocator: mem.Allocator,

pub fn init(allocator: mem.Allocator, text: []const u8, config: Config) Fsm {
    var fsm = Fsm{
        .allocator = allocator,
        .input = Input.init(text),
        .config = config,
    };

    if (config.start_with_katakana) {
        fsm.kanas.toggle();
    }

    return fsm;
}

/// Returns the computed string to be written and updates the state
/// machine with new input.
pub fn next(fsm: *Fsm) ?[]const u8 {
    switch (fsm.state) {
        .init => {
            if (fsm.input.len == 0) {
                return null;
            }

            fsm.state = .raw_toggle;
            return "";
        },
        .raw_toggle => {
            const size = 1;

            if (fsm.config.special_chars.raw_text) |toggle| {
                if (fsm.input.startsWith(toggle)) {
                    fsm.input.shrink(size, null);
                    fsm.state = .{ .raw_text = toggle };

                    return "";
                }
            }

            fsm.state = .kana_toggle;
            return "";
        },
        .raw_text => |toggle| {
            const size = 1;

            if (fsm.input.startsWith(toggle)) {
                fsm.input.shrink(size, null);
                fsm.state = .raw_toggle;

                return "";
            }

            if (fsm.input.view(size)) |char| {
                fsm.input.shrink(char.len, size);

                return char;
            }
        },
        .kana_toggle => {
            const size = 1;

            if (fsm.config.special_chars.kana) |toggle| {
                if (fsm.input.startsWith(toggle)) {
                    fsm.kanas.toggle();
                    fsm.input.shrink(size, null);
                    fsm.state = .init;

                    return "";
                }
            }

            fsm.state = .size_router;

            return "";
        },
        .size_router => {
            const n = fsm.input.len;

            fsm.state = if (n >= 4) .long else switch (n) {
                3 => .medium,
                2 => .sokuon,
                1 => .tiny,
                else => .init,
            };
            return "";
        },
        .long => return fsm.goto_next(4, .medium),
        .medium => return fsm.goto_next(3, .sokuon),
        .short => return fsm.goto_next(2, .tiny),
        .tiny => return fsm.goto_next(1, .virt_sokuon),
        .sokuon => {
            const size = 2;
            const query = fsm.input.view(size).?;
            const current_kana = fsm.kanas.get();

            if (current_kana.get_sokuon(fsm.allocator, query)) |result| {
                const str = fsm.input.view(size - 1).?;

                fsm.input.shrink(str.len, size - 1);
                fsm.state = .init;

                return result;
            }

            fsm.state = .small_vowel;

            return "";
        },
        .small_vowel => {
            if (fsm.config.special_chars.small_vowel) |trigger| {
                const current_kana = fsm.kanas.get();

                if (fsm.input.startsWith(trigger)) {
                    fsm.input.shrink(1, null);

                    const query = fsm.input.view(1).?;

                    if (current_kana.get_small_vowels(fsm.allocator, query)) |result| {
                        fsm.state = .chouonpu;

                        return result;
                    }
                }
            }

            fsm.state = .short;

            return "";
        },
        .punctuation => |values| {
            const size = values.@"0";
            const next_state = values.@"1";

            defer fsm.allocator.destroy(next_state);

            if (!fsm.config.parse_punctuation) {
                _ = fsm.input.view(size);
                fsm.state = next_state.*;

                return "";
            }

            if (fsm.input.startsWith('\'')) {
                fsm.input.shrink(1, null);
                fsm.state = .init;

                defer fsm.single_quotes.toggle();

                return fsm.single_quotes.get();
            }

            if (fsm.input.startsWith('"')) {
                fsm.input.shrink(1, null);
                fsm.state = .init;

                defer fsm.double_quotes.toggle();

                return fsm.double_quotes.get();
            }

            const query = fsm.input.view(size).?;
            if (punctuation.Marks.get(query)) |result| {
                fsm.input.shrink(size, null);
                fsm.state = .init;

                return result;
            }

            fsm.state = next_state.*;

            return "";
        },
        .chouonpu => {
            const size = 2;
            const query = fsm.input.view(size).?; // oo | o^
            const base = fsm.input.view(1).?; // o

            fsm.input.shrink(base.len, 1); // o | ^

            if (fsm.config.special_chars.reset_prolongation) |reset| {
                if (fsm.input.startsWith(reset)) {
                    fsm.input.shrink(1, null); // ?
                    fsm.state = .init;

                    return "";
                }
            }

            const current_kana = fsm.kanas.get();
            if (current_kana.get_chouonpu(fsm.allocator, query, fsm.config)) |result| {
                fsm.state = .chouonpu;

                return result;
            }

            fsm.state = .init;

            return "";
        },
        .virt_sokuon => {
            if (fsm.config.special_chars.virt_stop) |stop| {
                if (fsm.input.startsWith(stop)) {
                    fsm.input.shrink(1, null);
                    fsm.state = .init;

                    return switch (fsm.kanas.get()) {
                        .hiragana => hiragana.sokuon,
                        .katakana => katakana.sokuon,
                    };
                }
            }

            fsm.state = .fallback;

            return "";
        },
        .fallback => {
            const char = fsm.input.view(1).?;

            fsm.input.shrink(char.len, 1);
            fsm.state = .init;

            return char;
        },
    }

    return null;
}

fn goto_next(fsm: *Fsm, size: usize, next_state: State) ?[]const u8 {
    const query = fsm.input.view(size).?;
    const cur_kana = fsm.kanas.get();

    if (cur_kana.get_syllabogram(fsm.allocator, query, fsm.config)) |result| {
        const str = fsm.input.view(size - 1).?;

        fsm.input.shrink(str.len, size - 1);
        fsm.state = .chouonpu;

        return result;
    }

    const next_state_ptr = fsm.allocator.create(State) catch {
        return null;
    };
    errdefer fsm.allocator.destroy(next_state_ptr);

    next_state_ptr.* = next_state;
    fsm.state = .{ .punctuation = .{ .@"0" = size, .@"1" = next_state_ptr } };

    return "";
}

test "init" {}
