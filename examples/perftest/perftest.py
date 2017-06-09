px8 / python cartridge
version 1
__python__

import time


first_time = True


def small_rect():
    rect(64, 64, 74, 74)


def small_rectfill():
    rectfill(64, 64, 74, 74)


def large_rect():
    rect(0, 0, 127, 127)


def large_rectfill():
    rectfill(0, 0, 127, 127)


def text():
    px8_print("ABCDEFGHIJKLMNOPQRSTUVWXYZ", 0, 64, 7)


def line_test():
    line(0, 0, 127, 127, 7)


def circ_test():
    circ(63, 63, 40, 7)


def circfill_test():
    circfill(63, 63, 40, 7)


def perftest(name, f, iterations):
    start = time.time()
    for i in range(iterations):
        f()
    end = time.time()
    elapsed_time = end - start
    time_per_op = elapsed_time * 1000000 / iterations
    print("%-20s%8d%20.3f%20.3f" %
          (name, iterations, elapsed_time, time_per_op))


tests = [
    #("cls", cls, 1000000),
    ("cls", cls, 1000000),
    ("small_rect", small_rect, 1000000),
    ("small_rectfill", small_rectfill, 1000000),
    ("large_rect", large_rect, 100000),
    ("large_rectfill", large_rectfill, 10000),
    ("text", text, 100000),
    ("line", line_test, 100000),
    ("circ", circ_test, 100000),
    ("circfill", circfill_test, 10000)
]


def _init():
    pass


def _update():
    pass


def _draw():
    global first_time
    if first_time:
        cls()
        print("%-20s%-20s%-20s%-20s" %
              ("test", "iterations", "total_time(secs)", "time_per_op(us)"))

        for name, func, iterations in tests:
            perftest(name, func, iterations)
            first_time = False
