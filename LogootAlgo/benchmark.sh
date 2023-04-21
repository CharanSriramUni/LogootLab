#!/bin/bash

cargo build --release

# Define the ranges of sentences and revisions to test
sentences_range=(10 50 100)
revisions_range=(50 100 200)
input_files=("/Users/sidsabhnani/Code/CSB/Concurrency/final-project/LogootLab/DataProcessing/text/George"
             "/Users/sidsabhnani/Code/CSB/Concurrency/final-project/LogootLab/DataProcessing/text/Succession")

# Create a CSV file to store the benchmark results
echo "Logoot Size, Article Size, Num Revisions, Num Sentences, Input File Path" > results.csv

# Iterate over the input files, the sentences range, and the revisions range
for input_file in "${input_files[@]}"
do
  for num_sentences in "${sentences_range[@]}"
  do
    for num_revisions in "${revisions_range[@]}"
    do
      Run the Rust program three times and capture the "LOGOOT SIZE" and "ARTICLE SIZE" lines from the second and third runs
      for i in {1..3}
      do
        if [ $i -eq 1 ]; then
          # Ignore the output of the first run
          target/release/logoot_algo -n $num_revisions -s $num_sentences $input_file > /dev/null
        else
          # Capture the output of the second and third runs
          result=$(target/release/logoot_algo -n $num_revisions -s $num_sentences -f $input_file)
          if [ $i -eq 2 ]; then
            logoot_size_1=$(echo "$result" | grep "LOGOOT SIZE" | awk '{print $3}')
            article_size_1=$(echo "$result" | grep "ARTICLE SIZE" | awk '{print $3}')
          else
            logoot_size_2=$(echo "$result" | grep "LOGOOT SIZE" | awk '{print $3}')
            article_size_2=$(echo "$result" | grep "ARTICLE SIZE" | awk '{print $3}')
          fi
        fi
      done

      Average the "LOGOOT SIZE" and "ARTICLE SIZE" values from the second and third runs
      logoot_size_avg=$(echo "scale=3; ($logoot_size_1 + $logoot_size_2) / 2" | bc)
      article_size_avg=$(echo "scale=3; ($article_size_1 + $article_size_2) / 2" | bc)

      # Write the results to the CSV file
      echo "$logoot_size_avg, $article_size_avg, $num_revisions, $num_sentences, $input_file" >> results.csv
    done
  done
done
