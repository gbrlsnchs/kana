const std = @import("std");
const scfg = @import("zig-scfg");

const kana = @import("./lib.zig");

const testing = std.testing;
const heap = std.heap;
const mem = std.mem;

test "unit tests" {
    _ = @import("./helpers.zig");
    _ = @import("./Input.zig");
}

test "syllabary test" {
    var arena = heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();

    const allocator = arena.allocator();
    const data = @embedFile("./testdata/syllabary.scfg");
    const root = try scfg.parse(allocator, data);

    for (root) |syllabary| {
        const name = syllabary.params[0];
        const cfg = if (mem.eql(u8, name, "katakana"))
            kana.Config{ .start_with_katakana = true }
        else if (mem.eql(u8, name, "extended_katakana"))
            kana.Config{
                .start_with_katakana = true,
                .extended_katakana = true,
            }
        else
            kana.Config{};

        for (syllabary.blocks[0]) |block| {
            try testing.expectEqualStrings(block.params[0], try kana.transliterate(
                allocator,
                block.name,
                cfg,
            ));
        }
    }
}

test "pokemon test" {
    var arena = heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();

    const allocator = arena.allocator();
    const data = @embedFile("./testdata/pokemon.scfg");
    const root = try scfg.parse(allocator, data);

    for (root) |syllabary| {
        for (syllabary.blocks[0]) |block| {
            try testing.expectEqualStrings(block.params[0], try kana.transliterate(
                allocator,
                block.name,
                .{
                    .start_with_katakana = true,
                    .extended_katakana = true,
                    .parse_punctuation = true,
                    .special_chars = .{
                        .reset_prolongation = '^',
                        .small_vowel = '_',
                        .virt_stop = '%',
                    },
                },
            ));
        }
    }
}

