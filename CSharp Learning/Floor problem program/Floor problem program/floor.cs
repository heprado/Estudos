using System;


namespace Floor_problem_program
{
    public class floor
    {
        const int valor_calc = 100;

        public static int calcandar()
        {
            Console.WriteLine("Digite o numero do andar:");
            int andar_ask = Console.Read();
            int andar_respondido = andar_ask / valor_calc;
            Console.WriteLine("O andar é: {0}",andar_respondido);

        }


    }
}
