import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.stream.Collectors;

public class AdventOfCode {

  ArrayList<ArrayList<Character>> input = new ArrayList<ArrayList<Character>>();

  private void readInput(String line) {
    // first index at 1 the rest every 4
    Integer ix = 0;
    for (int i = 1; i < line.length(); i += 4) {
      char c = line.charAt(i);
      if (c == '1') {
        break;
      }
      if (input.size() <= ix) {
        input.add(new ArrayList<Character>());
      }
      if (c != ' ') {
        if (input.get(ix).size() == 0) {
          input.get(ix).add(c);
        } else {
          input.get(ix).add(0, c);
        }
      }
      ix++;
    }
  }

  private static int[] parseMoveInput(String line) {
    String[] parts = line.split(" ");
    int count = Integer.parseInt(parts[1]);
    int from = Integer.parseInt(parts[3]);
    int to = Integer.parseInt(parts[5]);
    int[] result = new int[] {count, from, to};

    return result;
  }

  private void moveCrate(int count, int from, int to) {
    // Part 1
    // for (int i = 0; i < count; i++) {
    //   char c = input.get(from - 1).remove(input.get(from - 1).size() - 1);
    //   input.get(to - 1).add(c);
    // }

    // Part 2
    for (int i = count; i > 0; i--) {
      char c = input.get(from - 1).remove(input.get(from - 1).size() - i);
      input.get(to - 1).add(c);
    }
  }

  private static char getLast(ArrayList<Character> list) {
    return list.get(list.size() - 1);
  }

  public static void main(String[] args) {
    if (args.length != 1) {
      System.out.println("Usage: AdventOfCode <input file>");
      System.exit(1);
    }

    BufferedReader reader;
    AdventOfCode AOC = new AdventOfCode();
    try {
      reader = new BufferedReader(new FileReader(args[0]));
      String line = reader.readLine();
      Boolean isInput = true;

      while (line != null) {
        if (line != null && line.length() == 0) {
          isInput = false;
          line = reader.readLine();
        }
        if (isInput) {
          AOC.readInput(line);
        } else if (line.length() > 0) {
          int[] instructions = parseMoveInput(line);
          AOC.moveCrate(instructions[0], instructions[1], instructions[2]);
        }
        line = reader.readLine();
      }
    } catch (IOException e) {
      e.printStackTrace();
      System.exit(1);
    }

    System.out.println(AOC.input.stream()
                           .map(list -> getLast(list))
                           .map(c -> c.toString())
                           .collect(Collectors.joining()));
  }
}
