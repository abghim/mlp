const std = @import("std");
const mlp = @import("mlp");


pub fn main() !void {
    var m1: mlp.Matrix = mlp.Matrix.new(3, 4);
    try m1.set(0, 0, 1);
    try m1.set(1, 1, 1);
    try m1.set(1, 2, 5);
    try m1.set(2, 3, 99);
    try m1.print();
}
