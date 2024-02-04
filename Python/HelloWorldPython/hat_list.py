hatList = [1, 2, 3, 4, 5]  

#Muda o numero do meio da lista para o numero inserido pelo usuario
hatList[2] = int(input("Digite um numero:"))

#Deleta o ultimo numero da list
del hatList[-1]

#Printa o tamanho da lista
print("Tamanho da lista: {}".format(len(hatList)))

print(str(hatList).strip("[""]"))