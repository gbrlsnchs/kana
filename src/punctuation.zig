const std = @import("std");

const KV = struct { @"0": []const u8, @"1": []const u8 };
pub const Marks = std.ComptimeStringMap([]const u8, [_]KV{
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
