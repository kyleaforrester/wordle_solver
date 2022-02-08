This is a command line program to solve wordle puzzles.

Usage: ./wordle_solver words.txt rules.txt

words.txt:
    This parameter is the location of the dictionary.  On most linux systems the file can be found at a path similar to /usr/share/dict/words.  This is the word bank the solver will use to come up with guesses.

rules.txt:
    This parameter is the location of a text file that contains the hints and information used from previous guesses.  There are three types of hints that Wordle gives that the text file must be able to store:

    1) Correct letters: Whenever a letter is 'Green', in that its position was guessed correctly, we can add a line to rules.txt to tell wordle_solver of this information like so:

    a at 2

    Where the first parameter is the lower-case letter of the guess and the second is the index (starting at 0) of the correct guess.

    2) Mispositioned letters: Whenever a letter is 'Yellow', in that its existence was guessed correctly but the position is incorrect, we can add a line to rules.txt to tell wordle_solver of this information like so:

    r not 0

    Where the first parameter is the lower-case letter of the guess and the second is the index (starting at 0) of the mispositioned guess.

    3) Wrong letters: Whenever a guessed letter is 'Gray', in that it does not appear at all in the word, we can add a line to rules.txt to tell wordle_solver of this information like so:

    no x

    Where the only parameter is a comma-separated list of letters that do not exist.  Instead of having a new line of the 'not' rule for each Gray letter, you may concatenate all Gray letters into a comma separated value in the same rule like so:

    no x,z,q,h



Here is an example rules.txt text file describing a wordle puzzle:

a not 0
r at 1
e at 4
not o,s
