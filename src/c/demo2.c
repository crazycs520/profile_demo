#include <stdio.h>
int main() {
    for (int i = 0; i <= 100; i = i + 1){
        query(i);
    }
    return 0;
}

int query(int load){
    read_data(load);
    return 0;
}

int read_data(int n) {
    n = n* 100000000;
    int sum = 0;
    for (int i = 0; i <= n; i = i + 1){
        sum+=i;
    }
    return sum;
}
