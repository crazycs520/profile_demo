#include <stdio.h>

void handle_request();
void prepare();
void process_data();
void finish();

void main() {
    for (int i = 0; i <= 100000000000; i = i + 1){
        handle_request();
    }
}

void handle_request() {
    prepare();
    process_data();
    finish();
}


const int CYCLE= 1000000;

void prepare() {
    int sum = 0;
    for (int i = 0; i <= 2*CYCLE; i = i + 1){
        sum+=i;
    }
}

void process_data() {
    int sum = 0;
    for (int i = 0; i <= 7*CYCLE; i = i + 1){
        sum+=i;
    }
}

void finish() {
    int sum = 0;
    for (int i = 0; i <= 1*CYCLE; i = i + 1){
        sum+=i;
    }
}

