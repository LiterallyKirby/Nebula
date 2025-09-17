
# Makefile for NebulaC JNI project

# Compiler
CC = gcc

# JDK Include paths (adjust to your JDK)
JAVA_HOME = /usr/lib/jvm/java-8-openjdk
JNI_INC = -I$(JAVA_HOME)/include -I$(JAVA_HOME)/include/linux

# Source files
SRC = src/jni_wrapper.c src/main.c 
OBJ = $(SRC:.c=.o)

# Output shared library
TARGET = libNebulaJNI.so

# Compiler flags

# Add this line in your Makefile with other CFLAGS
CFLAGS = -Wall -fPIC -O2 $(JNI_INC) -Iinclude

LDFLAGS = -shared -lpthread

.PHONY: all clean

all: $(TARGET)

$(TARGET): $(OBJ)
	$(CC) $(LDFLAGS) -o $@ $^

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(OBJ) $(TARGET)
