const std = @import("std");
const eql = std.mem.eql;

pub fn isnum(ch: u8) bool {
    return '0' <= ch and ch <= '9';
}

pub fn parse_num(str: []u8) i32 {
    if (str.len == 1 and '0' <= str[0] and str[0] <= '9') return str[0] - '0';
    if (str.len == 3) {
        if (eql(u8, str, "two")) return 2;
        if (eql(u8, str, "one")) return 1;
        if (eql(u8, str, "six")) return 6;
    }
    if (str.len == 4) {
        if (eql(u8, str, "five")) return 5;
        if (eql(u8, str, "four")) return 4;
        if (eql(u8, str, "nine")) return 9;
        if (eql(u8, str, "zero")) return 0;
    }
    if (str.len == 5) {
        if (eql(u8, str, "seven")) return 7;
        if (eql(u8, str, "three")) return 3;
        if (eql(u8, str, "eight")) return 8;
    }
    return -1;
}

pub fn Part1(fname: []const u8) anyerror!i32 {
    var input = try std.fs.cwd().openFile(fname, .{});
    defer input.close();
    var buf_reader = std.io.bufferedReader(input.reader());
    var in_stream = buf_reader.reader();
    var buf: [1024]u8 = undefined;
    var ret: i32 = 0;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var low_idx: u64 = 0;
        if (line.len == 0) {
            continue;
        }
        var high_idx: u64 = line.len - 1;
        var crnt_num: i32 =
            while (!isnum(line[low_idx]) or !isnum(line[high_idx]))
        {
            if (!isnum(line[low_idx])) {
                low_idx += 1;
            }
            if (!isnum(line[high_idx])) {
                high_idx -= 1;
            }
        } else (line[low_idx] - '0') * 10 + line[high_idx] - '0';
        ret += crnt_num;
    }
    return ret;
}

pub fn Part2(fname: []const u8) anyerror!i32 {
    var input = try std.fs.cwd().openFile(fname, .{});
    defer input.close();
    var buf_reader = std.io.bufferedReader(input.reader());
    var in_stream = buf_reader.reader();
    var buf: [1024]u8 = undefined;
    var ret: i32 = 0;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var lst = std.ArrayList(i32).init(std.heap.page_allocator);
        defer lst.deinit();
        var idx: u64 = 0;
        while (idx < line.len) : (idx += 1) {
            var end = idx + 1;
            while (end <= line.len) : (end += 1) {
                var parsed = parse_num(line[idx..end]);
                if (parsed >= 0) {
                    try lst.append(parsed);
                }
            }
        }
        var crnt = lst.items[0] * 10 + lst.getLast();
        ret += crnt;
    }
    return ret;
}
pub fn main() !void {
    std.debug.print("Part 1 {s} {d} \n", .{ "test", try Part1("test") });
    std.debug.print("Part 1 {s} {d} \n", .{ "input", try Part1("input") });
    std.debug.print("Part 2 {s} {d} \n", .{ "test2", try Part2("test2") });
    std.debug.print("Part 2 {s} {d} \n", .{ "input", try Part2("input") });
}
