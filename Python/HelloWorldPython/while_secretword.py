secret_word = "CHUPACABRA"

secret_question = input("Write the secret word to get out of the while loop:").upper()
while True:
    if secret_word == secret_question:
        break
    else:
        secret_question = input("Nope, isn't the right one, try again:").upper()
        
print("You've successfully left the loop.")