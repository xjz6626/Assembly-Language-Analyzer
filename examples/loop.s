// 循环计数示例 (计算 1+2+...+10)
mov x0, #0         // sum = 0
mov x1, #1         // i = 1
mov x2, #10        // limit = 10

loop:
    add x0, x0, x1 // sum += i
    add x1, x1, #1 // i++
    cmp x1, x2     // compare i with limit
    b.le loop      // if i <= limit, continue loop

// x0 现在包含结果 (55)
