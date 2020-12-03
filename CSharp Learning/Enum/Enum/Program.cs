using System;

namespace Enum
{
    /*Enums são conjuntos de constantes que contem numeros inteiros*/
    public enum Pessoas
    {
        Henrique = 1,
        Lara = 2,
        Igor = 3
    }
    class Program
    {
        static void Main(string[] args)
        {/*Para chamar um enum é assim que fazemos*/
            Console.WriteLine(Pessoas.Henrique);
            /*Para chamar o inteiro do enum precisamos fazer um cast*/
            Console.WriteLine((int)Pessoas.Lara);
        }
    }
}

