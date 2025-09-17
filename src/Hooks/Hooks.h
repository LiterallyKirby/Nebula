#include <jni.h>
#include <memory>
class Hooks {
public:
  Hooks() { 
		

		 is_init = true; 
	}
  ~Hooks() { 
	

		is_init = false; 
	}

	bool GetInit() {
		return is_init;
	}

private:
  bool is_init{false};
};

inline std::unique_ptr<Hooks> p_Hooks;
