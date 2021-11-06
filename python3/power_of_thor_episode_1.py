# Auto-generated code below aims at helping you parse
# the standard input according to the problem statement.
# ---
# Hint: You can use the debug stream to print initialTX and initialTY, if Thor seems not follow your orders.

# light_x: the X position of the light of power
# light_y: the Y position of the light of power
# initial_tx: Thor's starting X position
# initial_ty: Thor's starting Y position
light_x, light_y, x, y = [int(i) for i in input().split()]

# game loop
while 1:
    remaining_turns = int(input())  # The remaining amount of turns Thor can move. Do not remove this line.

    direction = ""

    if light_y < y:
        direction += "N"
        y -= 1
    elif light_y > y:
        direction += "S"
        y += 1

    if light_x > x:
        direction += "E"
        x += 1
    elif light_x < x:
        direction += "W"
        x -= 1

    # A single line providing the move to be made: N NE E SE S SW W or NW
    print(direction)
