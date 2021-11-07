#include <stdio.h>

void handle_sql_request(int load);

int main() {
    while (1)
    {
        // mock for different kinds of sql request.
        for (int i = 0; i <= 100; i = i + 1){
            handle_sql_request(i);
        }
    }
    return 0;
}

void handle_sql_request(int load) {
    const int CYCLE = 1000000;
    int sum = 0;
    for (int i = 0; i <= load * CYCLE; i++){
        sum+=i;
    }
}
