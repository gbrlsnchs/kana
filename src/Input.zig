const std = @import("std");
const unicode = std.unicode;

const Input = @This();

data: unicode.Utf8View,
len: usize,

pub fn init(text: []const u8) Input {
    return .{
        .data = unicode.Utf8View.initUnchecked(text),
        .len = unicode.utf8CountCodepoints(text) catch 0,
    };
}

pub fn view(in: *Input, size: usize) ?[]const u8 {
    if (size == 0) {
        return "";
    }

    var iter = in.data.iterator();
    const cp_view = iter.peek(size);

    return if (cp_view.len > 0) cp_view else null;
}

pub fn shrink(in: *Input, byte_size: usize, codepoint_size: ?usize) void {
    in.data.bytes = in.data.bytes[byte_size..];
    in.len -= codepoint_size orelse byte_size;
}

pub fn startsWith(in: *Input, c: u8) bool {
    return in.data.bytes.len > 0 and in.data.bytes[0] == c;
}

test "Input init" {
    var in = Input.init("かな.グル");

    try std.testing.expect(in.len == 5);
}

test "Input view" {
    const Test = struct {
        input: usize,
        want: []const u8,
    };

    var in = Input.init("かな.グル");

    const tests = [_]Test{
        .{ .input = 1, .want = "か" },
        .{ .input = 2, .want = "かな" },
        .{ .input = 3, .want = "かな." },
        .{ .input = 4, .want = "かな.グ" },
        .{ .input = 5, .want = "かな.グル" },
        .{ .input = 9999, .want = "かな.グル" },
    };

    for (tests) |test_case| {
        try std.testing.expectEqualStrings(test_case.want, in.view(test_case.input) orelse unreachable);
    }
}

test "Input shrink" {
    const Test = struct {
        input: struct { @"0": usize, @"1": usize },
        want: struct { @"0": []const u8, @"1": usize },
    };

    var in = Input.init("かな.グル");

    const tests = [_]Test{
        .{ .input = .{ .@"0" = "か".len, .@"1" = 1 }, .want = .{ .@"0" = "な.グル", .@"1" = 4 } },
        .{ .input = .{ .@"0" = "な.グ".len, .@"1" = 3 }, .want = .{ .@"0" = "ル", .@"1" = 1 } },
    };

    for (tests) |test_case| {
        in.shrink(test_case.input.@"0", test_case.input.@"1");
        try std.testing.expectEqualStrings(test_case.want.@"0", in.data.bytes);
        try std.testing.expect(in.len == test_case.want.@"1");
    }
}

test "Input startsWith" {
    var in = Input.init("^foo");

    try std.testing.expect(in.startsWith('^'));
    try std.testing.expect(!in.startsWith('@'));
}
