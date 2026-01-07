//! By convention, root.zig is the root source file when making a library.
const std = @import("std");

pub const Matrix = struct {
    row: usize,
    col: usize,
    data: []f64,

    pub fn get(self: Matrix, r: usize, c: usize) f64 {
        std.debug.assert(0<=r && r<self.row);
        std.debug.assert(0<=c && c<self.col);
        
        return self.data[r * self.col + c];
    }

    pub fn set(self: Matrix, r: usize, c: usize, to: f64) void {
        std.debug.assert(0<=r && r<self.row);
        std.debug.assert(0<=c && c<self.col);
        
        self.data[r * self.col + c] = to;
    }

    pub fn print(self: Matrix) void {
        var buffer: [4096]u8 = undefined;
        var stdout_writer = std.fs.File.stdout().writer(&buffer);
        const out = &stdout_writer.interface;
        for (0..self.row) |ri| {
            for (0..self.col) |ci| {
                try out.print("{}\t", .{self.get(ri, ci)});
            } try out.writeByte('\n');
        }
        try out.flush();
    }
    
    pub fn add(self: Matrix, other: Matrix) Matrix {
        std.debug.assert((self.row == other.row ) && (self.col == other.col));
        var result: Matrix = self.new(self.row, self.col);
        
    }
    
    pub fn transpose(self: Matrix) Matrix {
        return self.new(1, 1);
    }
    

    pub fn new(r: usize, c: usize) Matrix {
        const alloc = std.heap.page_allocator;
        const d = try alloc.alloc(f64, r * c);
        @memset(d, 0);
        return Matrix{ .row = r, .col = c, .data = d };
    }
};

test "test matrix lib" {
    var M: Matrix = try Matrix.new(3, 3);
    try M.set(0, 0, 1);
    try M.set(1, 1, 1);
    try M.set(1, 2, 1);

    try M.print();
    try std.testing.expect(M.get(0, 0) == 1);
    try std.testing.expect(M.get(1, 2) == 1);
    try std.testing.expect(M.get(0, 1) == 0);
}
