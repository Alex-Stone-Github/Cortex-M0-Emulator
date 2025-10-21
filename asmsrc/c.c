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
