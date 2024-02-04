user_input = int(input("Digite um numero:"))


numeros = []

for numero in range(user_input):
    
    if numero%2 != 0:
        numeros.append(":<")
    else:
        numeros.append(":>")

print(numeros)