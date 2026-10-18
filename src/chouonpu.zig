const std = @import("std");

const KV = struct { []const u8 };

pub const matches = std.StaticStringMap(void).initComptime(&[_]KV{
    .{"aa"},
    .{"ii"},
    .{"uu"},
    .{"ee"},
    .{"oo"},
});

pub const graph = "ー";
