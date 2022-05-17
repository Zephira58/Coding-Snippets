from turtle import goto
import turtle
tt=turtle.Turtle()

tt.goto(-340,300)
tt.begin_fill()
for i in range(4):
    tt.color('black','blue')
    tt.forward(20)
    tt.left(90)
tt.end_fill()
