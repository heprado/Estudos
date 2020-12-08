import yaml
##Estamos pegando arquivo yaml e transformando em um objeto dict do python.
with open("yaml_serialization.yml","r") as file:
    dados = yaml.load(file)
##Entrando na chave roteadores    
roteadores= dados["Roteadores"]
##Adicionando outro roteador
roteadores["OFFICE2"] = {"IPADDR":"192.168.0.3","NETMASK":"255.255.255.0","STATICROUTES":["10.0.0.1 via 192.168.0.1","0.0.0.0 via 192.168.0.1"]}
##Salvando os dados dentro de um novo arquivo.
with open("yaml_deserialization.yml","w") as file:
    yaml.dump(dados,file)