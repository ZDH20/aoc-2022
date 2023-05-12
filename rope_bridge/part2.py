#!/bin/python3

import math

def distance(x1, y1, x2, y2):
    m1 = (x2 - x1) ** 2
    m2 = (y2 - y1) ** 2
    return math.sqrt(m1 + m2)


def update_two_points(head, tail, head_history):
    before = (tail[0], tail[1])
    dist = distance(head[0], head[1], tail[0], tail[1])
    if dist >= 2:
        tail[0] = head_history[-1][0]
        tail[1] = head_history[-1][1]
        return (True, before)
    return (False, (-1, -1))


file = open("input.txt")
data = file.read()

#         head    tails...
snake = [[0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0]]
snake_history = [[], [], [], [], [], [], [], [], [], [], []]

visited = set()

for entry in data.split('\n'):
    try:
        direction, steps = entry.split()
    except:
        break
    for _ in range(0, int(steps)):
        snake_history[0].append((snake[0][0], snake[0][1]))

        if direction == 'R':
            snake[0][0] += 1
        elif direction == 'L':
            snake[0][0] -= 1
        elif direction == 'U':
            snake[0][1] += 1
        else:
            snake[0][1] -= 1

        for i in range(0, 10):
            Visited.add((snake[1][0], snake[1][1]))
            res = update_two_points(snake[i], snake[i+1], snake_history[i])
            if res[0]:
                old_point = res[1]
                snake_history[i + 1].append((old_point[0], old_point[1]))
print(len(visited))

