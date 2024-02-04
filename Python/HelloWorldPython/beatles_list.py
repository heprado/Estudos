beatles = []
print("Step 1:", beatles)
##Podemos usar extend para colocar diversos valores de uma vez, ao em vez do append
##Append só poemos passar um valor por vez
beatles.extend(["John Lennon","Paul McCartney","George Harrison"])
print("Step 2:", beatles)

#Adicionando mais 2 beatles
for i in range(2):
    carinha = input("Digite o nome de quem vc quer adicionar nos Beatles:")
    beatles.append(carinha)
    
# step 3
print("Step 3:", beatles)

del beatles[3]
del beatles[-1]
print("Step 4:", beatles)

beatles.insert(0,"Ringo Starr")
print("Step 5:", beatles)


# testing list legth
print("The Fab", len(beatles))