##O codigo abaixo faz 2 elevado a 0 até 10 e coloca dentro da lista
lista_normal = []

for i in range(0,9):
    lista_normal.append(2**i)
print(lista_normal)    
##Podemos fazer isso dentro da lista direto da seguinte forma

lista_comprehensive = [2 ** i for i in range(0,9)]

print(lista_comprehensive)



## EXEMPLOS


## Fazendo numeros elevados a 2
squares = [i ** 2 for i in range(0,9)]
##Pegando somente os pares da lista squares
pares = [i for i in squares if i%2 == 0]
#pegando somente os impares da lista squares
impares = [i for i in squares if i%2 != 0]

print("Os numeros pares são:{}".format(pares),"\nOs numero impares são:{}".format(impares))


