#include <stdio.h>

void handle_sql_request(int load);

int main() {
    while (1)
    {
        // mock for different kinds of sql request.
        for (int id = 1; id <= 4; id = id + 1){
            handle_sql_request(id);
        }
    }
    return 0;
}

void handle_sql_request(int load) {
    const int CYCLE = 1000000;
    int sum = 0;
    for (int i = 0; i <= load * CYCLE; i = i + 1){
        sum = sum + i;
    }
}
