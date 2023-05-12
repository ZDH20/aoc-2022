#!/bin/python3

import math

def distance(x1, y1, x2, y2):
    m1 = (x2 - x1) ** 2
    m2 = (y2 - y1) ** 2
    return math.sqrt(m1 + m2)


def move(head, tail, direction):
    global tail_history, head_history, visited

    head_history.append([head[0], head[1]])
    tail_history.append([tail[0], tail[1]])

    if direction == 'R':
        head[0] += 1
    elif direction == 'L':
        head[0] -= 1
    elif direction == 'U':
        head[1] += 1
    else:
        head[1] -= 1

    dist = distance(head[0], head[1], tail[0], tail[1])

    if dist >= 2:
        tail[0], tail[1] = head_history[-1]

    if (tail[0], tail[1]) not in visited:
        visited.add((tail[0], tail[1]))


file = open("input.txt")
data = file.read()

tail = [0, 0]
head = [0, 0]

tail_history = []
head_history = []

visited = set()

for entry in data.split('\n'):
    try:
        direction, steps = entry.split()
    except:
        break
    for _ in range(0, int(steps)):
        move(head, tail, direction)

print(len(visited))

