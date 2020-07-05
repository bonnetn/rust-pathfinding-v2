from cffi import FFI
from dataclasses import dataclass
from typing import List

ffibuilder = FFI()

lib = ffibuilder.dlopen("./target/release/libpathfinder_v2.so")

ffibuilder.cdef('''
typedef struct Position { 
    int x; 
    int y;
} Position;

typedef struct Segment { 
    Position start;
    Position end;
} Segment;

int32_t find_path_ffi(Position*, int32_t, Segment*, int32_t, Position*, Position*);
''')


@dataclass
class Position:
    x: int
    y: int


@dataclass
class Segment:
    start: Position
    end: Position


def find_path(obstacles: List[Segment], start: Position, end: Position) -> List[Position]:
    # INPUT:
    input_obstacles = ffibuilder.new(f'Segment[{len(obstacles)}]')
    for i in range(len(obstacles)):
        o = obstacles[i]
        input_obstacles[i].start.x = o.start.x
        input_obstacles[i].start.y = o.start.y
        input_obstacles[i].end.x = o.end.x
        input_obstacles[i].end.y = o.end.y

    input_start = ffibuilder.new('Position *')
    input_start.x = start.x
    input_start.y = start.y

    input_end = ffibuilder.new('Position *')
    input_end.x = end.x
    input_end.y = end.y

    # OUTPUT:
    BUFFER_SIZE = 256
    buffer = ffibuilder.new(f'Position[{BUFFER_SIZE}]')

    output_size = lib.find_path_ffi(buffer, BUFFER_SIZE, input_obstacles, len(obstacles), input_start, input_end)
    assert output_size >= 0
    return [Position(x=buffer[i].x, y=buffer[i].y) for i in range(output_size)]


def maze():
    return [
        Segment(
            start=Position(x=-100, y=-1),
            end=Position(x=100, y=-1),
        ),
        Segment(
            start=Position(x=11, y=100),
            end=Position(x=11, y=-100),
        ),
        Segment(
            start=Position(x=100, y=11),
            end=Position(x=-100, y=11),
        ),
        Segment(
            start=Position(x=-1, y=100),
            end=Position(x=-1, y=-100),
        ),
        Segment(
            start=Position(x=1, y=9),
            end=Position(x=1, y=-100),
        ),
        Segment(
            start=Position(x=3, y=9),
            end=Position(x=3, y=-100),
        ),
        Segment(
            start=Position(x=5, y=9),
            end=Position(x=5, y=-100),
        ),
        Segment(
            start=Position(x=7, y=9),
            end=Position(x=7, y=-100),
        ),
        Segment(
            start=Position(x=9, y=9),
            end=Position(x=9, y=-100),
        ),
        Segment(
            start=Position(x=2, y=100),
            end=Position(x=2, y=1),
        ),
        Segment(
            start=Position(x=4, y=100),
            end=Position(x=4, y=1),
        ),
        Segment(
            start=Position(x=6, y=100),
            end=Position(x=6, y=1),
        ),
        Segment(
            start=Position(x=8, y=100),
            end=Position(x=8, y=1),
        )
    ]


print(find_path(
    maze(),
    Position(
        x=0,
        y=0,
    ),
    Position(
        x=10,
        y=10,
    ),
))
