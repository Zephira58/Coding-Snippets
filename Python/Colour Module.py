from colorama import Fore, Back, Style

if keyboard.read_key() == "z":
    cls()
    print(Fore.RED +"Hold R for the colour Red")
    print(Fore.BLUE + "Hold B for the colour Blue")
    print(Fore.GREEN + "Hold G for the colour Green")
    print(Fore.YELLOW + "Hold Y for the colour Yellow")
    print(Fore.WHITE)
    time.sleep(1)
    if keyboard.read_key() == "r":
        cls()
        print(Fore.RED + "You have chosen the colour red!")
        time.sleep(5)
        cls()
        intro()
    elif keyboard.read_key() == "y":
        cls()
        print(Fore.YELLOW + "You have chosen the colour Yellow!")
        time.sleep(5)
        cls()
        intro()
    elif keyboard.read_key() == "b":
        cls()
        print(Fore.BLUE + "You have chosen the colour blue!")
        time.sleep(5)
        cls()
        intro()
    elif keyboard.read_key() == "g":
        cls()
        print(Fore.GREEN + "You have chosen the colour Green!")
        time.sleep(5)
        cls()
        intro()
    else:
        intro()