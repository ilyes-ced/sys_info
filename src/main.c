#include <stdio.h>
#include <sys/syscall.h>



int main(int argc, char const *argv[]) {
    long long test_long = 0xb00;
    int my_dude = 0x123456789;
    char string[] = "hello thereé";


    int gg = sys_function();

    return teeeeeeeeeeeeeeest(0xaaaaaaaaaaaaaaaaa, 0xbbbbbbbbbbbbbbbb);
}


int teeeeeeeeeeeeeeest(int vv, int bb){
    while (vv){
        vv--;
    }
    int ggggggg = vv+bb;
    return vv*5;
}
int sys_function(){
    syscall(SYS_write, 1, "done\n", 1);
    return 0;
}