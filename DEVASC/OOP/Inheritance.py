class Pai():
    ##Metodo construtor da classe Pai
    def __init__(self):
        self.cor = "Vermelho"
        self.tamanho = "2CM"
##Criando a classe Filha derivando da classe Pai        
class Filha(Pai):
    
    def __init__(self):
        ##Herdando os parametros da classe construtora do Pai
        super().__init__()
        ##Subscrevendo o parametro cor da classe construtora do Pai para Azul.
        self.cor = "Azul"
        print(self.cor)
        print(self.tamanho)
        
##Instanciando a classe filha        
filha_obj = Filha()

