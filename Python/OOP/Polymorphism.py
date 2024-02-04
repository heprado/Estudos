class Pai():
    ##Criando metodo construtor da classe pai
    def __init__(self):
        print("OLA EU SOU O METODO CONSTRUTOR DA CLASSE PAI")
    
class Filha(Pai):
    ##Fazendo um metodo construtor com o mesmo nome do da classe Pai, isso se chama method overriding
    def __init__(self):
        print("""OLA EU SOU O METODO CONSTRUTORA DA SUB CLASSE FILHA, 
              EU ESTOU FAZENDO METHOD OVERRIDING DO METODO CONSTRUTOR DA CLASSE PAI""")
        
        
pai_obj = Pai()
filha_obj = Filha()