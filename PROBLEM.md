# Hungarian Lottery

## The problem

This problem is related to the Hungarian lottery. In case you are not familiar with it: players pick **5 distinct numbers** from **1 to 90**. There is a weekly lottery picking event when the lotto organization picks **5 distinct numbers randomly between 1 and 90** – just like the players did. A player’s reward then depends on how many of the player’s numbers match with the ones selected at the lotto picking. A player wins if they have **2, 3, 4 or 5 matching numbers**.

Now, the problem: at the lottery event, right after picking the numbers, a computer shall be able to report quickly that how many winners are in each category, for example:

| Numbers matching | Winners |
|------------------|---------|
| 5                | 0       |
| 4                | 12      |
| 3                | 818     |
| 2                | 22613   |

This report shall be generated as soon as possible after picking the winning numbers. The players' numbers are known a few minutes before the show starts. In peak periods, there are currently about **10 million players**.

## Technical specification

Write a console application in a freely chosen programming language that can be compiled on Linux.

Your application will be called like this:

```bash
./yourapp input.txt
```

where `input.txt` file exists in the same folder and is an ascii file, in which each line contains **5 space separated integers** (in the range of 1-90) representing one player’s numbers.

When your application finished processing the player’s dataset from the file, it should write a line to the standard output like this:

```
READY
```

Note that it should be newline terminated. After that, the program may receive multiple lines (identical to the file’s lines) representing the lottery’s picks and it should be able to report **4 space separated numbers** in the standard output as fast as possible (line should be newline terminated).

The four numbers shall mean the number of winners with **2, 3, 4 and 5 matches** respectively.

### Example C boilerplate code

```c
#include <stdio.h>

int main() {
    int a,b,c,d,e;
    printf("READY\n");
    int i = 5;
    while(i == 5) {
        i = scanf("%d %d %d %d %d", &a, &b, &c, &d, &e);
        if(i == 5) {
            printf("%d %d %d %d\n", 0, 1, 2, 3);
        }
    }
    return 0;
}
```

## Expectations

- Write an optimized solution that can report the results in **100ms or less**
- Document the **asymptotic run time** of your solution
- Use **code comments** (enough to make it easy to understand)
- Document your ideas about how you could further **improve the calculation speed** or handle more players
- Submit your code by email or share it privately (**do not share it in public space like github**), including instructions on how to build and run
