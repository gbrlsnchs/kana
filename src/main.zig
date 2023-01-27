const std = @import("std");
const compdata = @import("compdata");
const clap = @import("zig-clap");

const kana = @import("./lib.zig");
const Fsm = @import("./Fsm.zig");
const Config = @import("./Config.zig");

const mem = std.mem;
const io = std.io;
const heap = std.heap;
const os = std.os;
const ascii = std.ascii;
const File = std.fs.File;

const buf_size = 4096;
const version = compdata.version;

const params = clap.parseParamsComptime(
    \\-k, --katakana                   Start parsing with katakana instead of hiragana
    \\-e, --extended                   Use extended version of katakana
    \\-p, --punctuation                Parse punctuation marks
    \\-P, --force-prolongation         Force hiragana to use a prolongation character
    \\-t, --kana-toggle <CHAR>         Use an ASCII character to toggle between kanas
    \\-r, --raw-toggle <CHAR>          Use an ASCII character to toggle between raw text and kanas
    \\-R, --prolongation-reset <CHAR>  Use an ASCII character to reset a prolongation
    \\-s, --vowel-shortener <CHAR>     Use an ASCII character to insert a small vowel
    \\-S, --virtual-stop <CHAR>        Use an ASCII character to insert a virtual glottal stop
    \\-h, --help                       Display this help and exit
    \\-v, --version                    Show version and exit
);

pub fn main() !void {
    var diag = clap.Diagnostic{};

    start(&diag) catch |err| {
        const stderr = io.getStdErr().writer();

        switch (err) {
            error.InvalidAsciiChar => {
                try diag.report(stderr, err);
                try stderr.print("\n", .{});
                try printHelp(stderr);
            },
            else => {
                try stderr.print("error: {s}\n", .{@errorName(err)});
            },
        }

        os.exit(1);
    };
}

fn start(diag: *clap.Diagnostic) !void {
    const stdout = io.getStdOut();
    const stdin = io.getStdIn().reader();
    var buffered_stdin = io.bufferedReader(stdin);

    const parsers = comptime .{ .CHAR = clap.parsers.string };

    var app = try clap.parse(clap.Help, &params, parsers, .{ .diagnostic = diag });
    defer app.deinit();

    var arena = heap.ArenaAllocator.init(heap.page_allocator);
    defer arena.deinit();

    return run(arena.allocator(), stdout.writer(), buffered_stdin.reader(), app);
}

fn run(
    allocator: mem.Allocator,
    stdout: anytype,
    stdin: anytype,
    app: anytype,
) !void {
    if (app.args.help) {
        return printHelp(stdout);
    }

    if (app.args.version) {
        return stdout.print("kana version {s}\n", .{version});
    }

    const args = app.args;
    const cfg = kana.Config{
        .start_with_katakana = args.katakana,
        .extended_katakana = args.extended,
        .parse_punctuation = args.punctuation,
        .force_prolongation = args.@"force-prolongation",
        .special_chars = .{
            .kana = try parseChar(args, "kana-toggle"),
            .raw_text = try parseChar(args, "raw-toggle"),
            .reset_prolongation = try parseChar(args, "prolongation-reset"),
            .small_vowel = try parseChar(args, "vowel-shortener"),
            .virt_stop = try parseChar(args, "virtual-stop"),
        },
    };

    var buf: [buf_size]u8 = undefined;
    while (try stdin.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        const result = try kana.transliterate(allocator, line, cfg);

        if (result.len > 0) {
            try stdout.print("{s}\n", .{result});
        }
    }
}

fn printHelp(output: anytype) !void {
    try output.print(
        \\Usage: kana [OPTIONS...] < input
        \\
        \\Options:
        \\
    , .{});
    try clap.help(output, clap.Help, &params, .{
        .description_on_new_line = false,
        .description_indent = 0,
        .indent = 2,
        .spacing_between_parameters = 0,
    });
}

fn parseChar(args: anytype, comptime field: []const u8) !?u8 {
    const option = @field(args, field);

    if (option) |str| {
        if (str.len != 1) {
            return error.InvalidAsciiChar;
        }

        const char = str[0];
        if (!ascii.isASCII(char)) {
            return error.InvalidAsciiChar;
        }

        return char;
    }
    return null;
}
