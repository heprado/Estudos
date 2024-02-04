word = input("Digite uma palavra:")

lenght= len(word)
#Lista onde guardamos as letras ao contrario
palavra = []
for letter in range(0,lenght):
    
    #Fazemos o append das ultimas letras da palavra, depois da penultima e assim sucessivamente
    #PRIMEIRA ITERAÇÃO
    # Palavra Henrique tem 8 letras então lenght = 8 - letter=0 - 1 = 7 word[7] = ultima letra da palavra
    #SEGUNDA ITERAÇÂO
    #Palavra Henrique tem 8 letras então lenght = 8 -letter=1 -1 = 6 word[6] = penultima letra da palavra.
    #Assim até o final do for teremos colocados todas as letras na ordem contraria na lista
    palavra.append(word[lenght-letter-1])

print("".join(palavra))