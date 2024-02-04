blocks = int(input("Enter the number of blocks: "))


##Na base vamos retirar 1 depois na camada de cima 2 e assim por diante
blocosdentro = 1
##Aumenta a cada iteração, até não sobrar mais blocos
height = 0


while blocosdentro <= blocks:
   ##Remove gradativamente os blocos
   blocks -= blocosdentro
   ##Aumenta 1 na altura a cada iteração
   height += 1
   ##Aumenta a quantidade de blocos a ser retirado, 1 depois 2 depois 3 montando a piramide.
   blocosdentro += 1
    
    

print("The height of the pyramid:", height)