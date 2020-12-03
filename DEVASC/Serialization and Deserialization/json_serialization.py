import json


dicionario = {
    "usuarios": {
        "Henrique":{
            "nome":"Henrique Prado",
            "idade":22
        },
        "Lara":{
            "nome":"Lara Ayumi Nagamatsu",
            "idade":22
        }
        
    }
}

## Cria um arquivo json_serialization.json e grava o DICIONARIO em um arquivo .json
with open("json_serialization.json", "w") as file:
    json.dump(dicionario,file, indent=4)

## No codigo abaixo estou pegando o dicionario e transformando em string, o s do dumps é de string.  
## o indent faz com que a indentação do json fique mais limpa e facil de ler.
string = json.dumps(dicionario,indent=4)

print(string)



          