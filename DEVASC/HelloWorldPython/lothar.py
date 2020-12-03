c0 = int(input("Digit a number, it cant be negative or zero:"))

steps = 0

#Se o numero não for igual a 1 a iteração continua
while c0 != 1:
    #Se o numero for par divide ele por 2
    if c0%2 == 0:
        c0 = c0 // 2
    #se o numero for impar faz ele vezes 3 e adiciona 1    
    elif c0%2 != 0:
        c0 = 3 * c0 + 1
    #Só para contar a quantidade de iterações
    steps += 1
    print(c0)    
    
print("Steps={}".format(steps))
