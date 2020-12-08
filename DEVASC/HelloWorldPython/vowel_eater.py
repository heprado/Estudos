# Prompt the user to enter a word
# and assign it to the userWord variable.
userWord = input("Digite uma palavra:").upper()

vowels = ["A","E","I","O","U"]
palavra = []
for letter in userWord:
    
    if letter in vowels:
        continue
    elif letter != vowels:
        palavra.append(letter)

#Join pega todos os items de uma lista e junta em uma string
print("".join(palavra))
        
    