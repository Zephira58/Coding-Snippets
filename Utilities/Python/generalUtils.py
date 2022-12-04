import time, sys
import os
import time

def clearTerminal():
    os.system('cls||clear')
    
def credits(projectName: str):
    clearTerminal()
    print("Made by Xanthus In my spare time\n")
    print("Check out my other works at https://github.com/Xanthus58\n")
    print("Email me on 'Xanthus58@protonmail.com'\n")
    print("Feel free to fork; submit issues; or otherwise interact with the project here!\n")
    print("https://github.com/Xanthus58/"+projectName+"\n")
    input("Press Enter to continue \n")
    clearTerminal()

def typeWrite(str):
    for letter in str:
        sys.stdout.write(letter)
        sys.stdout.flush()
        time.sleep(0.1)

def terminalGen():
    cls()
    print('Generating')
    t5()
    cls()
    print('Generating.')
    t5()
    cls()
    print('Generating..')
    t5()
    cls()
    print('Generating...')
    cls()
