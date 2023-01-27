const std = @import("std");
const zbs = std.build;

const process = std.process;
const Builder = zbs.Builder;

pub fn build(b: *Builder) void {
    const target = b.standardTargetOptions(.{});
    const mode = b.standardReleaseOptions();

    const exe = b.addExecutable("kana", "src/main.zig");

    exe.setTarget(target);
    exe.setBuildMode(mode);

    exe.strip = b.option(bool, "strip", "Omit debug information") orelse false;
    exe.pie = b.option(bool, "pie", "Build a Position Independent Executable") orelse false;

    const version = b.option([]const u8, "version", "Set program version") orelse "unknown";
    const compdata = b.addOptions();
    compdata.addOption([]const u8, "version", version);

    exe.addOptions("compdata", compdata);
    exe.addPackagePath("zig-clap", "deps/zig-clap/clap.zig");
    exe.install();

    const run = exe.run();
    if (b.args) |args| {
        run.addArgs(args);
    }
    run.step.dependOn(b.getInstallStep());

    const run_step = b.step("run", "Run program");
    run_step.dependOn(&run.step);

    const tests = b.addTest("src/tests.zig");
    tests.addPackagePath("zig-scfg", "deps/zig-scfg/scfg.zig");
    tests.setBuildMode(exe.build_mode);

    const test_step = b.step("test", "Run tests");
    test_step.dependOn(&tests.step);
}
