pub fn Switch(comptime T: type) type {
    return struct {
        const Self = @This();

        index: u1 = 0,
        data: [2]T,

        pub fn get(self: *Self) T {
            return self.data[self.index];
        }

        pub fn toggle(self: *Self) void {
            self.index +%= 1;
        }
    };
}

test "Switch" {
    const std = @import("std");
    const testing = std.testing;

    var sw = Switch(u21){ .data = [2]u21{ 'あ', 'ア' } };
    try testing.expectEqual(sw.get(), 'あ');

    sw.toggle();
    try testing.expectEqual(sw.get(), 'ア');

    sw.toggle();
    try testing.expectEqual(sw.get(), 'あ');
}
