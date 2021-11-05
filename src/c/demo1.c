#include <stdio.h>
int main() {
    for (int i = 0; i <= 100000000000; i = i + 1){
        handle_request();
    }
    return 0;
}

int handle_request() {
    prepare();
    process_data();
    finish();
    return 0;
}


const int CYCLE= 1000000;

int prepare() {
    int sum = 0;
    for (int i = 0; i <= 2*CYCLE; i = i + 1){
        sum+=i;
    }
    return 0;
}

int process_data() {
    int sum = 0;
    for (int i = 0; i <= 7*CYCLE; i = i + 1){
        sum+=i;
    }
    return 0;
}

int finish() {
    int sum = 0;
    for (int i = 0; i <= 1*CYCLE; i = i + 1){
        sum+=i;
    }
    return 0;
}

