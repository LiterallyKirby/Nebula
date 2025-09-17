# Compiler
CC = g++

# Compiler flags


CFLAGS = -Wall -fPIC -O2 \
    -I/usr/lib/jvm/java-8-openjdk/include \
    -I/usr/lib/jvm/java-8-openjdk/include/linux \
    -Iinclude \
    -Ideps \
    -Imapping \
    -IModules \
    -Iutils \
    -Isrc/Utils \
    -Isrc/mapping/impl \
    -Isrc \
    $(shell pkg-config --cflags gtk+-3.0)



# Linker flags
LDFLAGS = -shared
LIBS = $(shell pkg-config --libs gtk+-3.0) -lglfw

# Optional: add extra libraries here
 #EXTRA_LIBS = deps/libMinHook-x86-v141-mt.a   # example static library

# Sources and objects
SRC = $(wildcard src/*.cpp)
OBJ = $(SRC:.cpp=.o)
TARGET = libNebulaJNI.so

.PHONY: all clean

all: $(TARGET)

$(TARGET): $(OBJ)
	$(CC) $(CFLAGS) $(OBJ) -o $@ $(LDFLAGS) $(LIBS) $(EXTRA_LIBS)

%.o: %.cpp
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(OBJ) $(TARGET)
