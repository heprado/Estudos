import xml.etree.ElementTree as ET
##Transformando o XML de um arquivo para um objeto
xml = ET.parse("xml_deserialization.xml")
##Pegando o root desse objeto
root = xml.getroot()

##Criando o dicionario
dc = {}
##Adicionado DC3 ao dicionario
dc["dc3"] = {"ipaddr":"192.168.0.3",
             "netmask":"255.255.255.0",
             "defaultroute":"192.168.0.1"}
##Adicionando DC4 ao dicionario
dc["dc4"] = {"ipaddr":"192.168.0.4",
             "netmask":"255.255.255.0",
             "defaultroute":"192.168.0.1"}
##Para cada datacenter no dicionario DC
for datacenter in dc:
    ##Crie um Subelemento no root com o nome do datacenter
    newdc = ET.SubElement(root,datacenter)
    ##Crie subelementos dentro desse novo datacenter
    ipaddr= ET.SubElement(newdc,"ipaddr")
    netmask= ET.SubElement(newdc,"netmask")
    defaultroute= ET.SubElement(newdc,"defaultroute")
    ##Adicione o texto nesses novos subelementos, baseados todos no datacenter que a iteração do for estiver.
    ipaddr.text = dc[datacenter]["ipaddr"]
    netmask.text = dc[datacenter]["netmask"]
    defaultroute.text = dc[datacenter]["defaultroute"]
    

xml.write("xml_serialization_dict.xml")




