CC = g++
CXX = g++
CXXFLAGS = -march=native -Wall -Winline -O3 -ffast-math -g -std=c++11 -Iinclude -Igmpfrxx -fPIC
H_CXXFLAGS = -march=native -Wall -O3 -g -std=c++11 -Iinclude -Igmpfrxx -fPIC

LDFLAGS := -L$(HOME)/lib -std=c++11
LDLIBS := -lmpfr -lgmp -lgmpxx -g
ifdef PROFILE_BUILD
    LDFLAGS := $(LDFLAGS) -Wl,--no-as-needed
    LDLIBS := $(LDLIBS) -lprofiler
endif

OBJECTS = addition.o

EXECUTABLES = addition

addition: addition.o

LIB_NAME = addition
LIB_OBJS = gmpfrxx/gmpfrxx.o

lib$(LIB_NAME).so: $(LIB_OBJS)
	$(CXX) $(LDFLAGS) $(CXXFLAGS) -shared -fPIC $(LIB_OBJS) -o $@

$(LIB_NAME).so: $(LIB_OBJS)
	$(CXX) $(LDFLAGS) $(CXXFLAGS) -shared -fPIC $(LIB_OBJS) -o $@

lib$(LIB_NAME).a: $(LIB_OBJS) 
	ar rs lib$(LIB_NAME).a $(LIB_OBJECTS)

$(LIB_NAME).a: $(LIB_OBJS) 
	$(CXX) $(LDFLAGS) $(CXXFLAGS) $(H_CXXFLAGS) -lmpfr -lgmp -lgmpxx -Igmpfrxx/ -Iinclude/ $(LIB_OBJS) -o $@


.PHONY: clean
clean:
	-rm $(OBJECTS)
	-rm $(EXECUTABLES)
