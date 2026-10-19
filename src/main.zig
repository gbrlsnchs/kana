const std = @import("std");
const mem = std.mem;
const heap = std.heap;
const os = std.os;
const ascii = std.ascii;
const File = std.Io.File;
const build_options = @import("build_options");
const version = build_options.version;

const kana = @import("kana");
const Config = kana.Config;

const usage =
    \\Usage: kana [OPTIONS...] < input
    \\
    \\Options:
    \\-k           Start parsing with katakana instead of hiragana
    \\-x           Use extended version of katakana
    \\-pun         Parse punctuation marks
    \\-pro         Force hiragana to use a prolongation character
    \\-t=<CHAR>    Use an ASCII character to toggle between kanas
    \\-raw=<CHAR>  Use an ASCII character to toggle between raw text and kanas
    \\-res=<CHAR>  Use an ASCII character to reset a prolongation
    \\-vow=<CHAR>  Use an ASCII character to insert a small vowel
    \\-stop=<CHAR> Use an ASCII character to insert a virtual glottal stop
    \\-h           Display this help and exit
    \\-version     Show version and exit
;

pub fn main(init: std.process.Init) !void {
    const io = init.io;
    const arena = init.arena.allocator();
    const args = try init.minimal.args.toSlice(arena);

    var stdout = File.stdout().writer(io, &.{});
    const stdout_writer = &stdout.interface;

    var stdin_buffer: [4096]u8 = undefined;
    var stdin = File.stdin().reader(io, &stdin_buffer);
    const stdin_reader = &stdin.interface;

    const config = switch (parseArgs(args[1..])) {
        .help => {
            try stdout_writer.print("{s}\n", .{usage});
            try stdout_writer.flush();
            return;
        },
        .version => {
            if (build_options.sha) |sha| {
                try stdout_writer.print("kana {s} {s}\n", .{ version, sha });
            } else {
                try stdout_writer.print("kana {s}\n", .{version});
            }

            try stdout_writer.flush();
            return;
        },
        .invalid => |option| return std.debug.print(
            "Invalid option: {s}\n\n{s}\n",
            .{ option, usage },
        ),
        .valid => |c| c,
    };

    while (true) {
        const line = stdin_reader.takeDelimiterInclusive('\n') catch break;
        const result = try kana.transliterate(arena, line, config);

        if (result.len > 0) {
            try stdout_writer.print("{s}", .{result});
            try stdout_writer.flush();
        }
    }
}

const ParsedConfig = union(enum) {
    help,
    version,
    valid: Config,
    invalid: [:0]const u8,
};

fn parseArgs(args: []const [:0]const u8) ParsedConfig {
    var config: Config = .{};

    for (args) |arg| {
        if (mem.eql(u8, arg, "-h")) return .help;
        if (mem.eql(u8, arg, "-version")) return .version;

        if (mem.eql(u8, arg, "-k")) {
            config.start_with_katakana = true;
        } else if (mem.eql(u8, arg, "-x")) {
            config.extended_katakana = true;
        } else if (mem.eql(u8, arg, "-pun")) {
            config.parse_punctuation = true;
        } else if (startsWithPrefix(arg, "-t=")) |i| {
            config.special_chars.kana = parseAsciiChar(arg[i..]) catch
                return .{ .invalid = arg };
        } else if (startsWithPrefix(arg, "-raw=")) |i| {
            config.special_chars.raw_text = parseAsciiChar(arg[i..]) catch
                return .{ .invalid = arg };
        } else if (startsWithPrefix(arg, "-res=")) |i| {
            config.special_chars.reset_prolongation = parseAsciiChar(arg[i..]) catch
                return .{ .invalid = arg };
        } else if (startsWithPrefix(arg, "-vow=")) |i| {
            config.special_chars.small_vowel = parseAsciiChar(arg[i..]) catch
                return .{ .invalid = arg };
        } else if (startsWithPrefix(arg, "-stop=")) |i| {
            config.special_chars.virt_stop = parseAsciiChar(arg[i..]) catch
                return .{ .invalid = arg };
        } else {
            return .{ .invalid = arg };
        }
    }

    return .{ .valid = config };
}

fn startsWithPrefix(arg: [:0]const u8, prefix: [:0]const u8) ?usize {
    return if (mem.startsWith(u8, arg, prefix))
        prefix.len
    else
        null;
}

fn parseAsciiChar(arg: [:0]const u8) !u8 {
    if (arg.len != 1) return error.InvalidSize;

    const char = arg[0];
    if (!ascii.isAscii(char)) return error.NotAscii;

    return char;
}
