
# -------------------------
# Paths
# -------------------------
SRC_DIR := src
MODULES_DIR := modules
BUILD_DIR := build
JAVA_HOME := $(shell readlink -f /usr/bin/java | sed "s:bin/java::")

CXX := g++
CXXFLAGS := -std=c++17 -fPIC -I$(SRC_DIR) -I$(JAVA_HOME)/include -I$(JAVA_HOME)/include/linux -pthread
LDFLAGS := -shared -ldl

MODULES := $(patsubst $(MODULES_DIR)/%.cpp,$(BUILD_DIR)/%.so,$(wildcard $(MODULES_DIR)/*.cpp))
MAIN := $(BUILD_DIR)/pvp_client.so
MAIN_SRC := $(SRC_DIR)/main.cpp

# -------------------------
# Targets
# -------------------------
all: $(MAIN) $(MODULES)

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

# Build main .so
$(MAIN): $(MAIN_SRC) | $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) $(MAIN_SRC) -o $@ $(LDFLAGS)

# Build module .so files
$(BUILD_DIR)/%.so: $(MODULES_DIR)/%.cpp | $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) -shared $< -o $@

clean:
	rm -rf $(BUILD_DIR)

.PHONY: all clean
