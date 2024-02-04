import time
#Biblioteca utilizada para esperar algum tempo

seconds = 5

for i in range(1,seconds+1):
    print(i,"Mississippi")
    #Faz a thread que o codigo python estiver rodando esperar 1 segundo.
    time.sleep(1)
print("Ready or not, here i come")