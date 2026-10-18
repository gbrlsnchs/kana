const std = @import("std");
const unicode = std.unicode;
const testing = std.testing;

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
    const meta = std.meta;
    const in = Input.init("かな.グル");

    try testing.expect(in.len == 5);
    try testing.expect(meta.eql(in.data, unicode.Utf8View.initUnchecked("かな.グル")));
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
        try testing.expectEqualStrings(
            test_case.want,
            in.view(test_case.input) orelse unreachable,
        );
    }
}

test "Input shrink" {
    const Test = struct {
        input: struct { usize, usize },
        want: struct { []const u8, usize },
    };

    var in = Input.init("かな.グル");

    const tests = [_]Test{
        .{ .input = .{ "か".len, 1 }, .want = .{ "な.グル", 4 } },
        .{ .input = .{ "な.グ".len, 3 }, .want = .{ "ル", 1 } },
    };

    for (tests) |test_case| {
        in.shrink(test_case.input.@"0", test_case.input.@"1");
        try testing.expectEqualStrings(test_case.want.@"0", in.data.bytes);
        try testing.expect(in.len == test_case.want.@"1");
    }
}

test "Input startsWith" {
    var in = Input.init("^foo");

    try testing.expect(in.startsWith('^'));
    try testing.expect(!in.startsWith('@'));
}
