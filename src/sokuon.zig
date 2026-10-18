const std = @import("std");

const KV = struct { []const u8 };
pub const matches = std.StaticStringMap(void).initComptime([_]KV{
    .{"kk"},
    .{"cc"},
    .{"dd"},
    .{"gg"},
    .{"pp"},
    .{"ss"},
    .{"tt"},
});
