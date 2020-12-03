number = input("Digite um numero:")

number_replace = ""
for i in number:
    
    if i == "0":
        number_replace += "X"
    else:
        number_replace += i
        
print(number_replace)    
    