"""
Reformat to 1920x1080.

Scale up the image until the patches are just
touching the edges of the frame.

Blur a bit to average.


for i in nuke.selectedNodes():
    i['raw'].setValue(True)
"""

# For 1920x1080 only obviously
COLORCHECKER_COORDS = [
    [150, 950],
    [475, 950],
    [800, 950],
    [1125, 950],
    [1450, 950],
    [1784, 950],

    [150, 650],
    [475, 650],
    [800, 650],
    [1125, 650],
    [1450, 650],
    [1784, 650],

    [150, 390],
    [475, 390],
    [800, 390],
    [1125, 390],
    [1450, 390],
    [1784, 390],

    [150, 120],
    [475, 120],
    [800, 120],
    [1125, 120],
    [1450, 120],
    [1784, 120],
]

def get_triplet(node, pos_x, pos_y):
    triplet = []
    triplet.append(node.sample("red", pos_x, pos_y))
    triplet.append(node.sample("green", pos_x, pos_y))
    triplet.append(node.sample("blue", pos_x, pos_y))
    return triplet

def sample_colorchecker(node, coords):
    values = []
    for i in COLORCHECKER_COORDS:
        values.append(get_triplet(node, i[0], i[1]))
    return values

def txt_print(values):
    for i in values:
        print(f"{i[0]} {i[1]} {i[2]}")

def main():
    node = nuke.selectedNodes()[0]
    dataset = sample_colorchecker(node, COLORCHECKER_COORDS)
    txt_print(dataset)
    print(dataset)

main()