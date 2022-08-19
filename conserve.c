// "conserve", a program to query and set Battery Conservation Mode on
// supported Lenovo laptops.
// Created by LRitzdorf

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

// NOTE: Set your "battery conservation mode" file here:
#define TARGET "/sys/bus/platform/drivers/ideapad_acpi/VPC2004:00/conservation_mode"

// Initial setup
enum Action {
	Query,
	Enable,
	Disable,
	Toggle
};
char* queries[]  = {"?", "st", "stat", "status", "q", "query"};
char* enables[]  = {"1", "on", "true", "y", "yes", "en", "enable"};
char* disables[] = {"0", "off", "false", "n", "no", "dis", "disable"};
char* toggles[]  = {"-1", "t", "tog", "toggle", "sw", "switch"};


// Helper functions

void get_help(char* name) {
	printf(
			"Usage:\n"
			"	%s [action]\n"
			"Interact with battery conservation mode on Lenovo laptops.\n"
			"\n"
			"Actions:\n"
			"	Query:    ?,             st[at[us]], q[uery]   DEFAULT\n"
			"	Enable:   1, on,  true,  n[o],       en[able]\n"
			"	Disable:  0, off, false, y[es],      dis[able]\n"
			"	Toggle:  -1,             t[og[gle]], sw[itch]\n",
			name);
}

bool string_in(char* target, char** options, int numopts) {
	for (int i = 0; i < numopts; i++) {
		if (strcmp(options[i], target) == 0) {
			return true;
		}
	}
	return false;
}

bool get_mode(const char* filename) {
	FILE* fin = fopen(filename, "r");
	bool result = fgetc(fin) == '1';
	fclose(fin);
	return result;
}

bool set_mode(const char* filename, const bool mode) {
	FILE* fout = fopen(filename, "w");
	if (fout == NULL) {
		fprintf(stderr, "Failed to open \"%s\" for writing.\nAre you root?\n", filename);
		return false;
	}
	bool success = fputc(mode ? '1' : '0', fout);
	fclose(fout);
	return success;
}


// Main function

int main(int argc, char** argv) {
	// Process user input
	enum Action action;
	if (argc == 1) {
		action = Query;
	} else {
		if (string_in(argv[1], queries, sizeof(queries)/sizeof(queries[0]))) {
			action = Query;
		} else if (string_in(argv[1], enables, sizeof(enables)/sizeof(enables[0]))) {
			action = Enable;
		} else if (string_in(argv[1], disables, sizeof(disables)/sizeof(disables[0]))) {
			action = Disable;
		} else if (string_in(argv[1], toggles, sizeof(toggles)/sizeof(toggles[0]))) {
			action = Toggle;
		} else {
			get_help(argv[0]);
			exit(1);
		}
	}

	bool mode;
	switch (action) {
		case Query:
			printf("%d\n", get_mode(TARGET));
			exit(0);
		case Enable:
			mode = true;
			break;
		case Disable:
			mode = false;
			break;
		case Toggle:
			mode = !get_mode(TARGET);
			break;
	}

	// Apply desired mode
	printf("%d\n", mode);
	exit(!set_mode(TARGET, mode));
}
