[golden.b -- compute golden ratio
(c) 2019 Daniel B. Cristofani
http://brainfuck.org/]

+>>>>>>>++>+>+>+>++<[
    +[
        --[++>>--]->--[
            +[
                +<+[-<<+]++<<[-[->-[>>-]++<[<<]++<<-]+<<]>>>>-<<<<
                <++<-<<++++++[<++++++++>-]<.---<[->.[-]+++++>]>[[-]>>]
            ]+>>--
        ]+<+[-<+<+]++>>
    ]<<<<[[<<]>>[-[+++<<-]+>>-]++[<<]<<<<<+>]
    >[->>[[>>>[>>]+[-[->>+>>>>-[-[+++<<[-]]+>>-]++[<<]]+<<]<-]<]]>>>>>>>
]

This program computes the "golden ratio" (https://oeis.org/A001622). Because
this number is infinitely long, this program doesn't terminate on its own;
you will have to kill it.