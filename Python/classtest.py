class Employee:
    def __init__(self, first, last, pay):
        self.fname = first
        self.lname = last
        self.pay = pay
        self.email = first + '.' + last + '@gmail.com'
        self.fullname = first + " " + last

emp1 = Employee("Bob", "Smith", 50000)
emp2 = Employee("Joeanna", "Susan", 75000) 

#print(emp2)
#print(emp1)

print(emp1.fname)
print(emp2.fname)

print(emp1.fullname)
print(emp2.fullname)

print(emp1.pay)