test "random words test" {
    var arena = heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();

    const allocator = arena.allocator();

    const TestCase = struct {
        text: []const u8,
        config: kana.Config,
        expected: []const u8,
    };

    const tests = [_]TestCase{
        .{
            .text = "watashi",
            .config = .{},
            .expected = "わたし",
        },
        .{
            .text = "gaburieru",
            .config = .{
                .start_with_katakana = true,
            },
            .expected = "ガブリエル",
        },
        .{
            .text = "watashiha@gaburieru@desu",
            .config = .{
                .special_chars = .{
                    .kana = '@',
                },
            },
            .expected = "わたしはガブリエルです",
        },
        .{
            .text = "HELLO",
            .config = .{},
            .expected = "へLLお",
        },
        .{
            .text = "hello",
            .config = .{},
            .expected = "へllお",
        },
        .{
            .text = "#hello#",
            .config = .{
                .special_chars = .{
                    .raw_text = '#',
                },
            },
            .expected = "hello",
        },
        .{
            .text = "nihon@nihon@#Japan#nihon#@nihon@#",
            .config = .{
                .special_chars = .{
                    .kana = '@',
                    .raw_text = '#',
                },
            },
            .expected = "にほんニホンJapanにほん@nihon@",
        },
        .{
            .text = "#raw text",
            .config = .{
                .special_chars = .{
                    .raw_text = '#',
                },
            },
            .expected = "raw text",
        },
        .{
            .text = "",
            .config = .{},
            .expected = "",
        },
        .{
            .text = "'hana'",
            .config = .{},
            .expected = "'はな'",
        },
        .{
            .text = "'hana'",
            .config = .{
                .parse_punctuation = true,
            },
            .expected = "「はな」",
        },
        .{
            .text = "\"onamae\"",
            .config = .{
                .parse_punctuation = true,
            },
            .expected = "『おなまえ』",
        },
        .{
            .text = "chottomattekudasai.",
            .config = .{},
            .expected = "ちょっとまってください.",
        },
        .{
            .text = "chottomattekudasai.",
            .config = .{
                .parse_punctuation = true,
            },
            .expected = "ちょっとまってください。",
        },
        .{
            .text = "dame.",
            .config = .{
                .parse_punctuation = true,
            },
            .expected = "だめ。",
        },
        .{
            .text = "chottomatte",
            .config = .{},
            .expected = "ちょっとまって",
        },
        .{
            .text = "wwwwwww",
            .config = .{},
            .expected = "wwwwwww",
        },
        .{
            .text = "日本",
            .config = .{},
            .expected = "日本",
        },
        .{
            .text = "nihon",
            .config = .{},
            .expected = "にほん",
        },
        .{
            .text = "12jinitabemasu!",
            .config = .{},
            .expected = "12じにたべます!",
        },
        .{
            .text = "12jinitabemasu!",
            .config = .{
                .parse_punctuation = true,
            },
            .expected = "12じにたべます！",
        },
        .{
            .text = "123 GO!",
            .config = .{},
            .expected = "123 ご!",
        },
        .{
            .text = "oomen",
            .config = .{
                .start_with_katakana = true,
            },
            .expected = "オーメン",
        },
        .{
            .text = "tsyuu",
            .config = .{
                .start_with_katakana = true,
                .extended_katakana = true,
            },
            .expected = "ツュー",
        },
        .{
            .text = "suupaamario",
            .config = .{
                .start_with_katakana = true,
            },
            .expected = "スーパーマリオ",
        },
        .{
            .text = "pureisuteeshon",
            .config = .{
                .start_with_katakana = true,
            },
            .expected = "プレイステーション",
        },
        .{
            .text = "monkii dii rufi",
            .config = .{
                .start_with_katakana = true,
                .extended_katakana = true,
            },
            .expected = "モンキー ディー ルフィ",
        },
        .{
            .text = "wwwwwww",
            .config = .{
                .start_with_katakana = true,
            },
            .expected = "wwwwwww",
        },
        .{
            .text = "supagetti",
            .config = .{
                .start_with_katakana = true,
                .extended_katakana = true,
            },
            .expected = "スパゲッティ",
        },
        .{
            .text = "sandoicchi",
            .config = .{
                .start_with_katakana = true,
                .extended_katakana = true,
            },
            .expected = "サンドイッチ",
        },
        .{
            .text = "egguheddo",
            .config = .{
                .start_with_katakana = true,
                .extended_katakana = true,
            },
            .expected = "エッグヘッド",
        },
        .{
            .text = "日本",
            .config = .{
                .start_with_katakana = true,
            },
            .expected = "日本",
        },
        .{
            .text = "123 GO!",
            .config = .{
                .start_with_katakana = true,
            },
            .expected = "123 ゴ!",
        },
        .{
            .text = "raamen",
            .config = .{
                .force_prolongation = true,
            },
            .expected = "らーめん",
        },
        .{
            .text = "bacchi za rokku",
            .config = .{},
            .expected = "ばっち ざ ろっく",
        },
        .{
            .text = "a_a i_i u_u e_e o_o",
            .config = .{
                .special_chars = .{
                    .small_vowel = '_',
                },
            },
            .expected = "あぁ いぃ うぅ えぇ おぉ",
        },
        .{
            .text = "a_a",
            .config = .{
                .start_with_katakana = true,
                .special_chars = .{
                    .small_vowel = '_',
                },
            },
            .expected = "アァ",
        },
    };

    for (tests) |test_case| {
        try testing.expectEqualStrings(test_case.expected, try kana.transliterate(
            allocator,
            test_case.text,
            test_case.config,
        ));
    }
}

test "punctuation test" {
    var arena = heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();

    const allocator = arena.allocator();

    const TestCase = struct {
        text: []const u8,
        expected: []const u8,
    };

    const tests = [_]TestCase{
        .{ .text = "{", .expected = "｛" },
        .{ .text = "}", .expected = "｝" },
        .{ .text = "(", .expected = "（" },
        .{ .text = ")", .expected = "）" },
        .{ .text = "[", .expected = "［" },
        .{ .text = "]", .expected = "］" },
        .{ .text = "<", .expected = "【" },
        .{ .text = ">", .expected = "】" },
        .{ .text = ",", .expected = "、" },
        .{ .text = "=", .expected = "＝" },
        .{ .text = "...", .expected = "…" },
        .{ .text = ".", .expected = "。" },
        .{ .text = "''", .expected = "「」" },
        .{ .text = "\"\"", .expected = "『』" },
        .{ .text = ":", .expected = "：" },
        .{ .text = "!", .expected = "！" },
        .{ .text = "?", .expected = "？" },
        .{ .text = "~", .expected = "〜" },
    };

    for (tests) |test_case| {
        try testing.expectEqualStrings(
            test_case.expected,
            try kana.transliterate(allocator, test_case.text, .{ .parse_punctuation = true }),
        );
    }
}
