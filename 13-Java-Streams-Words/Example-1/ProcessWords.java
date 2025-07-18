import java.io.BufferedReader;
import java.io.IOException;
import java.io.StringReader;

import java.nio.file.Files;
import java.nio.file.Paths;

import java.util.Arrays;
import java.util.List;
import java.util.Objects;
import java.util.stream.Collectors;
import java.util.stream.Stream;


public class ProcessWords
{
    public static void main(String... args)
        throws IOException
    {
        final String wordFilename = args[0];

        try (BufferedReader buffer = Files.newBufferedReader(Paths.get(wordFilename))) {
            List<String> words = buffer
                .lines()
                .filter((line) -> !line.isEmpty())
                .map((line) -> line.strip())
                .filter((word) -> word.length() < 5)
                .filter((word) -> !word.contains(".") && !word.contains("+"))
                .collect(Collectors.toList());

            for (String word : words) {
                System.out.printf("|%s|%n", word);
            }
        }

    }
}
