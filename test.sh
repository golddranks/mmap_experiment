#!/bin/sh

> test.txt

for j in $(seq 1 1); do
	for i in $(seq 1 63); do
		printf "abcdefghijklmnopqrstuvwxyz123456abcdefghijklmnopqrstuvwxyz12345 " >> test.txt
	done
	printf "abcdefghijklmnopqrstuvwxyz123456abcdefghijklmnopqrstuvwxyz12345\n" >> test.txt
done

cargo build --release

target/release/mmap_experiment&
sleep 1
echo "before:"
cat test.txt
printf "A" | dd of=test.txt bs=1 conv=notrunc 2> /dev/null
printf "B" | dd of=test.txt bs=1 conv=notrunc seek=1 2> /dev/null
printf "A" | dd of=test.txt bs=1 conv=notrunc seek=32 2> /dev/null
printf "A" | dd of=test.txt bs=1 conv=notrunc seek=64 2> /dev/null
echo "after:"
cat test.txt
sleep 2
echo "test finished."
