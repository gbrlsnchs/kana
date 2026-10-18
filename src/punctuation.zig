const std = @import("std");

const KV = struct { []const u8, []const u8 };
pub const marks = std.StaticStringMap([]const u8).initComptime([_]KV{
    .{ ">", "】" },
    .{ "<", "【" },
    .{ "]", "］" },
    .{ "[", "［" },
    .{ "(", "（" },
    .{ ")", "）" },
    .{ "{", "｛" },
    .{ "}", "｝" },
    .{ "...", "…" },
    .{ ",", "、" },
    .{ "=", "＝" },
    .{ ".", "。" },
    .{ ":", "：" },
    .{ "!", "！" },
    .{ "?", "？" },
    .{ "~", "〜" },
});

pub const single_quotes = [_][]const u8{ "「", "」" };
pub const double_quotes = [_][]const u8{ "『", "』" };

pub const max_romaji_size = blk: {
    var n = 0;

    for (marks.keys()) |k| {
        if (k.len > n) n = k.len;
    }

    break :blk n;
};

pub const min_romaji_size = blk: {
    var n = max_romaji_size;

    for (marks.keys()) |k| {
        if (k.len < n) n = k.len;
    }

    break :blk n;
};
