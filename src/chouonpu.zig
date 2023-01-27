const std = @import("std");

const KV = struct { @"0": []const u8, @"1": void };
pub const Matches = std.ComptimeStringMap(void, [_]KV{
    .{ "aa", {} },
    .{ "ii", {} },
    .{ "uu", {} },
    .{ "ee", {} },
    .{ "oo", {} },
});
pub const Graph = "ー";
