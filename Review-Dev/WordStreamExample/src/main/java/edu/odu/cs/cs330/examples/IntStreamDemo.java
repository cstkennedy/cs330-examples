package edu.odu.cs.cs330.examples;

import java.util.Arrays;
import java.util.List;

public class IntStreamDemo
{
    public static void main(String... args)
    {
        List<String> some_terms = Arrays.asList("Hello", "world", "with",
                                                "for", "while", "int");

        int[] term_lengths = some_terms.stream()
                           .mapToInt(s -> s.length())
                           .toArray();

        for (int v : term_lengths) {
            System.out.println(v);
        }
    }
}
