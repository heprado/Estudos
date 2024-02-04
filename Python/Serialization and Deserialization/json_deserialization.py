import json
##No codigo abaixo estamos deserializando o arquivo json que estava salvo na pasta para um dicionario.
with open("json_serialization.json","r") as file:
    dados = json.load(file)
    usuario = dados["usuarios"]
    usuario["Victor"] = {"nome":"Victor Gindro","idade":22}

##Estamos serializando dnv o dicionario para um novo arquivo com o Victor adicionado.    
with open("json_deserialization.json","w") as file:
    json.dump(dados,file,indent=4)