MAINPROGCMD=TestPrimeGenerator
MAINPROGGUI=PrimeGui
MAINPROGTHD=PrimeGuiThread

.SUFFIXES: .java .class

SOURCES:=$(wildcard *.java)
#SOURCES+=$(wildcard utilities/*.java)
SOURCES+=$(wildcard */*.java)
SOURCES+=$(wildcard */*/*.java)
OBJECTS=$(SOURCES:.java=.class)
FLAGS=

all: cmd gui thd

cmd: $(OBJECTS)
	jar cvfe $(MAINPROGCMD).jar Driver.$(MAINPROGCMD) Driver/$(MAINPROGCMD).class Generator/*/*.class

gui: $(OBJECTS)
	jar cvfe $(MAINPROGGUI).jar Driver.$(MAINPROGGUI) Driver/$(MAINPROGGUI)*.class Generator/*/*.class

# jar: $(OBJECTS)
# 	jar cvfe $(MAINPROG).jar $(MAINPROG) *.class */*/*.class

thd: $(OBJECTS)
	jar cvfe $(MAINPROGTHD).jar Driver.$(MAINPROGTHD) Driver/$(MAINPROGTHD)*.class Generator/*/*.class

# $(MAINPROG): $(OBJECTS)
# 	javac $(FLAGS) $(OBJECTS) $@

.java.class: 
	javac $(FLAGS) $<

clean:
	rm *.class */*/*.class */*.class *.jar

runcmd: cmd
	java -jar $(MAINPROGCMD).jar 

rungui: gui
	java -jar $(MAINPROGGUI).jar 

runthd: thd
	java -jar $(MAINPROGTHD).jar 
