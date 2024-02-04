email = input("Digite o email:")
##String vazia para colocar as letras
email_sem_arroba = ""

for letter in email:
    #Se achar o arroba acaba com o for    
    if letter == "@":
        break
    #Adiciona a letra na string para cada iteração.
    email_sem_arroba += letter
    
print(email_sem_arroba)