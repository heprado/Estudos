ovelhas = int(input("Digite a quantidade de ovelhas: "))


for i in range(1,ovelhas+1):
    if i >= 100:
        print("Cara tu já contou {} ovelhas vai dormir".format(i))
    elif i < 100:
        print("Só {} ovelhas dorme ainda não".format(i))