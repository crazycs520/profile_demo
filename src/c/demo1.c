#include <stdio.h>
int main() {
    for (int i = 0; i <= 100; i = i + 1){
        handle_request();
    }
    return 0;
}

int handle_request() {
    prepare();
    write_data();
    return finish();
}


const int CYCLE= 100000000;

int prepare() {
    int sum = 0;
    for (int i = 0; i <= 2*CYCLE; i = i + 1){
        sum+=i;
    }
    return 0;
}

int write_data() {
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

