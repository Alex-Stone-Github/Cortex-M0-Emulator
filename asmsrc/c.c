extern volatile char my_four;

int square(int x);

int number = 5;
char* garbage = "HIC";

void centry() {
	my_four = 5;
	for (int i = 0; i < number; i ++) {
		my_four += 2;
	}
	number += square(4);
	my_four += square(4);
	return;
}

int square(int x) {
	// volatileness
	my_four ++;
	my_four --;
	//
	return x * x;
}

/// Cuts off after null termination or len expiration
void printserial(char const* msg, int len) {
	// Start of the special functional region(coudl call lua code)
	volatile char* adr = (char*)500;
	int i = 0;
	while (msg[i] != 0 && len > 0) {
		*adr = msg[i];
		++i;
		--len; 
	}
}

void theend() {
	char const* message = "Alex was here!!\n";
	printserial(message, 500);
	printserial(message, 4);
	printserial(message, 500);
}
