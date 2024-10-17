print("Bem vindo ao jogo Fizzbuzz! \nUm jogo em que printamos fizz quando o número for divisivel por 3 e buzz quando o numero for divisivel por 5, se for por ambos printamos Fizzbuzz")

quant = int(input("Digite até qual número vamos printar: "))

def fizzbuzz(quant:int):
    for i in range(1,quant+1):
        if i%3 == 0 and i%5 == 0:
            print(i,"Fizzbuzz")
        elif i%3 == 0:
            print(i,"Fizz")
        elif i%5 ==0:
            print(i,"Buzz")   
        else:
            print(i)

fizzbuzz(quant)