const std = @import("std");

const Fsm = @import("./Fsm.zig");
pub const Config = @import("./Config.zig");

const mem = std.mem;
const heap = std.heap;

pub fn transliterate(allocator: mem.Allocator, input: []const u8, cfg: Config) ![]const u8 {
    var fsm = Fsm.init(allocator, input, cfg);

    var result = try std.ArrayList(u8).initCapacity(allocator, input.len * 4);

    while (fsm.next()) |char| {
        try result.appendSlice(char);
    }

    return result.toOwnedSlice();
}
