import keyboard

def key_pressed(letter):
    print("You pressed p")

while True:
    if keyboard.read_key() == "p":
        key_pressed("p")
        break