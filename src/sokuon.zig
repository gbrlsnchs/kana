const std = @import("std");

const KV = struct { @"0": []const u8, @"1": void };
pub const Matches = std.ComptimeStringMap(void, [_]KV{
    .{ "kk", {} },
    .{ "cc", {} },
    .{ "dd", {} },
    .{ "gg", {} },
    .{ "pp", {} },
    .{ "ss", {} },
    .{ "tt", {} },
});
