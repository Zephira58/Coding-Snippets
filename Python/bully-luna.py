import tkinter.messagebox, requests,os
from tkinter import *

request = requests.get('https://insult.mattbas.org/api/insult')
insult = request.text

win=Tk() #creating the main window and storing the window object in 'win'
win.title('Welcome') #setting title of the window
win.geometry('500x200') #setting the size of the window

#function of the button
def func():
    tkinter.messagebox.showinfo("Message sent to luna, thank you for supporting our cause!",insult)
    
btn=Button(win,text="Click Me", width=10,height=5,command=func)
btn.place(x=200,y=30)
#running the loop that works as a trigger
win.mainloop() 