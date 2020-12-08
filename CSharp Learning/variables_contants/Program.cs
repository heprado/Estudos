using System;

namespace variables_contants
{
    class Program
    {
        static void Main(string[] args)
        {
           byte numero = 1;
           int numero_inteiro = numero;
//um byte é 8 bits, int é 32 então o compilador sabe que podemos transformar a variavel sem 
//comprometela//
            int numeroByte = 1000;
            byte numeroInt = (byte) numeroByte;
//Nesse caso vai compilar também pq eu usei o cast "(byte)" ele força o valor a ser transformado, o problema aqui é que perderemos
//dados pois o int é 32 bits e o byte somente 8, bytes vão somente até 255 e o int que estamos transformando tem o valor 1000 dentro//
            var mil = "1234";
            int milParaInt = Convert.ToInt32(mil);
//Nesse caso estamos convertendo uma string para uma int utilizando metodo toInt32 da Classe Convert do dotnet, assim conseguimos
// transformar uma string em numeros//
            try
            {
            string milhoes = "123456789";
            byte milhoesParaByte = Convert.ToByte(milhoes);
            Console.WriteLine("Numero transformado de byte para int{0},numero transformado de int para byte(vai da pau) {1}, crashando a porra toda transformando uma string num byte {2}",numero_inteiro,numeroInt,milhoesParaByte);
//Nesse caso o nosso programa vai simplismente CRASHAR TUDO, pois estamos pegando um valor 123456789 e colocando num byte que tem
//somente 8 bits e consegue guardar somente até 255 decimal, na hora de compilar a funcao Convert.ToByte() nao consegue tratar
//overflow então SÓ CRASHA TUDO//
            }
            catch (System.OverflowException)
            {
    
            }
            
            Console.WriteLine("Numero transformado de byte para int RESULTADO = {0} , numero transformado de int que vale 1000 para byte RESULTADO = {1} ,convertendo uma variavel para um int (funciona ok se o valor da string for até 2 bilhoes) RESULTADO = {2} ,já o outro transformando string pra byte não vai funcionar né, eu fiz o handle da exception la em cima",numero_inteiro,numeroInt,milParaInt);
            
        }
    }
}