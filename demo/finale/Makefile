pyrun:
	python3 main.py

ptc:
	ptc coursework.py

cbuild:
	gcc -O2 main.c coursework.c -o main

crun:
	./main

benchmark:
	hyperfine --warmup 10 './main' 'python3 main.py'

slow_benchmark:
	hyperfine --warmup 10 'ptc coursework.py && gcc -O2 main.c coursework.c -o main && ./main' 'python3 main.py'

coursework_diff:
	colordiff coursework.py fixed_coursework.py

clean:
	rm main coursework.c coursework.h
