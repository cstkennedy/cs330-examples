PKGPREFIX=edu.odu.cs.cs330.examples.guithread

MAINPROGCMD=TestPrimeGenerator
MAINPROGGUI=PrimeGui
MAINPROGTHD=PrimeGuiThread

.SUFFIXES: .java .class

#SOURCES:=$(wildcard *.java)
#SOURCES+=$(wildcard utilities/*.java)
#SOURCES+=$(wildcard */*.java)
#SOURCES+=$(wildcard */*/*.java)

# Thank you stack overflow
SOURCES:=$(shell find . -name '*.java')

OBJECTS=$(SOURCES:.java=.class)
FLAGS=

all: cmd gui thd

cmd: $(OBJECTS)
	jar cvfe $(MAINPROGCMD).jar $(PKGPREFIX).driver.$(MAINPROGCMD) edu/*


gui: $(OBJECTS)
	jar cvfe $(MAINPROGGUI).jar $(PKGPREFIX).driver.$(MAINPROGGUI) edu/*

# jar: $(OBJECTS)
# 	jar cvfe $(MAINPROG).jar $(MAINPROG) *.class */*/*.class

thd: $(OBJECTS)
	jar cvfe $(MAINPROGTHD).jar $(PKGPREFIX).driver.$(MAINPROGTHD) edu/*

# $(MAINPROG): $(OBJECTS)
# 	javac $(FLAGS) $(OBJECTS) $@

.java.class: 
	javac $(FLAGS) $<

clean:
	rm $(OBJECTS) *.jar

runcmd: cmd
	java -jar $(MAINPROGCMD).jar 

rungui: gui
	java -jar $(MAINPROGGUI).jar 

runthd: thd
	java -jar $(MAINPROGTHD).jar 
