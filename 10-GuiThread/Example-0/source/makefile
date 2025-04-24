MAINPROG=ButtonExample


.SUFFIXES: .java .class

SOURCES:=$(wildcard *.java)
#SOURCES+=$(wildcard utilities/*.java)
OBJECTS=$(SOURCES:.java=.class)
FLAGS=

all: jar

jar: $(OBJECTS)
	jar cvfe $(MAINPROG).jar $(MAINPROG) *.class 

.java.class: 
	javac $(FLAGS) $<

clean:
	rm *.class *.jar

run: jar
	java -jar $(MAINPROG).jar

