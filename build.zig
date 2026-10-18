const std = @import("std");
const mem = std.mem;

const version = "0.4.0-dev";

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const options = b.addOptions();
    if (mem.endsWith(u8, version, "-dev")) blk: {
        var rc: u8 = 0;
        const sha = b.runAllowFail(&.{ "git", "rev-parse", "HEAD" }, &rc, .inherit) catch {
            options.addOption(?[]const u8, "sha", null);
            break :blk;
        };

        options.addOption(?[]const u8, "sha", mem.trimEnd(u8, sha, "\n"));
    }
    options.addOption([]const u8, "version", version);
    const options_mod = options.createModule();

    const mod = b.addModule("kana", .{
        .root_source_file = b.path("src/root.zig"),
        .target = target,
    });

    const exe = b.addExecutable(.{
        .name = "kana",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/main.zig"),
            .target = target,
            .optimize = optimize,
            .imports = &.{
                .{ .name = "kana", .module = mod },
                .{ .name = "build_options", .module = options_mod },
            },
        }),
    });

    b.installArtifact(exe);

    const run_step = b.step("run", "Run the app");
    const run_cmd = b.addRunArtifact(exe);
    run_step.dependOn(&run_cmd.step);

    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const mod_tests = b.addTest(.{ .root_module = mod });
    const run_mod_tests = b.addRunArtifact(mod_tests);

    const exe_tests = b.addTest(.{ .root_module = exe.root_module });
    const run_exe_tests = b.addRunArtifact(exe_tests);

    const test_step = b.step("test", "Run tests");
    test_step.dependOn(&run_mod_tests.step);
    test_step.dependOn(&run_exe_tests.step);
}
