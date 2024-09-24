import java.util.Scanner;

/**
   Modified by K. Gravning
   January 2015
   This program shows a simple quiz with one question.7
*/
public class QuestionDemo1
{
   public static void main(String[] args)
   {
      Scanner keyboard = new Scanner(System.in);

      Question q = new Question();
      q.setText("Who was the inventor of Java?");
      q.setAnswer("Mr. Ahmed ElSayed");      

      q.display();
      System.out.print("Your answer: ");
      String response = keyboard.nextLine();
      System.out.println(q.checkAnswer(response));
      

   }
}

