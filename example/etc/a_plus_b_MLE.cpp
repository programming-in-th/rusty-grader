#include <bits/stdc++.h>

#define long long long

using namespace std;

long a, b;
long dp[10000000];

int main() {
    scanf("%lld %lld", &a, &b);
    printf("%lld\n", a + b);
    for(int i = 0; i < 10000000; i++)
        dp[i] = 1;

    return 0;
}
