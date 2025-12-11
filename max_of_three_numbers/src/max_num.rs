pub fn cmp_num(a: i32, b: i32, c: i32) ->i32 {

    if a > b && a > c {
       return a;
    } else if b > a && b > c {
       return b;
    } else if c > a && c > b {
       return c;
    }else if a == b && a > c && b > c {
        return a;
    } else if a == c && a > b && c > b {
        return c;
    } else if b == c && c > a && b > a {
        return b;
    } else {
        return 0;
    }
}
