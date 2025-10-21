extern volatile char my_four;

char* garbage = "HIC";

void centry() {
	my_four = 5;
	for (int i = 0; i < 5; i ++) {
		my_four += 2;
	}
	return;
}
