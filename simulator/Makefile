CC = g++ -std=gnu++17 -O2
CFLAGS = -Wall -Wextra

SRCS = print.cpp cpu.cpp instruction.cpp main.cpp
OBJS = $(SRCS:%.cpp=%.o)
DEPS = $(SRCS:%.cpp=%.d)

simulator: $(OBJS)
	$(CC) $(CFLAGS) -o $@ $+

%.o: %.cpp
	$(CC) $(CFLAGS) -c -MMD -MP -o $@ $<

-include $(DEPS)

.PHONY: clean
clean:
	rm -f $(OBJS) $(DEPS) simulator
