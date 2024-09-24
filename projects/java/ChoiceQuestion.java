import java.util.*;

/**
   A question with multiple choices.
*/
public class ChoiceQuestion extends Question
{
   private String[] choices;
   private int numberOfChoices;

   /**
      Constructs a choice question with no choices.
   */
   public ChoiceQuestion()
   {
      choices = new String[100];
      numberOfChoices = 0;
   }

   /**
      Adds an answer choice to this question.
      @param choice the choice to add
      @param correct true if this is the correct choice, false otherwise
   */
   public void addChoice(String choice, boolean correct)
   {
      choices[numberOfChoices] = choice;
      if (correct) 
      {
         // Convert choices.size() to string
         setAnswer(Integer.toString(numberOfChoices+1));
      }
      numberOfChoices++;
   }
   
   public void display()
   {
      // Display the question text
      super.display();
      // Display the answer choices
      for (int i = 0; i < numberOfChoices; i++)
      {
         int choiceNumber = i + 1;
         System.out.println(choiceNumber + ": " + choices[i]);     
      }
   }
}

