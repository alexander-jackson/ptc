pyrun:
	python3 main.py

ptc:
	ptc fibonacci.py

cbuild:
	gcc -O2 main.c fibonacci.c -o main

crun:
	./main

benchmark:
	hyperfine --warmup 10 './main' 'python3 main.py'

slow_benchmark:
	hyperfine --warmup 10 'ptc fibonacci.py && gcc -O2 main.c fibonacci.c -o main && ./main' 'python3 main.py'

clean:
	rm -r main fibonacci.c fibonacci.h __pycache__/
