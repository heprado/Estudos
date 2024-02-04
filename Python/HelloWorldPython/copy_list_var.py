list1 = ["Cisco","VMware","Arista"]

print("Lista 1=",list1)

list2 = []
list2 = list1
##Fazer uma lista igual a uma variavel não vai substituir a lista, na realidade vai criar uma copia, que são consideradas a mesma
##Se vc apagar um index de uma lista ela apaga da outra tbm.


del list2[1]
print("Lista 2=",list2)

