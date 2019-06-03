lexer: scanner.cpp
	g++ -std=c++11 -o lexer scanner.cpp

scanner.cpp: scanner.l
	flex -o scanner.cpp scanner.l

clean:
	rm -f scanner.cpp ./lexer