#include <iostream>

using namespace std;

void foo(int depth) {
    cout << "current depth: " << depth << endl;
    if (depth == 5) {
        throw 1;
    }
    else {
        cout << ends;
        foo(depth + 1);
    }
}

int main()
{
    try {
        foo(0);
    }
    catch (int e) {
        cout << "caught exception: " << e << endl;
    }

    return 0;
}
