secret_number = 777

print(
"""
+================================+
| Welcome to my game, muggle!    |
| Enter an integer number        |
| and guess what number I've     |
| picked for you.                |
| So, what is the secret number? |
+================================+
""")

secret_number = int(input("Digit the secret number:"))
if secret_number == 777:
    print("Oh my god, you got it in the first try")
else:
    while secret_number != 777:
        if secret_number == 777:
            secret_number = 777
            print("Congrats, you got the right one :)")
        else:
            number = int(input("Ouch you missed, try another one: "))
        
