# Prompt the user to enter a word
# and assign it to the userWord variable.
userWord = input("Digite uma palavra:").upper()

palavra = ""
for letter in userWord:
    
    if letter == "A" or letter == "E" or letter == "I" or letter == "O" or letter == "U":
       continue
    elif letter != "A" or letter != "E" or letter != "I" or letter != "O" or letter != "U":
        palavra += letter
    
    

print(palavra)
        
    