myList = [1, 2, 4, 4, 1, 4, 2, 6, 2, 9]

print("The list with repetitive elements:")
print(myList)

for i in myList:
    if i in myList:
        del myList[i]
    elif i not in myList:
        myList.append[i]
        
print("The list with unique elements:")
print(myList)