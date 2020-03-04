ptc:
	ptc no_type_hints.py

pyrun:
	python3 main.py

cbuild:
	gcc -O2 main.c no_type_hints.c -o main

crun:
	./main

benchmark:
	hyperfine --warmup 10 './main' 'python3 main.py'

slow_benchmark:
	hyperfine --warmup 10 'ptc no_type_hints.py && gcc -O2 main.c no_type_hints.c -o main && ./main' 'python3 main.py'

clean:
	rm main no_type_hints.c no_type_hints.h
